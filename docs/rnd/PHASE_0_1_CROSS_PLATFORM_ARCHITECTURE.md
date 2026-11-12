# CROSS-PLATFORM ARCHITECTURE VALIDATION — PHASE 0.1
## MITRA AUDIO NETWORK SYSTEM

**Tanggal:** 2026-11-11  
**Topik:** Pemisahan Shared Core & Platform Adapter Lintas 4 Platform  

---

### 1. Model Arsitektur Modular (Shared Core vs Platform Adapters)

```
┌────────────────────────────────────────────────────────────────────────┐
│               PRESENTATION LAYER (GUI Desktop & Headless)               │
│               - Desktop GUI: Tauri 2 (HTML5 / Modern CSS)              │
│               - Headless Daemon / CLI Service Mode                     │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ IPC / Typed RPC
┌───────────────────────────────────▼────────────────────────────────────┐
│                    MITRA AUDIO SHARED CORE (RUST)                      │
│                                                                        │
│  [Audio Pipeline]    [Network Engine]   [License Engine]  [Storage]    │
│  - Mixer & Matrix    - UDP Multicast    - Ed25519 Verify  - SQLite3    │
│  - Opus/PCM Codecs   - Unicast Session  - Native ID Hash  - SQLx ORM   │
│  - Jitter Buffer     - Packet Recovery  - Quota Policy    - Migrations │
│  - Volume & EQ       - Device Discovery - Feature Flags   - Audit Logs │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    │ Platform Adapter Traits
       ┌────────────────────────────┼────────────────────────────┐
       ▼                            ▼                            ▼
┌──────────────┐             ┌──────────────┐             ┌──────────────┐
│   WINDOWS    │             │    LINUX     │             │    macOS     │
│   ADAPTER    │             │   ADAPTER    │             │   ADAPTER    │
│ WASAPI Audio │             │ PipeWire/ALSA│             │ Core Audio   │
│ WMI Identity │             │ DMI / sysfs  │             │ IOKit UUID   │
│ Windows Svc  │             │ systemd unit │             │ launchd plist│
│ InnoSetup/MSI│             │ DEB/AppImage │             │ DMG / PKG    │
└──────────────┘             └──────────────┘             └──────────────┘
                                    │
                                    ▼ (Priority 4)
                             ┌──────────────┐
                             │   MITRAOS    │
                             │ ASL ADAPTER  │
                             │ Native HDA   │
                             │ ASL Vault    │
                             │ ASL IPC / SVC│
                             │ Raw Bundle   │
                             └──────────────┘
```

---

### 2. Validasi Kelayakan Teknis (Feasibility Check)

1. **Rust Portable Shared Core:**
   - Kompatibel 100% dengan Windows, Linux, dan macOS.
   - Untuk MitraOS: Karena ASL Kernel ditulis dengan Rust bare-metal (`no_std`), Shared Core dapat dikompilasi dengan konfigurasi feature `no_std` + `alloc` untuk berjalan secara native di lingkungan MitraOS tanpa emulasi Linux.
2. **Database Migration Strategy:**
   - Format biner Microsoft Access MDB (`DBS.mdb`) tidak dapat dibuka di Linux/macOS/MitraOS.
   - *Solusi:* SQLite adalah standar industri yang bersifat single-file, zero-configuration, cross-platform 100%, dan mendukung enkripsi transparent (SQLCipher).
3. **GUI Framework Strategy:**
   - Windows, Linux, macOS: Menggunakan Tauri 2 (menggunakan Webview native masing-masing OS sehingga ukuran biner sangat kecil ~10-15MB).
   - MitraOS: Aplikasi Mitra Audio Network System dapat berjalan sebagai background daemon / headless service terlebih dahulu, sebelum subsystem GUI MitraOS siap.
