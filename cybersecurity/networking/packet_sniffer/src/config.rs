use serde::Deserialize;
use std::fs::File;

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open("config.yaml")?;
    let config: Config = serde_yaml::from_reader(file)?;
    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub target_server: Option<TargetServer>,
    pub arp: Arp,
    pub ipv4: Ipv4,
    pub protocols: Protocols,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TargetServer {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Arp {
    pub log: bool,
    pub kind: String, // "all", "requests", "replies"
}

#[derive(Debug, Deserialize)]
pub struct Ipv4 {
    pub log: bool,
    pub protocols: Vec<String>, // ["tcp", "udp", "icmp"]
    pub include_headers: bool,
}

#[derive(Debug, Deserialize)]
pub struct Protocols {
    pub tcp: Tcp,
}

#[derive(Debug, Deserialize)]
pub struct Tcp {
    pub log: bool,
}
