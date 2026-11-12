# PHASE 0.3 — ARCHITECTURAL DECISION RECORDS (ADR CATALOGUE)

**Project:** Mitra Audio Network System  
**Document Code:** `RECORD-ADR-ALL-011`  
**Status:** APPROVED FOR BLUEPRINT (Phase 0.3)  

---

## 1. Daftar Keputusan Arsitektur Resmi

### [ADR-006] Pemisahan Strict Boundary: Shared Core vs Platform Adapters
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** Sistem harus berjalan di 4 platform dengan karakteristik kernel yang sangat berbeda (Windows, Linux, macOS, MitraOS).
- **Keputusan:** Hanya matematika DSP, protokol paket, prioritas audio, data model, dan kriptografi yang dimasukkan ke dalam Rust Shared Core (`no_std` compatible). Seluruh driver hardware audio, socket I/O multiplexer, dan manajemen service didelegasikan ke Platform Abstraction Layer (PAL) masing-masing OS.
- **Konsekuensi:** Menghindari anti-pattern runtime wrapper berat dan menjaga ukuran biner tetap ringkas serta aman.

---

### [ADR-007] Kontrak Biner 32-bit x86 `__cdecl` untuk `IPNBSSDK.dll`
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** Aplikasi klien legacy (`IPNBS_Station.exe`) mengonsumsi `IPNBSSDK.dll` dengan konvensi panggilan `__cdecl`.
- **Keputusan:** Rust Shared Core akan mengompilasi DLL target `i686-pc-windows-msvc` dengan 37 fungsi ekspor C tanpa mangling nama dan menggunakan berkas `.def` untuk mengunci ordinal ekspor dari 1 s/d 37.
- **Konsekuensi:** Menjamin kompatibilitas drop-in binary replacement tanpa perlu mengubah kode sumber aplikasi stasiun lama.

---

### [ADR-008] Kebijakan Non-Destruktif Codec: Dual-Stack IMA-ADPCM & Opus
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** Opus adalah codec modern superior, namun perangkat keras speaker horn dan interkom analog lawas hanya memiliki decoder firmware IMA-ADPCM / PCM.
- **Keputusan:** Opus tidak boleh menggantikan ADPCM secara sepihak. Sistem menerapkan engine transcoding otomatis yang menyiarkan IMA-ADPCM untuk terminal legacy dan Opus untuk terminal/aplikasi modern.
- **Konsekuensi:** Fleksibilitas tinggi dan mencegah malfungsi hardware di lapangan.

---

### [ADR-009] Penggantian Dongle USB dengan Skema Lisensi Asimetris Ed25519
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** Dongle USB Feitian ET99 menggunakan driver x86 lama yang tidak kompatibel dengan Linux 64-bit, macOS Apple Silicon, dan MitraOS.
- **Keputusan:** Mengadopsi tanda tangan digital Ed25519 dengan format berkas lisensi `.lic` yang ditandatangani secara asimetris. Kunci privat hanya disimpan di lingkungan penerbit lisensi (Issuer Portal). Dilengkapi compatibility layer bagi pemanggil API lama.
- **Konsekuensi:** Independen dari perangkat keras USB proprietary dan aman dari pemalsuan lisensi.

---

### [ADR-010] Standardisasi Penyimpanan Lokal Menggunakan SQLite 3 WAL Mode
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** Basis data lama Microsoft Jet 4.0 Access MDB sering rusak saat mati lampu dan tidak didukung di luar Windows 32-bit.
- **Keputusan:** Memigrasikan seluruh 14 tabel legacy ke SQLite 3 dengan mode Write-Ahead Logging (WAL) dan foreign keys diaktifkan. Disediakan alat konversi data otomatis satu kali saat instalasi.
- **Konsekuensi:** Tahan terhadap pemadaman listrik mendadak, portabel di semua platform, dan bebas biaya lisensi DBMS server.

---

### [ADR-011] Isolasi Kernel ASL (MitraOS) & Pendekatan 8-Stage Roadmap
- **Status:** DITERIMA
- **Tanggal:** 11 November 2026
- **Konteks:** MitraOS / ASL Kernel adalah sistem operasi independen yang masih dalam tahap pembangunan aktif (Prioritas 4).
- **Keputusan:** Kode kernel di `C:\asl` dipertahankan dalam status read-only (Preservation Lock). Pengembangan driver audio dan network multicast MitraOS dibagi menjadi 8 tahapan terpisah (Stage A s/d Stage H) yang hanya dieksekusi secara berurutan.
- **Konsekuensi:** Mencegah kontaminasi arsitektur kernel dan menjamin platform Windows (Prioritas 1) stabil terlebih dahulu.
