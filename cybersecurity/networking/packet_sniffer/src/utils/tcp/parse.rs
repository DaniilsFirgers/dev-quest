// MSS: Maximum Segment Size - trasnport layer (TCP) maximum payload size
// TCP advertises MSS during the SYN handshake

use std::net::{IpAddr, Ipv4Addr};

use crate::{
    config::TargetServer,
    utils::tcp::state::{ConnectionKey, ConnectionState, TcpReassemblyTable},
};

const TCP_HEADER_SIZE: usize = 20;

pub struct TcpParserParams {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub target_server: Option<TargetServer>,
}

pub fn parse_tcp(_data: &[u8], params: TcpParserParams, reassembly_table: &mut TcpReassemblyTable) {
    if _data.len() < TCP_HEADER_SIZE {
        println!("Data too short to contain a valid TCP header.");
        return;
    }

    // We look for 4 bits in the 13th byte (index 12) and then multiply by 4
    // because the data offset is given in 32-bit words
    let data_offset = (_data[12] >> 4) * 4; // in bytes

    // Ensure the data length is sufficient for the TCP header with options
    if _data.len() < data_offset as usize {
        println!("Data too short for TCP header with options.");
        return;
    }

    let src_port = u16::from_be_bytes([_data[0], _data[1]]);
    let dst_port = u16::from_be_bytes([_data[2], _data[3]]);

    let flags = _data[13];
    // TODO: cover bit masks
    let syn = flags & 0x02 != 0;
    let ack = flags & 0x10 != 0;
    let fin = flags & 0x01 != 0;
    let rst = flags & 0x04 != 0;

    let payload = &_data[data_offset as usize..];

    let conn_key = build_consistent_conn_key(params.src_ip, params.dst_ip, src_port, dst_port);

    let target_server = params.target_server.as_ref().map(|server| {
        let target_ip: Ipv4Addr = server.ip.parse().expect("Invalid target IP address");
        (target_ip, server.port)
    });

    if let Some((server_ip, server_port)) = target_server {
        let involves_server = (params.src_ip == server_ip && src_port == server_port)
            || (params.dst_ip == server_ip && dst_port == server_port);

        if !involves_server {
            return;
        }
    }

    let from_client = if let Some((server_ip, server_port)) = target_server {
        // Client → Server
        params.dst_ip == server_ip && dst_port == server_port
    } else if syn && !ack {
        // SYN without ACK is likely from client to server
        true
    } else if syn && ack {
        // SYN-ACK is from server to client
        false
    } else {
        // Established connection, determine direction based on port numbers (heuristic)
        true
    };

    let seq_num = u32::from_be_bytes([_data[4], _data[5], _data[6], _data[7]]);

    let conn_state = reassembly_table
        .connections
        .entry(conn_key.clone())
        .or_insert_with(|| ConnectionState::new());

    // --- SYN handling (sequence init) ---
    if syn {
        let next_seq = if from_client {
            &mut conn_state.next_seq_c2s
        } else {
            &mut conn_state.next_seq_s2c
        };

        if next_seq.is_none() {
            *next_seq = Some(seq_num + 1); // SYN consumes 1
        }
    }

    // -- Payload handling ---
    if !payload.is_empty() {
        conn_state.add_segment(seq_num, payload, from_client);
    }

    if fin || rst {
        reassembly_table.connections.remove(&conn_key);
        return;
    }

    let assembled_data = if from_client {
        &conn_state.assembled_c2s
    } else {
        &conn_state.assembled_s2c
    };

    if is_tcp_secure(assembled_data) {
        println!("Secure TCP payload detected (TLS/SSL), skipping HTTP parsing.");
        return;
    }

    // TODO: take into accunt retransmissions and out-of-order segments
    if !assembled_data.is_empty() {
        if let Ok(payload_str) = std::str::from_utf8(assembled_data) {
            println!("Reassembled TCP Payload:\n{}", payload_str);
        }
    }
}

fn is_tcp_secure(payload: &[u8]) -> bool {
    // Check for TLS ClientHello (0x16 0x03 0x01 or 0x16 0x03 0x03)
    if payload.len() < 3 {
        return false;
    }

    payload[0] == 0x16 && (payload[1] == 0x03 && (payload[2] == 0x01 || payload[2] == 0x03))
}

// Build a consistent connection key regardless of packet direction
fn build_consistent_conn_key(
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    src_port: u16,
    dst_port: u16,
) -> ConnectionKey {
    // Lexicographically order endpoints to ensure same key both directions
    if (src_ip, src_port) < (dst_ip, dst_port) {
        ConnectionKey {
            src_ip: IpAddr::V4(src_ip),
            dst_ip: IpAddr::V4(dst_ip),
            src_port,
            dst_port,
        }
    } else {
        ConnectionKey {
            src_ip: IpAddr::V4(dst_ip),
            dst_ip: IpAddr::V4(src_ip),
            src_port: dst_port,
            dst_port: src_port,
        }
    }
}
