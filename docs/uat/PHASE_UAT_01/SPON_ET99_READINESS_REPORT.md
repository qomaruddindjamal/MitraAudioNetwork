# SPON FT_ET99 HARDWARE READINESS REPORT (TASK 4)
**Target:** USB Hardware Dongle Token Feitian ePass ET99  
**Pustaka Driver:** `C:\IPNBS\FT_ET99_API.dll` (69.632 bytes, x86 32-bit PE)  
**Status Evaluasi:** **BLOCKED (NO PHYSICAL TOKEN DETECTED)**  

---

## 1. PEMERIKSAAN PERANGKAT KERAS USB (PNP ENTITIES)

Pemeriksaan entitas Plug-and-Play USB pada sistem host Windows:

- Perangkat USB yang Terhubung Saat Ini:
  - `USB\VID_046D&PID_C34A` (Logitech USB Composite Device)
  - `USB\VID_0781&PID_5567` (SanDisk USB Mass Storage Device)
  - `PCI\VEN_8086&DEV_A36D` (Intel USB 3.1 Host Controller)
- **Status Feitian ET99:** **TIDAK DITEMUKAN (ABSENT)**.
  - Tidak ada hardware token dengan Vendor ID Feitian (`VID_096E` atau `VID_0471`) yang terpasang pada port USB fisik host.

## 2. STATUS PUSTAKA API `FT_ET99_API.DLL`
- File `FT_ET99_API.dll` berada di direktori `C:\IPNBS\` dan terverifikasi utuh dengan hash baseline `7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE`.
- Arsitektur DLL adalah 32-bit (x86), cocok untuk dipanggil oleh proses 32-bit `IPNBS_Server.exe`.
- Ekspor API yang terdokumentasi:
  - `et_FindToken`
  - `et_OpenToken`
  - `et_CloseToken`
  - `et_GenRandom`
  - `et_Read`
  - `et_Write`

## 3. KESIMPULAN READINESS
Karena token USB fisik tidak tercolok, pemanggilan fungsi `et_FindToken` saat startup server akan mengembalikan kode error deteksi token (token count = 0). Sesuai aturan direktif rekayasa, dilarang membuat emulator dongle palsu.
Status Hardware Mode: **BLOCKED (PHYSICAL DONGLE REQUIRED)**.
