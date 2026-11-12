# PHASE 0.4 — TESTING AND ACCEPTANCE MATRIX (VERIFICATION FRAMEWORK)

**Project:** Mitra Audio Network System  
**Document Code:** `MATRIX-TEST-FINAL-007`  
**Status:** READY FOR PHASE 1 IMPLEMENTATION (Phase 0.4 Final Gate)  

---

## 1. Automated Testing Framework & Gates

Untuk memastikan stabilitas kelas enterprise, setiap tahap implementasi pada Phase 1 dipagari oleh automated test suites:

```mermaid
flowchart LR
    DEV[Developer Code / PR] --> GATE1[Gate 1: Static Analysis & Lints\n(cargo clippy & fmt)]
    GATE1 --> GATE2[Gate 2: Unit Tests\n(DSP Math, Framing, Crypto)]
    GATE2 --> GATE3[Gate 3: ABI Compatibility Tests\n(C Harness 37 APIs)]
    GATE3 --> GATE4[Gate 4: Audio Loopback & THD Tests\n(THD+N < -90dB, Latency < 10ms)]
    GATE4 --> GATE5[Gate 5: Network Stress & Jitter Tests\n(500 Terminals, 10% Loss)]
    GATE5 --> MERGE[Pass to Release Candidate]
```

---

## 2. Rincian Matriks Pengujian & Kriteria Keberhasilan

| ID Pengujian | Modul Target | Metodologi & Skenario Uji | Parameter / Metrik Terukur | Kriteria Keberhasilan (Pass / Fail Criteria) |
|---|---|---|---|---|
| **TST-ABI-01** | `mitra-sdk-c` (`IPNBSSDK.dll`) | Aplikasi harness C memanggil seluruh 37 fungsi ekspor C-ABI dengan argumen valid dan invalid. | - 37 fungsi ekspor terpanggil.<br>- 0 Memory Violation / Access Violation. | **PASS:** Seluruh return code sesuai spesifikasi, callback pointer terpanggil tanpa crash stack. |
| **TST-AUD-01** | WASAPI Driver | Perekaman audio mikrofon server dan loopback playback terus-menerus selama 2 jam. | - Buffer latency: **< 10 ms**.<br>- Buffer underrun counter: **0**. | **PASS:** Suara bebas dari audio glitch, pop, atau desis; memory footprint stabil (< 50 MB). |
| **TST-AUD-02** | Audio Resampling (Rubato) | Konversi sinyal sinus uji 1 kHz dari 44.1 kHz ke 48 kHz dan 22.05 kHz. | - Total Harmonic Distortion (THD+N): **< -90 dB**.<br>- SNR: **> 96 dB**. | **PASS:** Spektogram audio bersih tanpa aliasing artifact atau distorsi frekuensi tinggi. |
| **TST-NET-01** | UDP Multicast Stream | Pengiriman paket audio multicast `234.0.0.254:2048` ke 50 terminal penerima virtual. | - Packet Delivery Ratio: **> 99.8%**.<br>- Jitter delay: **< 5 ms**. | **PASS:** Semua 50 terminal menerima paket dengan sequence number lengkap dan terurut. |
| **TST-NET-02** | Jitter Buffer & Packet Loss | Injeksi gangguan jaringan buatan: 10% random packet drop + 30ms network jitter. | - Opus PLC Activation: **100%**.<br>- Audio Dropouts: **0**. | **PASS:** Kualitas vokal tetap terdengar jelas tanpa jeda audio atau terputusnya koneksi. |
| **TST-CON-01** | Concurrency & IOCP | Simulasi 500 koneksi terminal TCP simultan yang mengirimkan heartbeat setiap 3 detik. | - CPU Utilization Server: **< 15%**.<br>- Server Error Count: **0**. | **PASS:** Tidak ada error IOCP 995, waktu respons command siaran ke 500 terminal < 100 ms. |
| **TST-SEC-01** | Ed25519 Cryptography | Verifikasi berkas lisensi sah vs berkas lisensi palsu (dengan 1 byte dimodifikasi). | - Verification Time: **< 5 ms**.<br>- False Positive Rate: **0%**. | **PASS:** Lisensi sah langsung diterima; lisensi yang dimanipulasi ditolak seketika. |
| **TST-SEC-02** | Anti-Clock Rollback | Memundurkan kalender waktu sistem operasi mundur 6 bulan saat audio server berjalan. | - Tamper Detection Time: **< 1 detik**. | **PASS:** Sistem langsung mengunci fitur siaran darurat/komersial dan mencatat audit log keamanan. |
| **TST-DAT-01** | Database Migration | Migrasi berkas database `DBS.mdb` legacy (berisi 150 terminal, 12 zona, 30 jadwal bel) ke SQLite. | - Integrity Check: `PRAGMA integrity_check`.<br>- Data Loss: **0 record**. | **PASS:** Jumlah baris dan relasi terminal, zona, serta jadwal bel identik 100%. |
| **TST-INS-01** | Windows Installer | Instalasi pada Windows 10/11 mesin bersih, restart sistem, dan pengujian uninstall. | - Install Time: **< 30 detik**.<br>- Service Status: `Running`. | **PASS:** Service `MitraAudioService` berjalan otomatis, port firewall terbuka, uninstall bersih. |
