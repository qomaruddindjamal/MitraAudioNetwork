# PHASE 0.3 — WINDOWS COMPATIBILITY SPECIFICATION & FFI ABI CONTRACT

**Project:** Mitra Audio Network System  
**Document Code:** `SPEC-WIN-COMPAT-002`  
**Status:** VALIDATED (Phase 0.3 Blueprint)  
**Target:** 32-bit x86 & 64-bit x64 Binary Compatibility  

---

## 1. Analisis ABI & Calling Convention Legacy

Berdasarkan investigasi Phase 0.2 pada `IPNBSSDK.dll`:
- **Arsitektur Host:** 32-bit x86 PE (`IMAGE_FILE_MACHINE_I386` - `0x014C`).
- **Calling Convention Ekspor:** `__cdecl` (Caller membersihkan stack).
- **Dekorasi Nama:** Plain C export (Unmangled, tanpa prefiks `@` atau underscore pada PE Export Directory).
- **Export Table:** 37 fungsi ekspor terdaftar secara publik dari ordinal 1 s/d 37.
- **Tipe Data Parameter:** Pointer C dasar (`char*`, `int*`, `void*`), integer primitif (`int`), dan handle Win32 (`HWND`).

---

## 2. Matriks Kompatibilitas 37 Exported APIs (`IPNBSSDK.dll`)

