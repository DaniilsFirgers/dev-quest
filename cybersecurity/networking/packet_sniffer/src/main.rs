use libc::*;
use std::fmt;
use std::mem;
use std::ptr;

mod utils;
use crate::utils::ethernet::EtherType;

fn main() {
    // Using unsafe block to call low-level C functions

    unsafe {
        // Create a raw socket with the following parameters:
        // AF_PACKET: address family for low-level packet interface (gives access to 2 OSI layer)
        // SOCK_RAW: Raw socket type, I get [Ethernet, IP, TCP/UDP] headers
        // ETH_P_ALL: all ethernet protocols

        // Here we use htons to convert the protocol number to network byte order
        // Endianness matters in networking - which byte comes first in multi-byte values

        // Big-endian: most significant byte first
        // Little-endian: least significant byte first
        let sock = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_ALL as u16) as i32);

        // On Error, socket returns -1, we do not handle errors gracefully here for brevity
        if sock < 0 {
            panic!("Failed to create socket");
        }

        // NOTE: initialize a buffer to hold incoming packets
        // Size is of 16 bits (2 bytes) so max value is 65535
        // Make a buffer of 65536 (0 to 65535 as 2^16) bytes (max size for Ethernet frame)

        // This is the entire packet including headers
        // MTU (Maximum Transmission Unit) for Ethernet is typically 1500 bytes
        let mut buf = [0u8; 65536];

        loop {
            // Receive packets
            // recv function fills the buffer with incoming packet data
            // Need to convert buf to a pointer using as_mut_ptr()
            let size = recv(sock, buf.as_mut_ptr() as *mut _, buf.len(), 0);

            if size > 0 {
                println!("Received packet of size: {}", size);
                // NOTE: pass the buffer slice containing the packet data to parse_ethernet
                parse_ethernet(&buf[0..size as usize]);
            }
        }
    }
}

fn parse_ethernet(data: &[u8]) {
    // A raw packet starts with an Ethernet header
    // Ethernet header is 14 bytes long

    // 1. Destination MAC (6 bytes)
    // 2. Source MAC (6 bytes)
    // 3. EtherType (2 bytes)

    if data.len() < 14 {
        println!("Packet too short for Ethernet header");
        return;
    }

    let dst_max = &data[0..6];
    let src_mac = &data[6..12];
    let ether_type = &data[12..14];

    println!(
        "Ethernet Header: \n\tDestination MAC: {:02x?}\n\tSource MAC: {:02x?}\n\tEtherType: {:02x?}",
        dst_max, src_mac, ether_type
    );
    let ether_type_bytes = u16::from_be_bytes([ether_type[0], ether_type[1]]);

    if let Some(eth_type) = EtherType::from_u16(ether_type_bytes) {
        match eth_type {
            EtherType::IPv4 => println!("EtherType: IPv4"),
            EtherType::ARP => println!("EtherType: ARP"),
            EtherType::IPv6 => println!("EtherType: IPv6"),
        }
    } else {
        println!("EtherType: Unknown");
    }
}
