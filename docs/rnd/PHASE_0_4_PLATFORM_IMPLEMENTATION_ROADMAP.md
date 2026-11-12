# PHASE 0.4 — MULTI-PLATFORM IMPLEMENTATION ROADMAP & SEQUENCING

**Project:** Mitra Audio Network System  
**Document Code:** `ROADMAP-EXEC-FINAL-006`  
**Status:** READY FOR PHASE 1 IMPLEMENTATION (Phase 0.4 Final Gate)  
**Platform Sequence:** 1. Windows -> 2. Linux -> 3. macOS -> 4. MitraOS (ASL Kernel)  

---

## 1. Urutan Eksekusi Platform & Garis Batas Kesiapan

Setiap platform ditangani sesuai prioritasnya tanpa memaksakan semua platform harus selesai serentak. Kesiapan platform Windows menjadi fondasi stabilitas bagi platform berikutnya.

```mermaid
gantt
    title Mitra Audio Network System - Phased Implementation Roadmap
    dateFormat  YYYY-MM-DD
    section Phase 1: Windows (Priority 1)
    Shared Core & Rust Workspace Setup    :a1, 2026-11-15, 14d
    Audio DSP & WASAPI Driver             :a2, after a1, 14d
    Network Engine & tagDGRAMHEADER       :a3, after a1, 14d
    SQLite 3 Persistence & Jet Migrator   :a4, after a3, 10d
    IPNBSSDK.dll Drop-in 37 APIs FFI      :a5, after a3, 14d
    Desktop UI / Console & Live PTT       :a6, after a5, 21d
    Windows Installer (Inno/WiX) & SCM    :a7, after a6, 14d
    Windows Stabilization & Legacy Tests  :a8, after a7, 14d

    section Phase 2: Linux (Priority 2)
    Linux PAL Adapter (ALSA & epoll)      :b1, after a8, 21d
    systemd Daemon & Multi-NIC Multicast  :b2, after b1, 14d
    Packaging (DEB, RPM, AppImage)        :b3, after b2, 10d

    section Phase 3: macOS (Priority 3)
    macOS PAL Adapter (CoreAudio HAL)     :c1, after b3, 21d
    launchd Service & Universal DMG       :c2, after c1, 14d
    Apple Notarization & Gatekeeper       :c3, after c2, 7d

    section Phase 4: MitraOS (Priority 4)
    Stage A: ASL Kernel Discovery & ABI   :d1, after a8, 14d
    Stage B: Userspace Runtime (no_std)   :d2, after d1, 21d
    Stage C: Audio DMA Ring Buffer Driver :d3, after d2, 28d
    Stage D: smoltcp Multicast Networking :d4, after d3, 28d
    Stage E: Core DSP Porting to MitraOS  :d5, after d4, 21d
    Stage F: Native Compositor UI Client  :d6, after d5, 28d
    Stage G: End-to-End MitraOS Broadcast :d7, after d6, 21d
    Stage H: Real-Time Hardening & Stress :d8, after d7, 21d
```

---

## 2. Definisi Selesai (Definition of Done) Per Platform

### Platform 1: Windows (Priority 1) — Milestone Gates
1. **Core & SDK Gate:** Seluruh 37 fungsi `IPNBSSDK.dll` dapat dipanggil oleh aplikasi C/C++ native dan aplikasi legacy `IPNBS_Station.exe` tanpa error akses memori.
2. **Audio & Multicast Gate:** Siaran langsung audio mikrofon server via WASAPI terkirim melalui paket UDP multicast `234.0.0.254:2048` dan terdengar jernih pada terminal speaker horn di jaringan lokal.
3. **Database Migration Gate:** Berkas `DBS.mdb` lama berhasil dikonversi ke SQLite 3 WAL Mode tanpa kehilangan konfigurasi terminal dan jadwal bel.
4. **Installer Gate:** Berkas installer Windows memasang aplikasi, mendaftarkan Windows Service `MitraAudioService`, menyuntikkan aturan firewall, dan menyediakan first-launch wizard.

### Platform 2: Linux (Priority 2) — Milestone Gates
1. **Headless Daemon Gate:** `mitra-audio-server` berjalan stabil di bawah `systemd` pada Ubuntu 22.04/24.04 LTS dan Debian 12.
2. **Audio ALSA Gate:** Mampu menangkap audio dari soundcard ALSA hardware dan me-render stream tanpa buffer underrun.
3. **Packaging Gate:** Paket `.deb` dan standalone `AppImage` dapat dipasang dan diuji pada mesin Linux bersih.

### Platform 3: macOS (Priority 3) — Milestone Gates
1. **CoreAudio Gate:** Latensi capture dan playback audio terukur < 10 ms menggunakan Apple CoreAudio HAL.
2. **Notarization Gate:** Berkas Universal DMG (ARM64 Apple Silicon + Intel) ditandatangani dan lolos notarisasi Apple Developer tanpa peringatan keamanan Gatekeeper.

### Platform 4: MitraOS / ASL Kernel (Priority 4) — Milestone Gates
1. **Stage A & B Gate:** Aplikasi uji Rust `no_std` berjalan di userspace ASL Kernel menggunakan syscall native.
2. **Stage C & D Gate:** Kernel ASL mampu mengalirkan data audio melalui driver DMA soundcard dan mengirim paket UDP multicast via stack jaringan `smoltcp`.
3. **Stage G & H Gate:** Mitra Audio Network System berjalan secara native di atas MitraOS dan mampu membroadcast audio ke terminal fisik di jaringan.

---

## 3. Kebijakan Checkpoint & Rollback

- Setiap akhir milestone pada Phase 1 akan ditandai dengan checkpoint resmi di `CHECKPOINT.md` (e.g. `CP-PHASE-1-SDK-VERIFIED`, `CP-PHASE-1-WINDOWS-CORE-COMPLETE`).
- Jika terjadi regresi fatal pada suatu milestone, sistem dapat di-rollback ke checkpoint sebelumnya tanpa mempengaruhi integritas workspace lain atau berkas biner asli.
