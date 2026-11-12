//! Windows Platform Abstraction Layer Adapter
//!
//! Mengimplementasikan trait `AudioBackend`, `NetworkTransport`, dan `HardwareIdProvider`
//! untuk lingkungan sistem operasi Microsoft Windows.

pub mod audio_wasapi;
pub mod hw_windows;
pub mod net_windows;

pub use audio_wasapi::*;
pub use hw_windows::*;
pub use net_windows::*;
