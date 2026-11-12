//! Mitra Zone & Channel Domain Model

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneInfo {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub priority_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalInfo {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub ip_address: [u8; 4],
    pub port: u16,
    pub zone_id: Option<u32>,
    pub volume: u8,
    pub is_online: bool,
}

impl TerminalInfo {
    pub fn new(id: u32, code: String, name: String, ip_address: [u8; 4], port: u16) -> Self {
        Self {
            id,
            code,
            name,
            ip_address,
            port,
            zone_id: None,
            volume: 80,
            is_online: false,
        }
    }
}
