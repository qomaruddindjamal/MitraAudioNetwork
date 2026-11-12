# SPON SERVER & STATION OPERATIONAL READINESS REPORT
**Execution Date:** 12 November 2026  
**Phase:** Phase UAT — Audit F  
**Target:** `C:\IPNBS\IPNBS_Server.exe` & `C:\IPNBS\IPNBS_Station.exe`  

---

## 1. PEMERIKSAAN KESIAPAN OPERASIONAL SERVER & STATION

1. **Integritas File Biner:**
   - `IPNBS_Server.exe` (1.433.600 bytes) $\rightarrow$ Cocok dengan installer master resmi `SetupIPNBS_V3.3.0.exe` (SHA-256 match).
   - `IPNBS_Station.exe` (3.006.464 bytes) $\rightarrow$ Cocok dengan installer master resmi `SetupIPNBS_V3.3.0.exe` (SHA-256 match).
2. **Kesiapan Jaringan & Port Jaringan:**
   - Konfigurasi `IPNBS_Server.ini` mendefinisikan port kontrol `nTerminalPortRcv=2046` (TCP Control) dan port siaran `nTerminalPortSnd=2048` (UDP Multicast Audio).
   - Pengujian siklus hidup socket port 2046 pada test suite Rust (`tcp_2046_tests.rs`) membuktikan port bebas konflik dan dapat diikat (bindable).
3. **Ketergantungan Basis Data:**
   - `DATABASE\DBS.mdb` terverifikasi utuh dengan format Microsoft Access Jet 4.0.
   - Pustaka enkripsi database `DBSEnc.dll`, `DBSEnc1.dll`, `DBSEnc.dat` tersedia di direktori kerja.
4. **Hambatan Eksekusi Penuh (Operational Blocker):**
   - Startup penuh `IPNBS_Server.exe` dalam mode penyiaran komersial memerlukan deteksi Dongle ET99 fisik (`FT_ET99_API.dll`) atau Serial Number 29 karakter yang valid di `IPNBS_Server.ini`.
   - Tanpa salah satu dari keduanya, server masuk ke status unactivated (Error 0x9C42).
   - Status Kesiapan Operasional: **READY_FOR_DEPLOYMENT / BLOCKED ON LICENSE HARDWARE**.
