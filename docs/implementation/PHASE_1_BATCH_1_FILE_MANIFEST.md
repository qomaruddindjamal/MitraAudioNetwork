# PHASE 1 BATCH 1 — FILE MANIFEST & ASSET REGISTRATION

**Project:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Date:** 11 November 2026  
**Checkpoint:** `CP-PHASE-1-BATCH-1-FOUNDATION-COMPLETE`  

---

## 1. Daftar Berkas Baru yang Dibuat

| Relative Path | Deskripsi Fungsi & Peran | Bahasa / Tipe |
|---|---|---|
| `Cargo.toml` | Root Workspace definition yang memayungi seluruh crates proyek. | TOML |
| `crates/mitra-pal/Cargo.toml` | Manifest konfigurasi crate Platform Abstraction Layer (PAL). | TOML |
| `crates/mitra-pal/src/lib.rs` | Entry point modul `mitra-pal`. | Rust |
| `crates/mitra-pal/src/audio.rs` | Trait abstraksi perangkat dan stream audio (`AudioBackend`, `AudioStreamHandle`). | Rust |
| `crates/mitra-pal/src/network.rs` | Trait abstraksi soket UDP Multicast dan transport jaringan (`NetworkTransport`). | Rust |
| `crates/mitra-pal/src/hardware.rs`| Trait abstraksi hardware query (`HardwareIdProvider`, `HardwareToken`). | Rust |
| `crates/mitra-core/Cargo.toml` | Manifest konfigurasi crate Shared Core domain logic. | TOML |
| `crates/mitra-core/src/lib.rs` | Entry point modul `mitra-core`. | Rust |
| `crates/mitra-core/src/error.rs` | Taksonomi error sistem: Protocol, Audio, Priority, License, Validation. | Rust |
| `crates/mitra-core/src/protocol.rs` | Parser & Serializer biner 16-byte `tagDGRAMHEADER` dan paket datagram UDP. | Rust |
| `crates/mitra-core/src/audio.rs` | Model metadata format audio dan validasi codec (PCM, ADPCM, Opus, MP3). | Rust |
| `crates/mitra-core/src/priority.rs` | State machine arbitrase kanal siaran audio & preemption rule. | Rust |
| `crates/mitra-core/src/zone.rs` | Domain model entitas ZoneInfo dan TerminalInfo. | Rust |
| `crates/mitra-core/src/license.rs` | Engine verifikasi lisensi digital asimetris Ed25519 & claims validator. | Rust |
| `crates/mitra-core/tests/core_tests.rs` | Test suite pengujian otomatis untuk protocol, audio, priority, dan license. | Rust |
| `docs/implementation/PHASE_1_BATCH_1_IMPLEMENTATION_REPORT.md` | Laporan komprehensif implementasi Batch 1. | Markdown |
| `docs/implementation/PHASE_1_BATCH_1_TEST_REPORT.md` | Laporan hasil eksekusi pengujian otomatis Batch 1. | Markdown |
| `docs/implementation/PHASE_1_BATCH_1_FILE_MANIFEST.md` | Manifest inventaris berkas Batch 1. | Markdown |

---

## 2. Berkas Legacy yang Dilindungi (Untouched)

Seluruh berkas biner asli di root `C:\IPNBS\` tetap berada di bawah **Preservation Lock** dengan hash SHA-256 yang identik:
- `IPNBS_Server.exe` (`7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9`)
- `IPNBS_Station.exe` (`61286ABAD5EFCA2AA7721415D870DF2F60EEC0FB9CF3B2D1ADE6A2F74904C0E2`)
- `IPNBSSDK.dll` (`FBDC81009FF4306504C6374F6C3D18A54663EF699900A93C8A7489B30B904211`)
- `FT_ET99_API.dll` (`7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE`)
- Direktori ASL Kernel (`C:\asl`): Tidak disentuh sama sekali.
