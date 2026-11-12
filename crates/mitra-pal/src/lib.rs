//! Mitra Platform Abstraction Layer (PAL) Traits
//!
//! Menentukan kontrak antarmuka antara Shared Core dan Adapter Sistem Operasi.
//! Tidak mengandung implementasi I/O atau syscall langsung.

pub mod audio;
pub mod hardware;
pub mod network;

pub use audio::*;
pub use hardware::*;
pub use network::*;
