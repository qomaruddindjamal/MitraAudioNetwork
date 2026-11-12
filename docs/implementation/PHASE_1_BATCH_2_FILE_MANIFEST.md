# PHASE 1 BATCH 2 — FILE MANIFEST & ASSET REGISTRATION

**Project:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Date:** 12 November 2026  
**Checkpoint:** `CP-PHASE-1-BATCH-2-WINDOWS-PLATFORM-COMPLETE`  

---

## 1. Daftar Berkas Baru yang Dibuat pada Batch 2

| Relative Path | Deskripsi Fungsi & Peran | Bahasa / Tipe |
|---|---|---|
| `crates/mitra-adapter-windows/Cargo.toml` | Manifest crate Platform Abstraction Layer untuk Windows (WASAPI, Winsock, WMI HWID). | TOML |
| `crates/mitra-adapter-windows/src/lib.rs` | Entry point modul `mitra-adapter-windows`. | Rust |
| `crates/mitra-adapter-windows/src/audio_wasapi.rs` | Implementasi backend audio WASAPI (AudioBackend & AudioStreamHandle). | Rust |
| `crates/mitra-adapter-windows/src/net_windows.rs` | Implementasi Winsock2 UDP Multicast transport (`socket2`, SO_REUSEADDR, IP_ADD_MEMBERSHIP). | Rust |
| `crates/mitra-adapter-windows/src/hw_windows.rs` | Implementasi Windows Hardware ID Provider & SHA-256 Native ID derivation. | Rust |
| `crates/mitra-adapter-windows/tests/windows_tests.rs` | Automated integration tests untuk backend audio dan HWID Windows. | Rust |
| `crates/mitra-sdk-c/Cargo.toml` | Manifest crate FFI C-ABI drop-in replacement `IPNBSSDK.dll`. | TOML |
| `crates/mitra-sdk-c/IPNBSSDK.def` | Module Definition file untuk mengunci ordinal ekspor (1 s/d 37). | DEF |
| `crates/mitra-sdk-c/src/lib.rs` | Implementasi 37 fungsi ekspor C-ABI `extern "C"` (`__cdecl`) dengan thread-safe state. | Rust |
| `crates/mitra-sdk-c/tests/sdk_ffi_tests.rs` | Automated tests pemanggilan seluruh 37 fungsi SDK dan session lifecycle. | Rust |
| `target/i686-pc-windows-msvc/debug/IPNBSSDK.dll` | Biner hasil kompilasi 32-bit x86 PE DLL untuk compatibility drop-in test. | Binary PE DLL |
| `docs/implementation/PHASE_1_BATCH_2_IMPLEMENTATION_REPORT.md` | Laporan komprehensif implementasi Batch 2. | Markdown |
| `docs/implementation/PHASE_1_BATCH_2_TEST_REPORT.md` | Laporan hasil eksekusi pengujian otomatis Batch 2. | Markdown |
| `docs/implementation/PHASE_1_BATCH_2_PROTOCOL_COMPATIBILITY_REPORT.md`| Laporan evaluasi & koreksi protokol wire legacy (Batch 2A). | Markdown |
| `docs/implementation/PHASE_1_BATCH_2_WINDOWS_PLATFORM_REPORT.md` | Laporan implementasi platform adapter Windows (Batch 2B). | Markdown |
| `docs/implementation/PHASE_1_BATCH_2_FFI_COMPATIBILITY_MATRIX.md` | Matriks kompatibilitas biner 37 API SDK (Batch 2C). | Markdown |
| `docs/implementation/PHASE_1_BATCH_2_FILE_MANIFEST.md` | Manifest inventaris berkas Batch 2. | Markdown |

---

## 2. Berkas Legacy & Sistem Operasi yang Dilindungi (Untouched)

- Biner Legacy Asli di `C:\IPNBS`:
  - `IPNBS_Server.exe` (`7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9`) — UTUH
  - `IPNBS_Station.exe` (`61286ABAD5EFCA2AA7721415D870DF2F60EEC0FB9CF3B2D1ADE6A2F74904C0E2`) — UTUH
  - `IPNBSSDK.dll` (`FBDC81009FF4306504C6374F6C3D18A54663EF699900A93C8A7489B30B904211`) — UTUH
  - `FT_ET99_API.dll` (`7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE`) — UTUH
- Direktori ASL Kernel (`C:\asl`): Utuh 100% tanpa modifikasi.
- Direktori MitraOS (`C:\mitraOS`): Utuh 100% tanpa modifikasi.
- Git Repository: Bebas dari commit atau push yang tidak terotorisasi.
