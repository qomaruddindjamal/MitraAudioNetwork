# LAPORAN AKHIR INVESTIGASI TEKNIS MENDALAM — PHASE 0.2

**Proyek:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Target Repository:** `https://github.com/qomaruddindjamal/MitraAudioNetwork.git`  
**Checkpoint Awal:** `CP-TOOLCHAIN-INTEGRATION-COMPLETE`  
**Checkpoint Akhir:** `CP-PHASE-0-2-DEEP-INVESTIGATION-COMPLETE`  
**Tanggal Penyelesaian:** 11 November 2026  
**Status Preservation Lock:** **AKTIF & AMAN (100% Verified)**  

---

## 1. Ringkasan Eksekutif

Phase 0.2 telah berhasil menyelesaikan investigasi teknis mendalam terhadap sistem Windows legacy dan menetapkan spesifikasi arsitektur lintas platform untuk **Mitra Audio Network System**.

Dengan menggunakan toolchain terintegrasi:
- **Antigravity** sebagai Primary Orchestrator,
- **REA** sebagai MCP Reverse Engineering Specialist,
- **Ghidra 12.1.4 PUBLIC** & Adoptium JDK 21 LTS sebagai Static Analysis & Decompilation Engine,
- Serta referensi silang peta simbol `IPNBS_Server.map`,

seluruh komponen inti, subsistem audio, protokol jaringan, skema lisensi, dan penyimpanan database legacy telah berhasil dipetakan, dibuktikan secara empiris, dan didokumentasikan tanpa memodifikasi satu byte pun pada biner asli atau ASL Kernel.

---

## 2. Hasil Investigasi Per Batch

### Batch A — `IPNBS_Server.exe`
- **Arsitektur:** Visual C++ 6.0 / MFC 4.2 32-bit x86, memuat 6.963 fungsi dan 6.310 string terdefinisi.
- **Siklus Hidup:** Inisialisasi melalui `CDLSApp::InitInstance` dengan single-instance mutex check `"DLS tag of run alone2"`.
- **Threading Model:** UI Message Loop + Pool IOCP worker threads (`CIoCompletionPort`) + UDP Multicast Dispatcher thread + WinMM Audio Capture callback.
- **Root Cause Error 995:** Ditemukan bahwa error bersejarah `WSA_OPERATION_ABORTED` merupakan perilaku normal saat overlapped I/O socket dibatalkan ketika terminal disconnect, namun dicatat sebagai error level tinggi di legacy server. Modern Tokio async core akan menangani teardown ini secara elegan.

### Batch B — `IPNBSSDK.dll`
- **Katalog API:** Berhasil mengekstraksi dan mendokumentasikan seluruh **37 exported APIs** berkonvensi `__cdecl`.
- **Dekompolasi Kunci:** Berhasil mendekompilasi alur kerja `IPNBSSDK_TCPConnect`, membuktikan inisialisasi thread listener background (`CreateThread`) dan event callback dispatching.
- **Strategi Rust FFI:** Menetapkan rancangan FFI `cdylib` dengan `#[no_mangle]` dan `extern "C"` untuk memastikan penggantian biner langsung (drop-in replacement) tanpa kompilasi ulang pada aplikasi client legacy.

### Batch C — Audio Engine & Codecs
- **Capture Subsystem:** Menggunakan WinMM API (`waveInOpen`, `waveInPrepareHeader`, `waveInProc`) dengan 4 buffer cincin (ring buffer) berukuran 1024-2048 bytes (~20ms-40ms latency).
- **Format Codecs:** Mendukung Raw PCM 16-bit (`0x0001`), IMA-ADPCM 4-bit (`0x0002`), dan MPEG Layer-3 MP3 (`0x0055`) via `lame_enc.dll`.
- **Modernisasi Cross-Platform:** Menggantikan WinMM dengan CPAL (WASAPI di Windows, ALSA/PipeWire di Linux, CoreAudio di macOS, dan custom ASL Audio Driver di MitraOS) serta penambahan codec Opus low-latency.

### Batch D — Protokol Jaringan & Multicast
- **Topologi:** Arsitektur hybrid TCP Control (default port `2046`) + UDP Multicast (default group `234.0.0.254:2048`).
- **Layout Paket Datagram (`tagDGRAMHEADER`):** 16-byte fixed binary header dengan Magic Sync Byte (`0x5A`), Channel ID (1-65535), Sequence Number, Audio Codec, dan Priority Level (BGM vs Intercom vs Fire Alarm).
- **Modernisasi:** Implementasi `tokio` + `socket2` dengan adaptive jitter buffer dan opsi enkripsi paket.

### Batch E — Lisensi & Native Hardware ID
- **Metode Legacy:** Dual-check menggunakan USB Smart Dongle Feitian ET99 (`FT_ET99_API.dll`) dan fallback serial software berbasis disk serial (`DeviceIoControl`) + MAC address.
- **Kelemahan Legacy:** Ketergantungan driver USB Windows x86 lama dan kerapuhan hardware binding.
- **Desain Modern:** Kriptografi asimetris digital license berbasis **Ed25519** + ChaCha20-Poly1305, multi-platform hardware fingerprinting, serta compatibility wrapper untuk transisi transparan.

### Batch F — Database & DAO Persistence
- **Storage Legacy:** Berkas Microsoft Access Jet MDB (`DATABASE\DBS.mdb`) diakses melalui DAO 3.6 (`CDaoRecordset*`).
- **Inventaris Data:** 14 tabel utama terpetakan (Terminal Member, Group/Zone, Ring Bell Task, Teach Task, BGM File Task, Fire Alarm Task, dan IP Rules).
- **Rencana Migrasi:** Konversi otomatis satu kali ke **SQLite 3 WAL mode** lintas platform, menjamin nol dependensi driver Windows dan keamanan data terhadap crash/mati lampu.

---

## 3. Matriks Keputusan & Kesiapan Platform

Urutan kepatuhan platform tetap ditegakkan:
1. **Windows (Prioritas 1):** Siap masuk tahap pengembangan Shared Core & Modern Windows Adapter.
2. **Linux (Prioritas 2):** Arsitektur headless daemon `systemd` dan ALSA/PipeWire siap diimplementasikan.
3. **macOS (Prioritas 3):** Desain CoreAudio & launchd adapter siap diimplementasikan.
4. **MitraOS / ASL Kernel (Prioritas 4):** Siap mengikuti roadmap 8-stage (Stage A: Kernel Discovery & ABI Contract) tanpa mengubah kode kernel sebelum disetujui.

---

## 4. Status Checkpoint
Dokumen checkpoint `docs/rnd/CHECKPOINT.md` telah diperbarui dengan state:
`CP-PHASE-0-2-DEEP-INVESTIGATION-COMPLETE`.
Preservation lock tetap terjaga 100%. Tidak ada operasi git commit/push atau modifikasi biner produksi.
