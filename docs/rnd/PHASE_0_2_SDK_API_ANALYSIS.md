# PHASE 0.2 — BATCH B: IPNBSSDK.DLL API SPECIFICATION & CALLING CONVENTIONS

**Target Binary:** `IPNBSSDK.dll`  
**Image Base:** `0x10000000` | **Calling Convention:** `__cdecl` (Standard C Export)  
**Integrity Hash (SHA-256):** `FBDC81009FF4306504C6374F6C3D18A54663EF699900A93C8A7489B30B904211`  
**Total Exports:** 37 APIs  
**Analysis Provider:** REA 6.8.0 + Ghidra 12.1.4 PUBLIC

---

## 1. Complete API Catalog (37 Exported Functions)

Seluruh fungsi diekspor sebagai fungsi C murni (`__cdecl`), membungkus interaksi internal C++ singleton dispatcher (`DAT_100078a8`).

### 1.1 Inisialisasi & Koneksi Sesi

| Ordinal | Function Signature | Deskripsi Perilaku Decompiled |
|---|---|---|
| 35 | `void IPNBSSDK_SetServerIP(char *szIP)` | Menyimpan IP target server ke konfigurasi internal SDK. |
| 36 | `void IPNBSSDK_SetServerPort(int nPort)` | Menentukan port TCP komunikasi control server (default: 2046/2048). |
| 34 | `void IPNBSSDK_SetStatePort(int nPort)` | Menentukan port status heartbeat/monitoring. |
| 37 | `int IPNBSSDK_TCPConnect(char *szIP, void *pParam)` | Melakukan koneksi socket TCP ke server, membuat thread background listener (`CreateThread` @ `0x10002850`), dan memanggil callback status koneksi. |
| 33 | `int IPNBSSDK_TCPClose()` | Menghentikan background thread, menutup TCP stream, dan membersihkan buffer state. |
| 32 | `void IPNBSSDK_SetStatusCallBack(void (*pfn)(int nType, void *pData, int nLen))` | Mendaftarkan fungsi pointer callback untuk menerima status terminal, perubahan channel, dan stream audio. |
| 31 | `void IPNBSSDK_SetParentWnd(HWND hWnd)` | Mengirimkan window handle penerima event Windows messages (`WM_USER+...`). |
| 30 | `void IPNBSSDK_SetIsUnicode(int bUnicode)` | Flag encoding teks (ASCII vs Unicode UTF-16LE). |

### 1.2 Kontrol Broadcast & Audio Streaming

| Ordinal | Function Signature | Deskripsi Perilaku |
|---|---|---|
| 1 | `int IPNBSSDK_CtrlBroadcast(int nChannel, int *pTermList, int nTermCount, int bStart)` | Memulai atau menghentikan audio broadcast pada channel tertentu ke daftar terminal. Mengirimkan command packet UDP/TCP ke server. |
| 2 | `int IPNBSSDK_CtrlBroadcastEx(int nChannel, char *szGroupList, int bStart)` | Memulai broadcast berdasarkan group ID string. |
| 3 | `int IPNBSSDK_CtrlBroadcastSingle(int nTermID, int bStart)` | Broadcast terarah ke single terminal ID. |
| 26 | `int IPNBSSDK_CtrlSDPlay(int nTermID, char *szFileName, int bStart)` | Memerintahkan terminal untuk memutar audio lokal dari memory SD card. |
| 27 | `int IPNBSSDK_CtrlSetName(int nTermID, char *szName)` | Mengubah display name terminal secara remote. |
| 28 | `int IPNBSSDK_CtrlSetVolume(int nTermID, int nVolume)` | Mengatur volume output hardware terminal (0 - 100). |
| 29 | `int IPNBSSDK_CtrlTimerRing(int nTermID, int nRingID)` | Memicu nada bel terprogram pada terminal target. |
| 30 | `int IPNBSSDK_CtrlVoiceControl(int nTermID, int nLevel)` | Mengatur ambang batas VOX (Voice Activated Exchange). |

### 1.3 Intercom & IP Call System

