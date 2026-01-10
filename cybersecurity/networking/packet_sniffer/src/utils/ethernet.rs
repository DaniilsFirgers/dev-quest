// Ethernet is a link-layer protocol used for local area networking (LAN)
// Delivers a frame from one network interface to another on the same physical network
// ARP request is broadcasted to all devices on the local network
// Only understands MAC addresses (hardware addresses)!

// +-----------------+-----------------+-------------------+
// | Destination MAC | Source MAC      | EtherType         |  <-- Ethernet header (14 bytes)
// | 6 bytes         | 6 bytes         | 2 bytes           |
// +-----------------+-----------------+-------------------+
// | Payload (variable length, e.g., ARP or IPv4 packet)       |
// +----------------------------------------------------------+

use crate::config::Config;
use crate::utils::ipv4::parse_ipv4;

pub fn parse_ethernet(data: &[u8], config: &Config) {
    // A raw packet starts with an Ethernet header
    // Ethernet header is 14 bytes long

    // 1. Destination MAC (6 bytes)
    // 2. Source MAC (6 bytes)
    // 3. EtherType (2 bytes)

    if data.len() < HEADER_SIZE {
        println!("Packet too short for Ethernet header");
        return;
    }

    // let dst_mac = &data[0..6];
    // let src_mac = &data[6..12];
    let ether_type = &data[12..14];

    let ether_type_bytes = u16::from_be_bytes([ether_type[0], ether_type[1]]);

    if let Some(eth_type) = EtherType::from_u16(ether_type_bytes) {
        match eth_type {
            EtherType::IPv4 => {
                if config.ipv4.log {
                    parse_ipv4(&data[HEADER_SIZE..], config)
                }
            }
            EtherType::ARP => {
                if config.arp.log {
                    parse_arp(&data[HEADER_SIZE..], config)
                }
            }
            EtherType::IPv6 => {}
        }
    } else {
        println!("EtherType: Unknown");
    }
}

#[derive(Debug)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    // (Address resolution protocol) Link layer protocol to discover device on a local network for IPv4
    ARP = 0x0806,
    IPv6 = 0x86DD,
}

pub const HEADER_SIZE: usize = 14;

impl EtherType {
    pub fn from_u16(value: u16) -> Option<EtherType> {
        match value {
            0x0800 => Some(EtherType::IPv4),
            0x0806 => Some(EtherType::ARP),
            0x86DD => Some(EtherType::IPv6),
            _ => None,
        }
    }
}

// ARP headers:
// 1. Hardware Type (2 bytes) - Ethernet is 1
// 2. Protocol Type (2 bytes) - IPv4 is 0x0800
// 3. Hardware Size (1 byte) - MAC address size (6 bytes)
// 4. Protocol Size (1 byte) - IPv4 address size (4 bytes)
// 5. Opcode (2 bytes) - request (1) or reply (2)
// 6. Sender MAC Address (6 bytes)
// 7. Sender IP Address (4 bytes)
// 8. Target MAC Address (6 bytes)
// - for a request, this is all zeros (00:00:00:00:00:00)
// 9. Target IP Address (4 bytes)

// ff:ff:ff:ff:ff:ff is the broadcast MAC address (used in ARP requests)
// while 00:00:00:00:00:00 is a null MAC address (used in ARP requests for target MAC)

const ARP_HEADER_SIZE: usize = 28;
fn parse_arp(data: &[u8], config: &Config) {
    if data.len() < ARP_HEADER_SIZE {
        println!("Packet too short for ARP header");
        return;
    }

    let opcode = u16::from_be_bytes([data[6], data[7]]);
    match opcode {
        1 => {
            // 1 is request
            if config.arp.kind == "reply" {
                return;
            }
        }
        2 => {
            // 2 is reply
            if config.arp.kind == "request" {
                return;
            }
        }
        _ => {
            // TODO: log here
            return;
        }
    }

    let hardware_type = u16::from_be_bytes([data[0], data[1]]);
    let protocol_type = u16::from_be_bytes([data[2], data[3]]);
    let hardware_size = data[4];
    let protocol_size = data[5];

    let sender_mac = &data[8..14];
    let sender_ip = &data[14..18];

    let target_mac = &data[18..24];
    let target_ip = &data[24..28];

    println!(
        "ARP Header: \n\tHardware Type: {}\n\tProtocol Type: {:04x}\n\tHardware Size: {}\n\tProtocol Size: {}\n\tOpcode: {}\n\tSender MAC: {:02x?}\n\tSender IP: {}.{}.{}.{}\n\tTarget MAC: {:02x?}\n\tTarget IP: {}.{}.{}.{}",
        hardware_type,
        protocol_type,
        hardware_size,
        protocol_size,
        opcode,
        sender_mac,
        sender_ip[0], sender_ip[1], sender_ip[2], sender_ip[3],
        target_mac,
        target_ip[0], target_ip[1], target_ip[2], target_ip[3],
    );
}
