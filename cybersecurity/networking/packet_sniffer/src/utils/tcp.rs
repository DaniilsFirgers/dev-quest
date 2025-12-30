const TCP_HEADER_SIZE: usize = 20;

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
    if scr_port == 80 || dst_port == 80 {
        println!("HTTP Traffic Detected on port 80");
        println!("TCP payload size: {} bytes", payload.len());
    }
}
