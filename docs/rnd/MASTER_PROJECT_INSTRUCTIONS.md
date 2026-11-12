# MASTER PROJECT INSTRUCTIONS
# MITRA AUDIO NETWORK SYSTEM

**Workspace:** `C:\IPNBS`  
**GitHub Repository:** `https://github.com/qomaruddindjamal/MitraAudioNetwork.git`  
**Bahasa:** Bahasa Indonesia  
**Mode Saat Ini:** PHASE 0 — DISCOVERY, DEEP AUDIT, R&D, PRODUCT DESIGN & ARCHITECTURE RESEARCH  
**Status Keamanan:** READ-ONLY / NON-DESTRUCTIVE / PRESERVATION LOCK AKTIF  

---

## 1. ATURAN UTAMA & PRESERVATION LOCK
1. Tidak ada perubahan pada binary, database, konfigurasi, atau file legacy di `C:\IPNBS`.
2. Tidak ada perubahan atau modifikasi pada kernel ASL (`C:\asl`).
3. Tidak ada commit atau push ke remote Git tanpa persetujuan eksplisit.
4. Baseline integritas SHA-256 telah dihitung dan disimpan di `docs/rnd/BASELINE_SHA256.txt`.

## 2. URUTAN PRIORITAS TARGET PLATFORM
1. **Priority 1: Windows** — Audit mendalam sistem legacy IPNBS, arsitektur core & modernisasi.
2. **Priority 2: Linux** — Porting & integrasi native Linux (ALSA/PulseAudio/PipeWire, DEB/AppImage).
3. **Priority 3: macOS** — Porting & integrasi native macOS (Core Audio, Apple Silicon/Intel, DMG/PKG).
4. **Priority 4: MitraOS / ASL Kernel** — Target native mandiri (bukan Linux). Identifikasi kebutuhan syscall, audio subsystem, network stack, dan userspace ABI.

## 3. PHASE GATE
- RND → AUDIT → PERBAIKAN → IMPLEMENTASI → PENGUJIAN → VERIFIKASI → CHECKPOINT → LANJUTKAN.
- Hanya Phase 0 yang aktif saat ini.
