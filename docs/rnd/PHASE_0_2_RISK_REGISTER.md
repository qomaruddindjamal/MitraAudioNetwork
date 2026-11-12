# PHASE 0.2 — TECHNICAL RISK REGISTER & MITIGATION STRATEGY

**Project:** Mitra Audio Network System  
**Review Level:** Phase 0.2 Technical Deep Investigation  
**Last Updated:** 11 November 2026  

---

## 1. Risk Matrix

| Risk ID | Deskripsi Risiko | Dampak | Probabilitas | Tingkat Risiko | Strategi Mitigasi / Penanganan |
|---|---|---|---|---|---|
| **RSK-01** | Ketergantungan database Microsoft Jet DAO 3.6 (`DBS.mdb`) yang tidak tersedia di Linux, macOS, dan MitraOS. | Kritis | Pasti (100%) | **TINGGI** | Mengembangkan modul migrasi otomatis dari MDB ke SQLite 3 WAL engine di Shared Core Rust. Menjamin backward data portability. |
| **RSK-02** | Dongle USB Feitian ET99 tidak memiliki driver di Linux 64-bit, macOS ARM64, dan MitraOS. | Kritis | Pasti (100%) | **TINGGI** | Desain dual-license mode: mendukung kompatibilitas token lama di Windows via wrapper, sekaligus menerapkan standar Ed25519 digital license file untuk platform modern. |
| **RSK-03** | Pembatasan UDP Multicast pada router WAN / VLAN korporat memutus audio broadcast. | Tinggi | Sedang (50%) | **SEDANG** | Implementasikan hybrid routing: deteksi otomatis multicast support dengan fallback cerdas ke Unicast Multi-Stream UDP / TCP tunneling. |
| **RSK-04** | Error legacy IOCP 995 (`WSA_OPERATION_ABORTED`) saat disconnect socket membingungkan operator. | Rendah | Tinggi (80%) | **RENDAH** | Menggunakan async runtime modern (Tokio/Mio) yang menangani lifecycle soket secara graceful tanpa menimbulkan false positive error log. |
| **RSK-05** | ASL Kernel (MitraOS) belum memiliki driver audio dan network socket. | Kritis | Pasti (100%) | **TINGGI** | Mengikuti 8-stage development roadmap secara disiplin (Stage A s/d Stage H). Jangan menurunkan target menjadi Linux compatibility layer, dan jangan mengubah source ASL sebelum Stage A disetujui. |
| **RSK-06** | Regresi kompatibilitas fungsi SDK dengan aplikasi Station lama. | Tinggi | Rendah (20%) | **SEDANG** | Mempertahankan biner FFI `__cdecl` 37 API functions di `IPNBSSDK.dll` dengan signature byte-level identik, dilengkapi automated harness integration tests. |
