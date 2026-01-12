// MSS: Maximum Segment Size - trasnport layer (TCP) maximum payload size
// TCP advertises MSS during the SYN handshake

use std::{
    collections::BTreeMap,
    net::{IpAddr, Ipv4Addr},
};

use crate::config::TargetServer;

const TCP_HEADER_SIZE: usize = 20;

// TCP segment size
// TCP uses 16-bit length field, so 2^16 - 1 = 65535 bytes maximum
// It includes IP header, TCP header, and TCP payload, so 65535 - 20 (IP header) - 20 (TCP header) = 65515 bytes for TCP payload

// BUT, actual maximum segment size (MSS) is often lower due to network constraints like MTU
// Common MSS values are 1460 bytes for Ethernet (1500 MTU - 20 IP header - 20 TCP header)

// IMPORTANT:
// \r\n\r\n indicates the end of HTTP headers

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

pub struct TcpParserParams {
    pub parse_payload: bool,
    pub src_ip: Ipv4Addr,
    pub target_server: Option<TargetServer>,
}

pub fn parse_tcp(_data: &[u8], params: TcpParserParams) {
    if _data.len() < TCP_HEADER_SIZE {
        println!("Data too short to contain a valid TCP header.");
        return;
    }

    let scr_port = u16::from_be_bytes([_data[0], _data[1]]);
    if let Some(target) = &params.target_server {
        let target_ip: Ipv4Addr = target.ip.parse().expect("Invalid target IP address");
        if scr_port != target.port && params.src_ip != target_ip {
            return;
        }
    }

    // let dst_port = u16::from_be_bytes([_data[2], _data[3]]);

    // We look for 4 bits in the 13th byte (index 12) and then multiply by 4
    // because the data offset is given in 32-bit words
    let data_offset = (_data[12] >> 4) * 4; // in bytes

    // Ensure the data length is sufficient for the TCP header with options
    if _data.len() < data_offset as usize {
        println!("Data too short for TCP header with options.");
        return;
    }

    if !params.parse_payload {
        return;
    }

    let payload = &_data[data_offset as usize..];
    if payload.is_empty() {
        return;
    }
    if is_tcp_secure(payload) {
        println!("Secure TCP payload detected (TLS/SSL), skipping HTTP parsing.");
        return;
    }

    if let Ok(payload_str) = std::str::from_utf8(payload) {
        println!("HTTP Payload:\n{}", payload_str);
    } else {
        println!("HTTP Payload contains non-UTF8 data.");
    }
}

fn is_tcp_secure(payload: &[u8]) -> bool {
    // Check for TLS ClientHello (0x16 0x03 0x01 or 0x16 0x03 0x03)
    if payload.len() < 3 {
        return false;
    }

    payload[0] == 0x16 && (payload[1] == 0x03 && (payload[2] == 0x01 || payload[2] == 0x03))
}
