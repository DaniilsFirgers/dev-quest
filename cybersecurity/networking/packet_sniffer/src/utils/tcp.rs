// MSS: Maximum Segment Size - trasnport layer (TCP) maximum payload size
// TCP advertises MSS during the SYN handshake

const TCP_HEADER_SIZE: usize = 20

// TCP segment size 
// TCP uses 16-bit length field, so 2^16 - 1 = 65535 bytes maximum
// It includes IP header, TCP header, and TCP payload, so 65535 - 20 (IP header) - 20 (TCP header) = 65515 bytes for TCP payload

// BUT, actual maximum segment size (MSS) is often lower due to network constraints like MTU
// Common MSS values are 1460 bytes for Ethernet (1500 MTU - 20 IP header - 20 TCP header)

// IMPORTANT:
// \r\n\r\n indicates the end of HTTP headers

pub fn parse_tcp(_data: &[u8]) {
    if _data.len() < TCP_HEADER_SIZE {
        println!("Data too short to contain a valid TCP header.");
        return;
    }

    let scr_port = u16::from_be_bytes([_data[0], _data[1]]);
    let dst_port = u16::from_be_bytes([_data[2], _data[3]]);

    // We look for 4 bits in the 13th byte (index 12) and then multiply by 4
    // because the data offset is given in 32-bit words
    let data_offset = (_data[12] >> 4) * 4; // in bytes

    // Ensure the data length is sufficient for the TCP header with options
    if _data.len() < data_offset as usize {
        println!("Data too short for TCP header with options.");
        return;
    }

    let payload = &_data[data_offset as usize..];
    let is_http_request = dst_port == 80;

    if is_http_request {
        println!("HTTP request detected on port 80");
        if !payload.is_empty() {
            if let Ok(payload_str) = std::str::from_utf8(payload) {
                println!("HTTP Payload:\n{}", payload_str);
            } else {
                println!("HTTP Payload contains non-UTF8 data.");
            }
        }
    }
}
