# PHASE 0.2 — TECHNICAL EVIDENCE MATRIX

**Investigation Target:** Legacy Binaries & Subsystems Validation  
**Evidence Collection Date:** 11 November 2026  
**Methodology:** Independent Static Reverse Engineering, Decompilation, Symbol Map Cross-Referencing  

---

## 1. Matrix Bukti Teknis

| ID | Klaim / Komponen Sistem | Metode Pembuktian | Bukti Nyata Terverifikasi (Evidence Source) | Status Validasi |
|---|---|---|---|---|
| **EV-01** | Biner 32-bit x86 Visual C++ 6.0 / MFC 4.2 | REA Header Analysis & PE Inspection | `IPNBS_Server.exe` machine type `0x014c` (i386), subsistem Windows GUI, link ke MFC42.DLL / MSVCRT.DLL. | **TERBUKTI PENUH** |
| **EV-02** | Single Instance Mutex `"DLS tag of run alone2"` | Decompilasi Ghidra `CDLSApp::RunAlready` | Address `0x00411C80`: string literal `"DLS tag of run alone2"` digunakan pada panggilan `CreateMutexA`. | **TERBUKTI PENUH** |
| **EV-03** | Database Microsoft Jet MDB / DAO 3.6 | Map file & Import Analysis | Simbol `CDaoRecordset*` pada baris 217-256 `IPNBS_Server.map`, impor ke `DAO360.DLL`. | **TERBUKTI PENUH** |
| **EV-04** | Port Default & Multicast IP Audio | Ghidra string search & Map analysis | String `234.0.0.254`, port `2046` (control), port `2048` (audio multicast) ditemukan di `CChannel`. | **TERBUKTI PENUH** |
| **EV-05** | Error IOCP 995 (`WSA_OPERATION_ABORTED`) | Map analysis & Ghidra `CIoCompletionPort` | Terjadi saat pembatalan overlapped I/O soket ketika client disconnect; bukan kerusakan memori. | **TERBUKTI PENUH** |
| **EV-06** | Struktur API `IPNBSSDK.dll` (37 Exports) | REA Procedure Extract & Ghidra Decompile | 37 fungsi C diekspor; dekompilasi `IPNBSSDK_TCPConnect` (`0x10004420`) memanggil worker thread `0x10002850`. | **TERBUKTI PENUH** |
| **EV-07** | Subkoneksi Lisensi Feitian ET99 Dongle | Map file & PE Import `FT_ET99_API.dll` | Simbol impor `_et_FindToken@8`, `_et_OpenToken@12`, `_et_Read@20` terikat di `IPNBS_Server.exe`. | **TERBUKTI PENUH** |
| **EV-08** | Audio Capture WinMM `waveInOpen` | Ghidra Decompile `CACMInCtrl::Open` | Address `0x00401530`: inisialisasi `WAVEFORMATEX` 16-bit PCM, 4 ring buffers `WAVEHDR`. | **TERBUKTI PENUH** |
| **EV-09** | ASL Kernel Platform Gap | Direct Source Audit `C:\asl` | Bare-metal Rust x86_64 microkernel tanpa audio subsystem dan socket TCP/IP user-space. | **TERBUKTI PENUH** |

---

## 2. Integritas Binary Hash Log

Seluruh pengujian dilakukan pada salinan biner di `C:\IPNBS\analysis_work` dengan verifikasi SHA-256 sebelum dan sesudah investigasi:

```text
7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9  IPNBS_Server.exe  (MATCH)
61286ABAD5EFCA2AA7721415D870DF2F60EEC0FB9CF3B2D1ADE6A2F74904C0E2  IPNBS_Station.exe (MATCH)
FBDC81009FF4306504C6374F6C3D18A54663EF699900A93C8A7489B30B904211  IPNBSSDK.dll      (MATCH)
7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE  FT_ET99_API.dll   (MATCH)
```
Status: **Preservation Lock 100% Terjaga. Tidak ada modifikasi pada file biner asli.**
