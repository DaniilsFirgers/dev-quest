use std::net::Ipv4Addr;

use crate::config::Config;
use crate::utils::tcp::parse::{parse_tcp, TcpParserParams};
use crate::utils::tcp::state::TcpReassemblyTable;
// +----------------+----------------+----------------+----------------+
// | Version + IHL  | Type of Service| Total Length                     |
// +----------------+----------------+----------------+----------------+
// | Identification | Flags + Fragment Offset                     |
// +----------------+----------------+----------------+----------------+
// | TTL            | Protocol       | Header Checksum                 |
// +----------------+----------------+----------------+----------------+
// | Source IP Address (4 bytes)                                   |
// +----------------+----------------+----------------+----------------+
// | Destination IP Address (4 bytes)                              |
// +----------------+----------------+----------------+----------------+
// | Options (if IHL > 5) ...                                     |
// +----------------+----------------+----------------+----------------+
// | Payload (TCP/UDP/ICMP etc.)                                   |
// +----------------------------------------------------------------+

const IPV4_HEADER_MIN_SIZE: usize = 20;
#[derive(Debug)]
enum IPProtocol {
    ICMP = 1,
    TCP = 6,
    UDP = 17,
}

impl IPProtocol {
    pub fn from_u8(value: u8) -> Option<IPProtocol> {
        match value {
            1 => Some(IPProtocol::ICMP),
            6 => Some(IPProtocol::TCP),
            17 => Some(IPProtocol::UDP),
            _ => None,
        }
    }
}

pub fn parse_ipv4(_data: &[u8], config: &Config, reassembly_table: &mut TcpReassemblyTable) {
    if _data.len() < IPV4_HEADER_MIN_SIZE {
        println!("Packet too short for IPv4 header");
        return;
    }

    let version_ihl = _data[0];
    // By the means of bitwise operations, we can extract the version and IHL
    // It works as follows:
    // The version is stored in the higher 4 bits of the first byte
    // The IHL is stored in the lower 4 bits of the first byte
    let version = version_ihl >> 4;
    let ihl = version_ihl & 0x0F;
    let header_length = (ihl * 4) as usize; // IHL is in 32-bit words, so multiply by 4 to get bytes

    let protocol = _data[9];
    let src_ip = &_data[12..16];
    let dst_ip = &_data[16..20];

    if config.ipv4.include_headers {
        println!(
        "IPv4 Header: \n\tVersion: {}\n\tIHL: {} ({} bytes)\n\tProtocol: {:?}\n\tSource IP: {}.{}.{}.{}\n\tDestination IP: {}.{}.{}.{}",
        version,
        ihl,
        header_length,
        IPProtocol::from_u8(protocol),
        src_ip[0], src_ip[1], src_ip[2], src_ip[3],
        dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3],
    );
    }

    match IPProtocol::from_u8(protocol) {
        Some(IPProtocol::TCP) => {
            if !config.protocols.tcp.log {
                return;
            }
            let tcp_data = &_data[header_length..];
            parse_tcp(
                tcp_data,
                TcpParserParams {
                    parse_payload: config.protocols.tcp.parse_payload,
                    src_ip: from_bytes_to_ipv4_addr(src_ip),
                    dst_ip: from_bytes_to_ipv4_addr(dst_ip),
                    target_server: config.target_server.clone(),
                },
                reassembly_table,
            );
        }
        _ => {}
    }
}

fn from_bytes_to_ipv4_addr(bytes: &[u8]) -> Ipv4Addr {
    Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])
}
