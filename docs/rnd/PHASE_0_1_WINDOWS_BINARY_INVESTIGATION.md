# WINDOWS LEGACY BINARY INVESTIGATION — PHASE 0.1
## MITRA AUDIO NETWORK SYSTEM

**Tanggal:** 2026-11-11  
**Target:** `IPNBS_Server.exe`, `IPNBS_Station.exe`, `IPNBSSDK.dll`, `FT_ET99_API.dll`  
**Metode:** PE Header Analysis, Map File (`IPNBS_Server.map`) Cross-Reference, Import Table Mapping, Symbolic Reconstruction  
**Status Integritas:** Working copy di `c:\IPNBS\analysis_work\` terverifikasi SHA-256 identik dengan baseline.  

---

### 1. Arsitektur Biner & Compiler Runtime

- **Target Machine:** x86 32-bit (`IMAGE_FILE_MACHINE_I386`), Subsystem Windows GUI.
- **Compiler / Linker:** Microsoft Visual C++ 6.0 / MFC 4.2 (ditandai dengan impor `mfc42.dll`, struktur CRT `$XCA`..`$XCZ`, dan timestamp biner Agustus 2014).
- **Preferred Base Address:** `0x00400000`.
- **Calling Convention:** `__thiscall` untuk C++ member functions (misal `?Open@CACMInCtrl@@AAEHFJGFKF@Z`), `__stdcall` untuk Win32 callbacks (`?waveInProc@@YGXPAUHWAVEIN__@@IKKK@Z`).

---

### 2. Audio Subsystem (Capture, Playback, and Codec)

1. **Audio Capture Engine (`CACMInCtrl`):**
   - Menggunakan Windows Multimedia API (`winmm.dll`).
   - Callback handler: `?waveInProc@@YGXPAUHWAVEIN__@@IKKK@Z` menerima buffer audio mentah dari soundcard/mikrofon.
   - Manajemen buffer: `NewHeader` mengalokasikan struct `WAVEHDR`, `waveInPrepareHeader`, dan `waveInAddBuffer`.
   - Modul ini mengontrol dynamic encoding ke dalam `CNetCore` dispatcher melalui `InitEncoder`.
2. **Audio File Processing (`CWAVFileCtrl` & `IPNBS_MP3Tools.exe` / `lame_enc.dll`):**
   - Format didukung: Standar Linear PCM (WAV), Microsoft ADPCM, dan MP3 streaming via modul `lame_enc.dll` dan `NCTAudioFile2.dll`.

---

### 3. Network Transport & IOCP Architecture

1. **Socket Engine:** Menggunakan Windows Sockets 2 (`Ws2_32.dll`) dengan pola asynchronous I/O Completion Ports (`CIoCompletionPort` dan `GetQueuedCompletionStatus`).
2. **Komunikasi Multicast:**
   - Default Multicast Group: `234.0.0.254`.
   - Port Penerimaan Terminal: `2046` (`nTerminalPortRcv`).
   - Port Pengiriman Terminal: `2048` (`nTerminalPortSnd`).
   - Transmisi data dilakukan menggunakan `WSASendTo` dan `WSARecvFrom`.
3. **Analisis Insiden Error 995:**
   - Log `IPNBS_Server202611.log` mencatat: `Net error code: 995` (`WSA_OPERATION_ABORTED`).
   - *Akar Masalah:* Dalam arsitektur IOCP Win32 legacy, ketika socket ditutup atau thread worker dibatalkan saat I/O overlapping masih berstatus pending, OS memicu kode kesalahan 995. Pada arsitektur baru, hal ini diselesaikan menggunakan Tokio asynchronous event loop dengan gracefull cancellation token.

---

### 4. Database & Persistence Subsystem

- Kelas: `CDaoManage` dan `CDaoRecordset*` (`CDaoRecordsetMember`, `CDaoRecordsetBroadcastTask`, `CDaoRecordsetProgram`).
- Engine: Microsoft Jet Engine / DAO (Data Access Objects) mengakses database Microsoft Access MDB (`DATABASE\DBS.mdb`).
- Kelemahan Legacy: Berkas MDB mengunci berkas secara eksklusif dan tidak kompatibel secara native di luar sistem Windows 32-bit.

---

### 5. Hardware Dongle & Licensing Subsystem

1. **USB Hardware Dongle:**
   - Memanggil `FT_ET99_API.dll` fungsi `_et_FindToken@8`.
   - Tipe Hardware: Feitian ePass ET99 USB Dongle (HID-based smart token).
   - Fungsi: Menjadi jangkar keamanan perangkat keras fisik untuk memastikan aplikasi server berjalan pada server resmi berlisensi vendor.
2. **Native ID & Serial Number (`CEncryption`):**
   - Menggunakan panggilan `DeviceIoControl` (kemungkinan `IOCTL_STORAGE_QUERY_PROPERTY` atau `SMART_RCV_DRIVE_DATA`) untuk membaca nomor seri fisik Hard Disk atau UUID Motherboard.
   - Mengonversi ID perangkat keras menjadi string Serial Number alfanumerik (seperti `64E8A-A8F0C-2B1EC-BC7DC-0B05`).
   - Memiliki modul `EncodeLicense` dan `DecodeLicense` untuk menentukan tanggal kedaluwarsa (`GetLimitDay`, `TrailExpired`).
