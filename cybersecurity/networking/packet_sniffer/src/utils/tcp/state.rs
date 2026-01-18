// TCP segment size
// TCP uses 16-bit length field, so 2^16 - 1 = 65535 bytes maximum
// It includes IP header, TCP header, and TCP payload, so 65535 - 20 (IP header) - 20 (TCP header) = 65515 bytes for TCP payload

// BUT, actual maximum segment size (MSS) is often lower due to network constraints like MTU
// Common MSS values are 1460 bytes for Ethernet (1500 MTU - 20 IP header - 20 TCP header)

// IMPORTANT:
// \r\n\r\n indicates the end of HTTP headers

use std::{
    collections::{BTreeMap, HashMap},
    net::IpAddr,
};

// The key ALWAYS represents client → server
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConnectionKey {
    pub src_ip: IpAddr, // Client IP
    pub dst_ip: IpAddr, // Server IP
    pub src_port: u16,  // Ephemeral port
    pub dst_port: u16,  // Well-known port (e.g., 80 for HTTP)
}

pub struct ConnectionState {
    // Client to Server
    pub next_seq_c2s: u32,                  // Sequence number + payload length
    pub buffer_c2s: BTreeMap<u32, Vec<u8>>, // Buffer for out-of-order segments
    pub assembled_c2s: Vec<u8>,             // Assembled data

    // Server to Client
    pub next_seq_s2c: u32,
    pub buffer_s2c: BTreeMap<u32, Vec<u8>>,
    pub assembled_s2c: Vec<u8>,
}

pub struct TcpReassemblyTable {
    pub connections: HashMap<ConnectionKey, ConnectionState>,
}

impl ConnectionState {
    pub fn new(initial_seq_c2s: u32, initial_seq_s2c: u32) -> Self {
        return ConnectionState {
            next_seq_c2s: initial_seq_c2s,
            buffer_c2s: BTreeMap::new(),
            assembled_c2s: Vec::new(),
            next_seq_s2c: initial_seq_s2c,
            buffer_s2c: BTreeMap::new(),
            assembled_s2c: Vec::new(),
        };
    }

    pub fn add_segment(&mut self, seq_num: u32, payload: &[u8], from_client: bool) {
        if payload.is_empty() {
            return;
        }

        let (next_seq, buffer, assembled) = if from_client {
            (
                &mut self.next_seq_c2s,
                &mut self.buffer_c2s,
                &mut self.assembled_c2s,
            )
        } else {
            (
                &mut self.next_seq_s2c,
                &mut self.buffer_s2c,
                &mut self.assembled_s2c,
            )
        };

        // NOTE: if data arrives in order
        if *next_seq == seq_num {
            // Append current payload to assembled data
            assembled.extend_from_slice(payload);

            // Update next expected sequence number
            *next_seq += payload.len() as u32;

            // Read and append any buffered segments that can now be assembled
            // (i.e., segments with sequence numbers equal to next_seq)
            // Segments are ordered in BTreeMap, so we can iterate in order
            while let Some(segment) = buffer.remove(next_seq) {
                assembled.extend_from_slice(&segment);
                *next_seq += segment.len() as u32;
            }
        }
        // NOTE: if data arrives out of order (future segment)
        else if seq_num > *next_seq {
            // Insert if does not exist, and avoid overwriting existing segments
            buffer.entry(seq_num).or_insert_with(|| payload.to_vec());
        }
        // NOTE: if data is a retransmission (past segment)
        else {
            println!("Retransmitted segment detected, ignoring.");
        }
    }
}