| Ordinal | Function Signature | Deskripsi Perilaku |
|---|---|---|
| 4 | `int IPNBSSDK_CtrlCall(int nSrcTermID, int nDstTermID)` | Memulai panggilan audio dua arah antar terminal. |
| 5 | `int IPNBSSDK_CtrlCallEx(int nSrcTermID, int nDstTermID, int nPriority)` | Memulai panggilan dengan level prioritas (emergency override). |
| 6 | `int IPNBSSDK_CtrlCallPhone(int nTermID, char *szPhoneNum)` | Integrasi VoIP/PSTN trunk dialing. |
| 7 | `int IPNBSSDK_CtrlAnswer(int nTermID)` | Menjawab panggilan masuk dari terminal lain. |
| 8 | `int IPNBSSDK_CtrlAnswerEx(int nTermID, int nOption)` | Menjawab panggilan dengan mode mute/speaker. |
| 9 | `int IPNBSSDK_CtrlHang(int nTermID)` | Mengakhiri panggilan interkom aktif. |
| 10 | `int IPNBSSDK_CtrlHangEx(int nTermID, int nReason)` | Mengakhiri panggilan dengan kode terminasi/busy. |
| 11-18 | `IPNBSSDK_CtrlIPCall*` & `IPNBSSDK_CtrlIPAnswer*` | Variasi pemanggilan direct IP addressing tanpa lookup ID server. |

### 1.4 Monitoring, GPIO & Query Hardware

| Ordinal | Function Signature | Deskripsi Perilaku |
|---|---|---|
| 19 | `int IPNBSSDK_CtrlMonitor(int nSrcTermID, int nDstTermID, int bStart)` | Membuka silent audio monitor channel mikrofon remote terminal. |
| 20 | `int IPNBSSDK_CtrlIO(int nTermID, int nPort, int nState)` | Mengontrol pin GPIO / relay output pada terminal (e.g., buka kunci gerbang). |
| 21 | `int IPNBSSDK_CtrlFireAlarm(int nZone, int bTrigger)` | Menghidupkan mode emergency broadcast kebakaran ke seluruh zona. |
| 22 | `int IPNBSSDK_CtrlQueryID(char *szIP)` | Query terminal ID berdasarkan IP address perangkat. |
| 23 | `int IPNBSSDK_CtrlQueryIP(int nTermID, char *outIP)` | Query IP address terminal berdasarkan terminal ID. |
| 24 | `int IPNBSSDK_CtrlQueryPort(int nTermID)` | Query listen port terminal target. |
| 25 | `int IPNBSSDK_CtrlQueryState(int nTermID)` | Query status perangkat: Offline (0), Idle (1), Ringing (2), Talking (3), Broadcasting (4). |
| 26 | `int IPNBSSDK_CtrlQueryTermCount()` | Mengetahui jumlah total terminal yang terdaftar di database server. |

---

## 2. Analisis Implementasi Internal `IPNBSSDK_TCPConnect`

Hasil dekompilasi Ghidra membuktikan struktur eksekusi:
```c
void __cdecl IPNBSSDK_TCPConnect(char *szIP, void *pParam) {
    // 1. Membuat CString dari parameter IP string
    CString strIP(szIP);
    
    // 2. Memanggil method internal CNetClient dispatcher pada singleton DAT_100078a8
    FUN_10002a00(&DAT_100078a8, strIP, pParam);
}
```
Pada subrutin `FUN_10002a00`:
- Dibuat dedicated thread melalui Win32 API:
  `CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)LAB_10002850, this, 1, &param_2)`
- Thread handle disimpan pada offset objek `this + 0x58`.
- Dilakukan inisialisasi socket TCP stream (`FUN_10002c50`).
- Mengirimkan event state perubahan status koneksi ke callback handler (`FUN_100017e0`).

---

## 3. Rencana Kompatibilitas Rust FFI (Modern Shared Core)

Untuk menjaga kompatibilitas 100% dengan aplikasi client/station lama yang mengonsumsi `IPNBSSDK.dll`:
- Rust Shared Core akan mengompilasi dynamic library (`cdylib`) dengan atribut `#[no_mangle]` dan calling convention `extern "C"`.
- Semua 37 nama fungsi diekspor persis sama secara biner.
- Aplikasi Station legacy dapat langsung menggunakan `IPNBSSDK.dll` baru tanpa perlu kompilasi ulang (drop-in binary replacement).
