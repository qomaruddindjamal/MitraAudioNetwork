# PHASE 0.2 — CROSS-PLATFORM SYSTEM SPECIFICATION & ARCHITECTURE DESIGN

**Project:** Mitra Audio Network System  
**Document Code:** `SPEC-CROSS-PLATFORM-ARCH-V1`  
**Target Architecture:** Rust Core + Platform Adapters + Native UI / Web GUI  

---

## 1. Prinsip Desain & Hierarki Platform

Pengembangan Mitra Audio Network System mematuhi urutan prioritas yang ketat:
1. **Windows (Prioritas 1):** Lingkungan operasional utama saat ini; kompatibilitas backwards biner dan kelancaran transisi adalah prioritas tertinggi.
2. **Linux (Prioritas 2):** Server headless / desktop environment standar industri (Ubuntu, Debian, RHEL) untuk deployment broadcast server terpusat.
3. **macOS (Prioritas 3):** Desktop workstation untuk operator broadcast berbasis Apple silicon (ARM64) dan Intel (x86_64).
4. **MitraOS berbasis ASL Kernel (Prioritas 4):** Sistem operasi masa depan independen yang dikembangkan secara bertahap (Stage A s/d Stage H).

```mermaid
graph TD
    subgraph "Layer 4: User Interface & Presentation"
        UI_WIN["Windows Desktop UI (WinUI 3 / Slint / Webview)"]
        UI_LIN["Linux Desktop UI (Slint / GTK / Webview)"]
        UI_MAC["macOS Desktop UI (Cocoa / Slint / Webview)"]
        UI_MITRA["MitraOS UI (Stage F: Native Compositor UI)"]
        UI_WEB["Modern Responsive Web GUI (HTML5/WebSockets/REST)"]
    end

    subgraph "Layer 3: Platform Abstraction Layer (PAL)"
        PAL_WIN["Windows PAL Adapter (WASAPI, IOCP, SCM Service, Win Registry)"]
        PAL_LIN["Linux PAL Adapter (ALSA/PipeWire, epoll, systemd, /etc config)"]
        PAL_MAC["macOS PAL Adapter (CoreAudio, kqueue, launchd, plist)"]
        PAL_MITRA["MitraOS PAL Adapter (ASL Native Syscalls, Stage C/D Drivers)"]
    end

    subgraph "Layer 2: Mitra Audio Shared Core (Rust no_std/std Engine)"
        CORE_AUDIO["Audio DSP Engine (Resampling, Mixing, AGC, Codecs: PCM/ADPCM/Opus/MP3)"]
        CORE_NET["Network Engine (UDP Multicast, Jitter Buffer, Session Dispatcher, Framing)"]
        CORE_DB["Storage Layer (SQLite 3 WAL Engine, ORM, Schema Migrator)"]
        CORE_LIC["Cryptographic Licensing Engine (Ed25519, Secure Token, Clock Guard)"]
        CORE_TASK["Scheduler & Automation Engine (Bell Schedules, Cron, Zone Matrix)"]
    end

    subgraph "Layer 1: Legacy Compatibility Layer"
        LEG_SDK["IPNBSSDK.dll C-ABI Wrapper (37 Exported Functions)"]
        LEG_TOKEN["FT_ET99_API.dll Compatibility Wrapper"]
        LEG_MIG["Jet MDB to SQLite Data Importer"]
    end

    UI_WIN --> PAL_WIN
    UI_LIN --> PAL_LIN
    UI_MAC --> PAL_MAC
    UI_MITRA --> PAL_MITRA
    UI_WEB --> CORE_NET

    PAL_WIN --> CORE_AUDIO
    PAL_WIN --> CORE_NET
    PAL_LIN --> CORE_AUDIO
    PAL_LIN --> CORE_NET
    PAL_MAC --> CORE_AUDIO
    PAL_MAC --> CORE_NET
    PAL_MITRA --> CORE_AUDIO
    PAL_MITRA --> CORE_NET

    LEG_SDK --> CORE_NET
    LEG_TOKEN --> CORE_LIC
    LEG_MIG --> CORE_DB
```

---

## 2. Rincian Platform Adapter

### 2.1 Windows Platform Adapter (Priority 1)
- **Audio Backend:** WASAPI (Windows Audio Session API) dengan fallback DirectSound / WinMM.
- **Network Backend:** Windows Sockets 2 (`WSASocket`, IOCP / `wepoll` terintegrasi async tokio).
- **Service Integration:** Windows Service Control Manager (SCM) untuk running sebagai background daemon `MitraAudioService`.
- **Packaging:** MSI / Inno Setup modern installer dengan automatic prerequisite detector, registry handler, dan automatic migration tool.

### 2.2 Linux Platform Adapter (Priority 2)
- **Audio Backend:** ALSA Native & PipeWire / PulseAudio via CPAL.
- **Network Backend:** Native Linux `epoll` + POSIX sockets dengan `SO_BINDTODEVICE` dan multicast loop flags.
- **Service Integration:** `systemd` unit files (`mitra-audio.service`) dengan watchdogs.
- **Packaging:** Debian package (`.deb`), RPM package (`.rpm`), dan AppImage standalone.

### 2.3 macOS Platform Adapter (Priority 3)
- **Audio Backend:** Apple CoreAudio dengan low-latency HAL unit.
- **Network Backend:** BSD sockets dengan `kqueue` async dispatch.
- **Service Integration:** Apple `launchd` daemon (`com.mitra.audionetwork.plist`).
- **Packaging:** Signed and notarized Universal DMG (`.dmg`) bundle.

### 2.4 MitraOS / ASL Kernel Platform Adapter (Priority 4)
- **Status:** Sistem Operasi independen (bukan distro Linux, bukan POSIX emulation).
- **Pendekatan Pengembangan:** 8-Stage Sequential Roadmap:
  - **Stage A (Kernel Discovery & ABI Contract):** Audit syscall interface, memory paging, format ELF / custom binary loader ASL Kernel.
  - **Stage B (Runtime Support):** Menghubungkan Rust `core` / `alloc` ke ASL memory allocation syscalls (`sys_mmap` / `sys_brk`).
  - **Stage C (Audio Architecture):** Spesifikasi audio buffer interface kernel ASL (ring-buffer DMA audio).
  - **Stage D (Networking & Multicast):** Integrasi TCP/IP stack (e.g. `smoltcp` bare-metal) dengan driver kartu jaringan ethernet ASL.
  - **Stage E (Audio Processing Engine):** Porting DSP engine Shared Core ke atas ASL runtime.
  - **Stage F (Native Application & GUI):** Integrasi antarmuka visual native MitraOS compositor.
  - **Stage G (Mitra Audio Integration):** Deployment lengkap Mitra Audio Network pada MitraOS.
  - **Stage H (Verification & Validation):** Pengujian ketahanan, stabilitas, dan performa real-time audio.
