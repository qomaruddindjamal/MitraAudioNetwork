# MITRA AUDIO NETWORK SYSTEM — DECISION LOG

Dokumen ini mencatat keputusan arsitektural dan teknis (ADR - Architectural Decision Records) sepanjang siklus proyek.

---

## DAFTAR KEPUTUSAN ARSITEKTUR

### [ADR-001] Penetapan Toolchain Investigasi Dinamis
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Mengadopsi NSA Ghidra 12.1.4 PUBLIC dengan runtime Eclipse Temurin JDK 21 LTS dan REA 6.8.0 MCP Server.

---

### [ADR-002] Strategi Migrasi Database: SQLite 3 WAL Mode
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Mengganti Jet Engine dengan SQLite 3 WAL mode sebagai penyimpanan lokal di Shared Core Rust, dilengkapi one-time data migration tool otomatis saat instalasi.

---

### [ADR-003] Standar Kriptografi Lisensi Baru: Ed25519
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Menerapkan skema lisensi asimetris modern berbasis Ed25519 digital signature file (`.lic`) dengan multi-platform hardware fingerprinting, sembari mempertahankan wrapper kompatibilitas `FT_ET99_API.dll` untuk sistem lama.

---

### [ADR-004] Resolusi Error IOCP 995 (`WSA_OPERATION_ABORTED`)
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Menggunakan model async non-blocking berbasis Tokio di Rust Core yang menangani pembatalan soket dan teardown koneksi secara graceful tanpa false positive error logs.

---

### [ADR-005] Kebijakan Isolasi & Roadmap 8-Stage ASL Kernel (MitraOS)
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Tidak mengubah source code kernel di `C:\asl` selama tahap investigasi. Pengembangan MitraOS akan dilakukan secara bertahap melalui 8-stage roadmap (Stage A s/d Stage H) setelah Core Windows stabil.

---

### [ADR-006] Pemisahan Strict Boundary: Shared Core vs Platform Adapters
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Membatasi Shared Core pada matematika DSP, protokol paket, prioritas audio, data model, dan kriptografi. Seluruh driver hardware dan soket didelegasikan ke Platform Abstraction Layer (PAL).

---

### [ADR-007] Kontrak Biner 32-bit x86 `__cdecl` untuk `IPNBSSDK.dll`
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Rust Shared Core mengompilasi dynamic library `i686-pc-windows-msvc` dengan berkas `.def` untuk mengunci urutan 37 ordinal ekspor secara biner.

---

### [ADR-008] Kebijakan Non-Destruktif Codec: Dual-Stack IMA-ADPCM & Opus
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Sistem menerapkan engine transcoding otomatis: menyiarkan IMA-ADPCM untuk perangkat lawas dan Opus untuk stasiun/terminal modern.

---

### [ADR-009] Penempatan Kunci Privat Lisensi Asimetris
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Private Signing Key Ed25519 hanya berada di Issuer Vault/Portal. Biner aplikasi yang didistribusikan hanya memuat Public Verification Key.

---

### [ADR-010] Desain Toleransi Hardware ID & Anti-Clock Tampering
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Menggunakan fuzzy hardware score (2 dari 3 komponen identik) dan monotonic kernel timestamp watermarking untuk mendeteksi rollback tanggal sistem.

---

### [ADR-011] Isolasi Kernel ASL (MitraOS) & Pendekatan 8-Stage Roadmap
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Keputusan:** Menerapkan 8-stage roadmap tanpa menggunakan emulator Linux syscall, dan menjaga direktori `C:\asl` tetap read-only.

---

### [ADR-012] Penutupan Definitif Siklus Riset (R&D Final Closure)
- **Status:** DITERIMA & MENGIKAT
- **Tanggal:** 11 November 2026
- **Konteks:** Menegakkan kepatuhan mutlak terhadap directive Phase 0.4.
- **Keputusan:** Menetapkan Phase 0.4 sebagai gerbang akhir R&D. Tidak ada Phase 0.5, Phase 0.6, atau pengulangan audit pendahuluan. Seluruh aktivitas audit dinyatakan selesai penuh dengan status PASS. Fase proyek berikutnya adalah Phase 1 — IMPLEMENTATION.

---

### [ADR-013] Penataan Presisi 37 Ekspor Ordinal `IPNBSSDK.dll` Legacy ABI
- **Status:** DITERIMA & DITERAPKAN
- **Tanggal:** 12 November 2026
- **Konteks:** Menghilangkan pergeseran nomor ordinal akibat fungsi internal non-standar, menjamin kompatibilitas biner drop-in x86 bagi aplikasi host lama (`IPNBS_Server.exe` / `IPNBS_Station.exe`).
- **Keputusan:** Menata ulang file `IPNBSSDK.def` dan `crates/mitra-sdk-c/src/lib.rs` sehingga tepat memuat 37 fungsi dengan ordinal 1..37 identik 1:1 terhadap biner legacy hasil inspeksi `dumpbin.exe /EXPORTS`.

