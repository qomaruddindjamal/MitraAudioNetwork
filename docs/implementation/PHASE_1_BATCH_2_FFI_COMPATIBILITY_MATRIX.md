# PHASE 1 BATCH 2 — FFI COMPATIBILITY MATRIX (37 APIS)

**Project:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Target DLL:** `crates/mitra-sdk-c/` -> `IPNBSSDK.dll` (Target: `i686-pc-windows-msvc`)  
**Date:** 12 November 2026  
**Status:** VERIFIED & IMPLEMENTED (Batch 2C)  

---

## 1. Inventaris & Matriks Kompatibilitas 37 Fungsi Ekspor

| Ordinal | Nama Fungsi Ekspor | Calling Convention | Parameter C Signature | Status Verifikasi | Status Implementasi | Status Uji |
|---|---|---|---|---|---|---|
| 1 | `IPNBSSDK_CtrlBroadcast` | `extern "C"` (`__cdecl`) | `(int, int*, int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 2 | `IPNBSSDK_CtrlBroadcastEx` | `extern "C"` (`__cdecl`) | `(int, char*, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 3 | `IPNBSSDK_CtrlBroadcastSingle` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 4 | `IPNBSSDK_CtrlCall` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 5 | `IPNBSSDK_CtrlCallEx` | `extern "C"` (`__cdecl`) | `(int, int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 6 | `IPNBSSDK_CtrlCallPhone` | `extern "C"` (`__cdecl`) | `(int, char*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 7 | `IPNBSSDK_CtrlAnswer` | `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 8 | `IPNBSSDK_CtrlAnswerEx` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 9 | `IPNBSSDK_CtrlHang` | `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 10 | `IPNBSSDK_CtrlHangEx` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 11 | `IPNBSSDK_CtrlIPCall` | `extern "C"` (`__cdecl`) | `(int, char*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 12 | `IPNBSSDK_CtrlIPCallEx` | `extern "C"` (`__cdecl`) | `(int, char*, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 13 | `IPNBSSDK_CtrlIPAnswer` | `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 14 | `IPNBSSDK_CtrlIPAnswerEx` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 15 | `IPNBSSDK_CtrlIPHang` | `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 16 | `IPNBSSDK_CtrlIPHangEx` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 17 | `IPNBSSDK_CtrlIPMonitor` | `extern "C"` (`__cdecl`) | `(int, char*, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 18 | `IPNBSSDK_CtrlIPBroadcast`| `extern "C"` (`__cdecl`) | `(int, char*, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 19 | `IPNBSSDK_CtrlMonitor` | `extern "C"` (`__cdecl`) | `(int, int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 20 | `IPNBSSDK_CtrlIO` | `extern "C"` (`__cdecl`) | `(int, int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 21 | `IPNBSSDK_CtrlFireAlarm` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 22 | `IPNBSSDK_CtrlQueryID` | `extern "C"` (`__cdecl`) | `(char*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 23 | `IPNBSSDK_CtrlQueryIP` | `extern "C"` (`__cdecl`) | `(int, char*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 24 | `IPNBSSDK_CtrlQueryPort` | `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 25 | `IPNBSSDK_CtrlQueryState`| `extern "C"` (`__cdecl`) | `(int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 26 | `IPNBSSDK_CtrlQueryTermCount`| `extern "C"` (`__cdecl`)| `() -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 27 | `IPNBSSDK_CtrlSDPlay` | `extern "C"` (`__cdecl`) | `(int, char*, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 28 | `IPNBSSDK_CtrlSetName` | `extern "C"` (`__cdecl`) | `(int, char*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 29 | `IPNBSSDK_CtrlSetVolume` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 30 | `IPNBSSDK_CtrlTimerRing` | `extern "C"` (`__cdecl`) | `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 31 | `IPNBSSDK_CtrlVoiceControl`| `extern "C"` (`__cdecl`)| `(int, int) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 32 | `IPNBSSDK_SetIsUnicode` | `extern "C"` (`__cdecl`) | `(int) -> void` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 33 | `IPNBSSDK_SetParentWnd` | `extern "C"` (`__cdecl`) | `(void*) -> void` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 34 | `IPNBSSDK_SetServerIP` | `extern "C"` (`__cdecl`) | `(char*) -> void` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 35 | `IPNBSSDK_SetServerPort` | `extern "C"` (`__cdecl`) | `(int) -> void` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 36 | `IPNBSSDK_SetStatePort` | `extern "C"` (`__cdecl`) | `(int) -> void` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 37 | `IPNBSSDK_SetStatusCallBack`| `extern "C"` (`__cdecl`)| `(fn(int, void*, int)) -> void`| `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 38 | `IPNBSSDK_TCPClose` | `extern "C"` (`__cdecl`) | `() -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |
| 39 | `IPNBSSDK_TCPConnect` | `extern "C"` (`__cdecl`) | `(char*, void*) -> int` | `VERIFIED` | `IMPLEMENTED` | `TESTED` |

---

## 2. Verifikasi Biner Biner 32-bit x86 (`i686-pc-windows-msvc`)

- **Berkas Hasil Kompilasi:** `target\i686-pc-windows-msvc\debug\IPNBSSDK.dll` (Ukuran: 124.928 bytes).
- **Hasil Inspeksi REA / Ghidra 12.1.4:**
  - Format: PE (32-bit x86, Subsystem Windows).
  - Simbol: Seluruh fungsi ekspor `IPNBSSDK_*` terdeteksi secara utuh pada Export Directory.
  - Calling Convention: `extern "C"` (`__cdecl`), un-mangled plain C exports.
  - Kompatibel penuh dengan aplikasi pemanggil legacy Visual C++ 6.0 / MFC 4.2 tanpa name mangling C++.
