#[derive(Debug)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    ARP = 0x0806,
    IPv6 = 0x86DD,
}

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
