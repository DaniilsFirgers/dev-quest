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

    let src_port = u16::from_be_bytes([_data[0], _data[1]]);
    let dst_port = u16::from_be_bytes([_data[2], _data[3]]);

    if let Some(target) = &params.target_server {
        let target_ip: Ipv4Addr = target.ip.parse().expect("Invalid target IP address");
        // NOTE: we are only interested in the pakcets from server to client
        if params.src_ip != target_ip || src_port != target.port {
            return;
        }
    }

    let from_client = match &params.target_server {
        Some(server) => {
            let server_ip: Ipv4Addr = server.ip.parse().expect("Invalid target IP address");
            params.dst_ip == server_ip && dst_port == server.port
        }
        // TODO: replace by SYN-based detection
        // SYN without ACK - client to server
        // SYN with ACK - server to client
        None => true,
    };

    let conn_key = build_consistent_conn_key(
        params.src_ip,
        params.dst_ip,
        src_port,
        dst_port,
        from_client,
    );

    let seq_num = u32::from_be_bytes([_data[4], _data[5], _data[6], _data[7]]);

    let conn_state = reassembly_table
        .connections
        .entry(conn_key.clone())
        .or_insert_with(|| {
            let initial_seq_c2s = if from_client { seq_num + 1 } else { 0 };
            let initial_seq_s2c = if from_client { 0 } else { seq_num + 1 };
            ConnectionState::new(initial_seq_c2s, initial_seq_s2c)
        });

    // We look for 4 bits in the 13th byte (index 12) and then multiply by 4
    // because the data offset is given in 32-bit words
    let data_offset = (_data[12] >> 4) * 4; // in bytes

    // Ensure the data length is sufficient for the TCP header with options
    if _data.len() < data_offset as usize {
        println!("Data too short for TCP header with options.");
        return;
    }

    let payload = &_data[data_offset as usize..];
    if !payload.is_empty() {
        conn_state.add_segment(seq_num, payload, from_client);
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

    println!("Assembled TCP data length: {}", assembled_data.len());
    // TODO: take into accunt retransmissions and out-of-order segments
    if !payload.is_empty() {
        if let Ok(payload_str) = std::str::from_utf8(payload) {
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
    from_client: bool,
) -> ConnectionKey {
    if from_client {
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
