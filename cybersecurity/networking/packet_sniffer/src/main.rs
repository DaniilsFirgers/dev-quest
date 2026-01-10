use libc::*;

mod config;
mod utils;
use crate::utils::ethernet::parse_ethernet;

fn main() {
    let config = config::read_config();
    if let Err(e) = config {
        panic!("Failed to read config: {}", e);
    }

    let config = config.unwrap();

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
                // NOTE: pass the buffer slice containing the packet data to parse_ethernet
                parse_ethernet(&buf[0..size as usize], &config);
            }
        }
    }
}