---

### [ADR-014] Formula Penilaian Bobot Kesamaan Native ID Windows & Kebijakan Reaktivasi
- **Status:** DITERIMA & DITERAPKAN
- **Tanggal:** 12 November 2026
- **Konteks:** Mencegah pembatalan lisensi sepihak saat pemeliharaan hardware normal (penggantian disk boot) sekaligus mencegah pembajakan via disk cloning.
- **Keputusan:** Mengadopsi formula penilaian kesamaan token berbobot: Motherboard UUID (40%), CPU Processor ID (35%), System Drive Serial (25%). Ambang batas: >= 75% Auto-Pass Valid, 50% - 74% Grace Period 7 hari, < 50% Revocation seketika dengan kewajiban reaktivasi bertanda tangan kriptografi Ed25519. Versi kebijakan diisolasi secara kriptografis menggunakan domain separation (`Sha256(policy_version : tokens)`).

---

### [ADR-015] Model Lisensi Permanen Ed25519 & Pemisahan Masa Dukungan Penggantian SN
- **Status:** DITERIMA & DITERAPKAN
- **Tanggal:** 12 November 2026
- **Konteks:** Menegaskan aturan bisnis bahwa lisensi perangkat bersifat permanen tanpa waktu kedaluwarsa, sembari mencatat batas masa layanan dukungan penggantian SN selama 1 tahun secara terpisah.
- **Keputusan:**
  1. Menetapkan format payload lisensi kanonikal `schema_version = 1` dengan `license_type = "permanent"`. Tidak menyertakan expiry timestamp pada payload yang ditandatangani.
  2. Serial Number (SN) format baku `MANS-<PROD4>-<HASH4>-<HASH4>-<CHK4>` dibangkitkan dari Product ID, Native ID, dan Authority Salt. SN adalah identifier pendaftaran, bukan bukti tanda tangan digital.
  3. Tanggal pembelian (`Purchase Date`) dan batas layanan dukungan penggantian SN (`Support Until` = Purchase Date + 1 tahun) disimpan eksklusif pada basis data License Authority.
  4. Penegakan hak fitur aplikasi (misal `WEB_MANAGEMENT`) diwajibkan melalui verifikasi tanda tangan kriptografi di backend domain logic.
  5. Menolak kompatibilitas langsung adapter lisensi Feitian ET99 / SPON legacy karena perbedaan arsitektur mendasar dan ketiadaan spesifikasi resmi (status: *Belum Dapat Dibuktikan*).

---

### [ADR-016] Temuan Forensik Installer SPON V3.3.0 & Peran Opsional Dongle Feitian ET99
- **Status:** DITERIMA & DITERAPKAN
- **Tanggal:** 12 November 2026
- **Konteks:** Hasil pembongkaran biner installer `C:\Tools\SetupIPNBS_V3.3.0.exe` dan dekompilasi Ghidra terhadap eksekutif legacy `IPNBS_Server.exe`, `IPNBS_Station.exe`, dan `IPNBSSDK.dll`.
- **Keputusan:**
  1. Installer `SetupIPNBS_V3.3.0.exe` dibangun dengan teknologi **Setup Factory 9.0** (`irsetup.exe`), murni ekstraksi file biner dan dependensi runtime tanpa aktivasi lisensi atau validasi hardware dongle saat instalasi.
  2. Dongle hardware **Feitian ET99 (`FT_ET99_API.dll`) terbukti secara biner hanya bersifat opsional**. Satu-satunya pemanggil `et_FindToken` di seluruh workspace hanyalah `IPNBS_Server.exe` pada RVA `0x00418F62`. `IPNBS_Station.exe` dan `IPNBSSDK.dll` sama sekali tidak memiliki dependensi maupun impor terhadap `FT_ET99_API.dll`.
  3. Apabila dongle ET99 tidak terpasang (`et_FindToken != 0`), `IPNBS_Server.exe` secara mulus jatuh kembali (*fallback*) ke verifikasi teks `SerialNumber` yang tersimpan di file konfigurasi lokal `IPNBS_Server.ini`.
  4. Format SerialNumber legacy adalah 29 karakter (5 blok berisi 5 karakter alfanumerik dipisahkan tanda strip `-`) yang diverifikasi dengan fungsi hashing MD5 internal.
  5. Mitra Audio Network System menetapkan bahwa arsitektur lisensi modern berbasis Ed25519 dan Serial Number MANS (`MANS-PROD-HASH-HASH-CHKS`) sepenuhnya independen dari dongle fisik ET99, namun tetap kompatibel secara operasional tanpa memerlukan dongle USB pihak ketiga.
