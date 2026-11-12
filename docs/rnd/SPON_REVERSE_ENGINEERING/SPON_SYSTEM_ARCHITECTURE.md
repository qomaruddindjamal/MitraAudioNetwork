# SPON SYSTEM ARCHITECTURE & INTER-COMPONENT RELATIONSHIP
**Target:** IP Network Broadcasting System (IPNBS V3.3.0)  
**Environment:** `C:\IPNBS`  
**Analysis Date:** 12 November 2026  
**Classification:** READ-ONLY FORENSIC REVERSE ENGINEERING  

---

## 1. IKHTISAR ARSITEKTUR SISTEM SPON

Sistem SPON (IPNBS V3.3.0) dibangun di atas arsitektur Client-Server berbasis Win32 C/C++ (MSVC 6.0/Borland runtime era Windows 2000/XP/7), yang terdiri dari beberapa entitas utama:

```
[ IPNBS_Server.exe (Core Broadcast Server) ]
       |-- RVA 0x00418F62: et_FindToken() ----> [ FT_ET99_API.dll (Hardware Dongle) ]
       |-- RVA 0x00418F8D: GetModuleFileNameA()
       |-- RVA 0x004190DC: Read SerialNumber --> [ IPNBS_Server.ini ]
       |-- RVA 0x00439360: SN Verification
       |-- RVA 0x0043DB00: MD5 Engine
       |-- Jet OLEDB 4.0 Provider -----------> [ DATABASE\DBS.mdb (Access DB) ]
       |-- TCP Port 2046 (Control Link) -----> [ IPNBS_Station.exe (Client Console) ]
       |-- UDP Port 2048 (Multicast Audio) --> [ IP Audio Terminals (8000DA / 8000RM) ]
       |-- C-ABI Function Exports -----------> [ IPNBSSDK.dll (Third-party Integration) ]
```

## 2. PERAN MASING-MASING KOMPONEN

1. **`IPNBS_Server.exe` (1.433.600 bytes, x86 32-bit GUI):**
   - Mengelola koneksi jaringan terminal IP, penjadwalan bel otomatis, perutean audio, dan penegakan lisensi sistem siaran.
   - Bertindak sebagai TCP Server pada port 2046 dan UDP Multicast Transmitter pada port 2048.
2. **`IPNBS_Station.exe` (3.006.464 bytes, x86 32-bit GUI):**
   - Antarmuka stasiun kerja operator untuk paging manual, interkom mikrofon, dan pemantauan terminal siaran.
3. **`IPNBSSDK.dll` (40.960 bytes, x86 32-bit Dynamic Link Library):**
   - Menyediakan 37 fungsi ekspor C-ABI untuk integrasi sistem pihak ketiga (`IPNBSSDK_TCPConnect`, `IPNBSSDK_CtrlBroadcast`, dll.).
4. **`FT_ET99_API.dll` (69.632 bytes, x86 32-bit DLL):**
   - Driver pustaka API untuk USB Smart Card Token Feitian ePass ET99.
5. **`DATABASE\DBS.mdb` (Microsoft Access Database Jet 4.0 format):**
   - Menyimpan konfigurasi terminal IP, zona, playlist lagu, riwayat panggilan, dan akun pengguna.
