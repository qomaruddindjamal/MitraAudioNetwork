//! Windows Network Transport Adapter
//!
//! Mengimplementasikan Windows Socket 2 (Winsock) UDP Multicast transport
//! menggunakan `socket2` dengan konfigurasi SO_REUSEADDR dan IP_ADD_MEMBERSHIP.

use mitra_pal::{MulticastSocket, NetworkPalError, NetworkTransport};
use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct WindowsMulticastSocket {
    socket: Socket,
    multicast_addr: Ipv4Addr,
    port: u16,
    interface_addr: Ipv4Addr,
}

impl MulticastSocket for WindowsMulticastSocket {
    fn send_to_multicast(&self, packet: &[u8]) -> Result<usize, NetworkPalError> {
        let dest = SocketAddr::V4(SocketAddrV4::new(self.multicast_addr, self.port));
        self.socket
            .send_to(packet, &dest.into())
            .map_err(|e| NetworkPalError::IoError(e.to_string()))
    }

    fn recv_multicast(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddrV4), NetworkPalError> {
        let mut uninit_buf: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); buffer.len()];

        let (bytes_read, addr) = self
            .socket
            .recv_from(&mut uninit_buf)
            .map_err(|e| NetworkPalError::IoError(e.to_string()))?;

        // Safely extract initialized bytes
        let init_slice =
            unsafe { std::slice::from_raw_parts(uninit_buf.as_ptr() as *const u8, bytes_read) };
        buffer[..bytes_read].copy_from_slice(init_slice);

        if let Some(sock_addr_v4) = addr.as_socket_ipv4() {
            Ok((bytes_read, sock_addr_v4))
        } else {
            Err(NetworkPalError::IoError(
                "Non-IPv4 sender address".to_string(),
            ))
        }
    }

    fn leave_group(&self) -> Result<(), NetworkPalError> {
        self.socket
            .leave_multicast_v4(&self.multicast_addr, &self.interface_addr)
            .map_err(|e| NetworkPalError::IoError(e.to_string()))
    }
}

pub struct WindowsNetworkTransport {
    name: &'static str,
}

impl Default for WindowsNetworkTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsNetworkTransport {
    pub fn new() -> Self {
        Self {
            name: "Windows Winsock2 Transport",
        }
    }
}

impl NetworkTransport for WindowsNetworkTransport {
    fn name(&self) -> &'static str {
        self.name
    }

    fn bind_multicast(
        &self,
        multicast_addr: Ipv4Addr,
        port: u16,
        interface_addr: Option<Ipv4Addr>,
    ) -> Result<Box<dyn MulticastSocket>, NetworkPalError> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
            .map_err(|e| NetworkPalError::IoError(e.to_string()))?;

        // Mengizinkan multiple sockets binding pada port multicast yang sama
        socket
            .set_reuse_address(true)
            .map_err(|e| NetworkPalError::IoError(e.to_string()))?;

        let bind_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
        socket
            .bind(&bind_addr.into())
            .map_err(|e| NetworkPalError::IoError(e.to_string()))?;

        let iface = interface_addr.unwrap_or(Ipv4Addr::UNSPECIFIED);

        socket
            .join_multicast_v4(&multicast_addr, &iface)
            .map_err(|e| NetworkPalError::MulticastJoinFailed(e.to_string()))?;

        // Atur Multicast Time-To-Live (TTL) standar LAN
        socket
            .set_multicast_ttl_v4(32)
            .map_err(|e| NetworkPalError::IoError(e.to_string()))?;

        Ok(Box::new(WindowsMulticastSocket {
            socket,
            multicast_addr,
            port,
            interface_addr: iface,
        }))
    }
}
