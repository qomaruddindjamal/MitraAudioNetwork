//! Trait Abstraksi Transport Jaringan

use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkPalError {
    AddressInUse,
    AddressNotAvailable,
    MulticastJoinFailed(String),
    InterfaceNotFound(String),
    ConnectionRefused,
    Timeout,
    IoError(String),
}

pub trait MulticastSocket: Send + Sync {
    fn send_to_multicast(&self, packet: &[u8]) -> Result<usize, NetworkPalError>;
    fn recv_multicast(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddrV4), NetworkPalError>;
    fn leave_group(&self) -> Result<(), NetworkPalError>;
}

pub trait NetworkTransport: Send + Sync {
    fn name(&self) -> &'static str;
    fn bind_multicast(
        &self,
        multicast_addr: Ipv4Addr,
        port: u16,
        interface_addr: Option<Ipv4Addr>,
    ) -> Result<Box<dyn MulticastSocket>, NetworkPalError>;
}
