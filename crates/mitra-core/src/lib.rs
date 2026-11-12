//! Mitra Audio Network System — Pure Portable Shared Core
//!
//! Mengimplementasikan model domain, penanganan protokol biner tagDGRAMHEADER,
//! model format audio dan codec metadata, model zona dan prioritas audio,
//! serta fondasi verifikasi lisensi kriptografis asimetris Ed25519.

pub mod audio;
pub mod error;
pub mod license;
pub mod priority;
pub mod protocol;
pub mod zone;

pub use audio::*;
pub use error::*;
pub use license::*;
pub use priority::*;
pub use protocol::*;
pub use zone::*;
