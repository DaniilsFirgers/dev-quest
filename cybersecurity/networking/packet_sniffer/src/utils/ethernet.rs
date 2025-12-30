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

use super::tcp::parse_tcp;

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
pub fn parse_arp(data: &[u8]) {
    if data.len() < ARP_HEADER_SIZE {
        println!("Packet too short for ARP header");
        return;
    }

    let hardware_type = u16::from_be_bytes([data[0], data[1]]);
    let protocol_type = u16::from_be_bytes([data[2], data[3]]);
    let hardware_size = data[4];
    let protocol_size = data[5];
    let opcode = u16::from_be_bytes([data[6], data[7]]);

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

pub fn parse_ipv4(_data: &[u8]) {
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

    println!(
        "IPv4 Header: \n\tVersion: {}\n\tIHL: {} ({} bytes)\n\tProtocol: {:?}\n\tSource IP: {}.{}.{}.{}\n\tDestination IP: {}.{}.{}.{}",
        version,
        ihl,
        header_length,
        IPProtocol::from_u8(protocol),
        src_ip[0], src_ip[1], src_ip[2], src_ip[3],
        dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3],
    );

    match IPProtocol::from_u8(protocol) {
        Some(IPProtocol::TCP) => {
            let tcp_data = &_data[header_length..];
            parse_tcp(tcp_data);
        }
        _ => {}
    }
}
