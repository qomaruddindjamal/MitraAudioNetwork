# PHASE 0.2 — LEGACY FUNCTION INVENTORY & SYMBOL MAPPING

**Target Binaries:** `IPNBS_Server.exe`, `IPNBSSDK.dll`, `FT_ET99_API.dll`  
**Toolchain Reference:** Microsoft Visual C++ 6.0 / MFC 4.2  
**Source Evidence:** Ghidra 12.1.4 Decompilation + `IPNBS_Server.map` + Dependency Analysis

---

## 1. Modul Server Inti (`IPNBS_Server.exe`)

| Module / Class | Function / Symbol | Virtual Address | Peran & Perilaku Teknis |
|---|---|---|---|
| `CDLSApp` | `InitInstance` | `0x00411CB0` | Entry point MFC: Single instance mutex check, read INI config, open Jet MDB, init Winsock. |
| `CDLSApp` | `RunAlready` | `0x00411C80` | Mutex check `"DLS tag of run alone2"` via `OpenMutexA`/`CreateMutexA`. |
| `CDLSApp` | `ExitInstance` | `0x00412210` | Shutdown sequence: release sockets, close database, free threads, delete mutex. |
| `CDLSDlg` | `OnInitDialog` | `0x0040D1F0` | Setup dialog utama: bind controls, start status timers, populate terminal list view. |
| `CACMInCtrl` | `Open` | `0x00401530` | Inisialisasi WinMM recording (`waveInOpen`), format PCM 16-bit. |
| `CACMInCtrl` | `NewHeader` | `0x004016A0` | Alokasi 4 buah `WAVEHDR` buffer cincin untuk audio capture. |
| `CACMInCtrl` | `Close` | `0x00401750` | Reset dan pelepasan handle `waveInClose`. |
| `CACMInCtrl` | `InitEncoder` | `0x00401850` | Setup encoder audio (PCM, IMA-ADPCM, atau LAME MP3). |
| `CACMInCtrl` | `OpenRecord` | `0x00401C40` | Memulai capture audio hardware soundcard. |
| `CACMInCtrl` | `CloseRecord` | `0x00401CC0` | Menjeda capture audio hardware. |
| `CChannel` | `StartMulticast` | `0x00404A50` | Bergabung ke IP Multicast group `234.0.0.254` dan mulai stream audio. |
| `CChannel` | `StopMulticast` | `0x00404AA0` | Meninggalkan Multicast group (`IP_DROP_MEMBERSHIP`). |
| `CChannel` | `SendTo` | `0x00404C50` | Mengirimkan paket datagram audio UDP dengan header `tagDGRAMHEADER`. |
| `CChannel` | `SendToWanMemberList` | `0x00404DC0` | Fallback unicast UDP jika multicast dibatasi oleh router WAN. |
| `CIoCompletionPort` | `WorkerThread` | `0x00435120` | Loop IOCP `GetQueuedCompletionStatus` untuk penerimaan koneksi TCP. |
| `CEncryption` | `GetSerialNumber` | `0x00437580` | Generate machine code dari disk serial `DeviceIoControl` & MAC address. |
| `CEncryption` | `IsValidSerialNumber`| `0x00437C60` | Validasi hash serial key lisensi software. |
| `CEncryption` | `TrailExpired` | `0x00438310` | Pengecekan masa berlaku trial server. |
| `CEncryption` | `DecodeLicense` | `0x00438410` | Ekstraksi tanggal expired dan limit jumlah terminal dari lisensi. |
| `CDaoManage` | `OpenDatabase` | `0x00409E20` | Buka file `DATABASE\DBS.mdb` melalui engine Jet/DAO. |

---

## 2. Modul SDK API (`IPNBSSDK.dll`)

| Exported API | Calling Convention | Ordinal | Peran Teknis & Mapping Rust FFI |
|---|---|---|---|
| `IPNBSSDK_TCPConnect` | `__cdecl` | 37 | Membuka TCP connection ke Server, spawn worker thread. |
| `IPNBSSDK_TCPClose` | `__cdecl` | 33 | Menutup koneksi dan destroy background threads. |
| `IPNBSSDK_SetServerIP` | `__cdecl` | 35 | Konfigurasi alamat IP server tujuan. |
| `IPNBSSDK_SetServerPort`| `__cdecl` | 36 | Konfigurasi port TCP server (default: 2046). |
| `IPNBSSDK_SetStatusCallBack` | `__cdecl` | 32 | Pendaftaran function pointer callback event. |
| `IPNBSSDK_SetParentWnd` | `__cdecl` | 31 | Pendaftaran target `HWND` untuk pesan notifikasi. |
| `IPNBSSDK_CtrlBroadcast` | `__cdecl` | 1 | Trigger siaran audio ke daftar terminal. |
| `IPNBSSDK_CtrlBroadcastEx` | `__cdecl` | 2 | Trigger siaran audio ke grup/zona terminal. |
| `IPNBSSDK_CtrlCall` | `__cdecl` | 4 | Inisiasi panggilan interkom 2 arah. |
| `IPNBSSDK_CtrlAnswer` | `__cdecl` | 7 | Menerima panggilan interkom masuk. |
| `IPNBSSDK_CtrlHang` | `__cdecl` | 9 | Memutus hubungan interkom. |
| `IPNBSSDK_CtrlFireAlarm` | `__cdecl` | 21 | Trigger alarm kebakaran darurat ke seluruh zona. |
| `IPNBSSDK_CtrlQueryState` | `__cdecl` | 25 | Query status real-time terminal (Idle, Call, Broadcast). |
| `IPNBSSDK_CtrlQueryTermCount` | `__cdecl` | 26 | Mengambil jumlah terminal aktif terdaftar. |

---

## 3. Modul Hardware Dongle (`FT_ET99_API.dll`)

| Exported Symbol | Parameter | Return Code | Peran Teknis |
|---|---|---|---|
| `_et_FindToken@8` | `(BYTE *pPID, int *pCount)` | `0` (Success) / Non-zero (Error) | Mencari token hardware ET99 yang tercolok pada port USB. |
| `_et_OpenToken@12` | `(HANDLE *phToken, BYTE *pPID, int nIndex)` | `0` (Success) | Membuka koneksi handle komunikasi ke chip token. |
| `_et_Read@20` | `(HANDLE hToken, WORD wOffset, BYTE *pBuf, WORD wLen)` | `0` (Success) | Membaca data kapasitas lisensi dari EEPROM token. |
| `_et_CloseToken@4` | `(HANDLE hToken)` | `0` (Success) | Menutup handle USB token. |