| Ordinal | Nama Fungsi Ekspor | Calling Convention | Parameter Signature | Mapping Rust FFI | Status Kompatibilitas |
|---|---|---|---|---|---|
| 1 | `IPNBSSDK_CtrlBroadcast` | `__cdecl` | `(int, int*, int, int)` | `unsafe extern "C" fn(c_int, *const c_int, c_int, c_int) -> c_int` | `VERIFIED` |
| 2 | `IPNBSSDK_CtrlBroadcastEx` | `__cdecl` | `(int, char*, int)` | `unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int` | `VERIFIED` |
| 3 | `IPNBSSDK_CtrlBroadcastSingle` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 4 | `IPNBSSDK_CtrlCall` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 5 | `IPNBSSDK_CtrlCallEx` | `__cdecl` | `(int, int, int)` | `unsafe extern "C" fn(c_int, c_int, c_int) -> c_int` | `VERIFIED` |
| 6 | `IPNBSSDK_CtrlCallPhone` | `__cdecl` | `(int, char*)` | `unsafe extern "C" fn(c_int, *const c_char) -> c_int` | `VERIFIED` |
| 7 | `IPNBSSDK_CtrlAnswer` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 8 | `IPNBSSDK_CtrlAnswerEx` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 9 | `IPNBSSDK_CtrlHang` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 10 | `IPNBSSDK_CtrlHangEx` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 11 | `IPNBSSDK_CtrlIPCall` | `__cdecl` | `(int, char*)` | `unsafe extern "C" fn(c_int, *const c_char) -> c_int` | `VERIFIED` |
| 12 | `IPNBSSDK_CtrlIPCallEx` | `__cdecl` | `(int, char*, int)`| `unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int` | `VERIFIED` |
| 13 | `IPNBSSDK_CtrlIPAnswer` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 14 | `IPNBSSDK_CtrlIPAnswerEx` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 15 | `IPNBSSDK_CtrlIPHang` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 16 | `IPNBSSDK_CtrlIPHangEx` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 17 | `IPNBSSDK_CtrlIPMonitor` | `__cdecl` | `(int, char*, int)`| `unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int` | `VERIFIED` |
| 18 | `IPNBSSDK_CtrlIPBroadcast`| `__cdecl` | `(int, char*, int)`| `unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int` | `VERIFIED` |
| 19 | `IPNBSSDK_CtrlMonitor` | `__cdecl` | `(int, int, int)` | `unsafe extern "C" fn(c_int, c_int, c_int) -> c_int` | `VERIFIED` |
| 20 | `IPNBSSDK_CtrlIO` | `__cdecl` | `(int, int, int)` | `unsafe extern "C" fn(c_int, c_int, c_int) -> c_int` | `VERIFIED` |
| 21 | `IPNBSSDK_CtrlFireAlarm` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 22 | `IPNBSSDK_CtrlQueryID` | `__cdecl` | `(char*)` | `unsafe extern "C" fn(*const c_char) -> c_int` | `VERIFIED` |
| 23 | `IPNBSSDK_CtrlQueryIP` | `__cdecl` | `(int, char*)` | `unsafe extern "C" fn(c_int, *mut c_char) -> c_int` | `VERIFIED` |
| 24 | `IPNBSSDK_CtrlQueryPort` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 25 | `IPNBSSDK_CtrlQueryState`| `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int) -> c_int` | `VERIFIED` |
| 26 | `IPNBSSDK_CtrlQueryTermCount`| `__cdecl`| `()` | `unsafe extern "C" fn() -> c_int` | `VERIFIED` |
| 27 | `IPNBSSDK_CtrlSDPlay` | `__cdecl` | `(int, char*, int)`| `unsafe extern "C" fn(c_int, *const c_char, c_int) -> c_int` | `VERIFIED` |
| 28 | `IPNBSSDK_CtrlSetName` | `__cdecl` | `(int, char*)` | `unsafe extern "C" fn(c_int, *const c_char) -> c_int` | `VERIFIED` |
| 29 | `IPNBSSDK_CtrlSetVolume` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 30 | `IPNBSSDK_CtrlTimerRing` | `__cdecl` | `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 31 | `IPNBSSDK_CtrlVoiceControl`| `__cdecl`| `(int, int)` | `unsafe extern "C" fn(c_int, c_int) -> c_int` | `VERIFIED` |
| 32 | `IPNBSSDK_SetIsUnicode` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int)` | `VERIFIED` |
| 33 | `IPNBSSDK_SetParentWnd` | `__cdecl` | `(HWND)` | `unsafe extern "C" fn(*mut c_void)` | `VERIFIED` |
| 34 | `IPNBSSDK_SetServerIP` | `__cdecl` | `(char*)` | `unsafe extern "C" fn(*const c_char)` | `VERIFIED` |
| 35 | `IPNBSSDK_SetServerPort` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int)` | `VERIFIED` |
| 36 | `IPNBSSDK_SetStatePort` | `__cdecl` | `(int)` | `unsafe extern "C" fn(c_int)` | `VERIFIED` |
| 37 | `IPNBSSDK_SetStatusCallBack`| `__cdecl`| `(StatusCallbackFn)`| `unsafe extern "C" fn(Option<extern "C" fn(c_int, *mut c_void, c_int)>)` | `VERIFIED` |
| 38 | `IPNBSSDK_TCPClose` | `__cdecl` | `()` | `unsafe extern "C" fn() -> c_int` | `VERIFIED` |
| 39 | `IPNBSSDK_TCPConnect` | `__cdecl` | `(char*, void*)` | `unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int` | `VERIFIED` |

---

## 3. Strategi Drop-in Binary Replacement

Untuk menjamin aplikasi legacy (seperti `IPNBS_Station.exe`) dapat langsung berjalan menggunakan DLL Rust baru tanpa kompilasi ulang:

1. **Target Kompilasi Rust:** Menggunakan target `i686-pc-windows-msvc` (atau `i686-pc-windows-gnu`) untuk menghasilkan 32-bit x86 PE DLL murni.
2. **Definisi Module Definition (`.def`) File:**
   Untuk mengunci urutan ordinal eksak 1 s/d 37 dan mencegah "mangling" nama ekspor oleh linker MSVC:
   ```def
   LIBRARY IPNBSSDK.dll
   EXPORTS
       IPNBSSDK_CtrlBroadcast        @1
       IPNBSSDK_CtrlBroadcastEx      @2
       ...
       IPNBSSDK_TCPClose             @38
       IPNBSSDK_TCPConnect           @39
   ```
3. **Safety Boundary pada Callback & Threading:**
   - Callback pointer `IPNBSSDK_SetStatusCallBack` disimpan secara atomik (`AtomicPtr`).
   - Eksekusi callback dari thread Rust async Tokio dialihkan melalui event dispatcher yang aman terhadap re-entrancy UI thread Windows.
   - String buffer yang dikembalikan pada `IPNBSSDK_CtrlQueryIP` disalin dengan batas aman (`strncpy_s`) untuk mencegah buffer overflow pada buffer aplikasi caller lama.
