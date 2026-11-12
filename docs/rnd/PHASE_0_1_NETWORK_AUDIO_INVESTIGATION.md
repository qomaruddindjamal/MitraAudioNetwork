# NETWORK AUDIO INVESTIGATION — PHASE 0.1
## MITRA AUDIO NETWORK SYSTEM

**Tanggal:** 2026-11-11  
**Topik:** Evaluasi Protokol IP Audio, Multicast, Jitter Buffer, dan Abstraksi Audio Engine  

---

### 1. Protokol Multicast & Format Paket Legacy

Berdasarkan analisis `IPNBS_Server.map` pada modul `CChannel`, `CNetCore`, dan header biner:
1. **Multicast Topology:** Menggunakan grup multicast Class D (`234.0.0.254`).
2. **Channel Allocation:** Channel ID dialokasikan secara dinamis (`dwChannelId: 0`..`N`). Setiap channel audio merepresentasikan satu sesi siaran/broadcast ke terminal.
3. **Packet Header Format:** Menggunakan `tagDGRAMHEADER` (ditemukan di `Member.obj` dan `Channel.obj`) yang memuat:
   - Header magic identifier
   - Channel index & Task priority (0 = Tertinggi s.d. 3 = Terendah)
   - Sequence number (untuk mitigasi packet reordering)
   - Payload length & Codec identifier (PCM, ADPCM, MP3)
   - Audio sample payload
4. **Port Multiplexing:**
   - Port 2046: Terminal Command & Control / Receive
   - Port 2048: Audio Stream Multicast / Broadcast Data

---

### 2. Rekomendasi Modernisasi Audio Core

1. **Abstraksi Audio Engine Lintas Platform:**
   - Menggunakan Rust portable audio traits (seperti CPAL / Core Audio / ALSA / WASAPI bindings).
   - Di Windows: Menggantikan WinMM legacy dengan **WASAPI (Windows Audio Session API)** untuk low-latency audio sharing.
   - Di Linux: Menggunakan **PipeWire / PulseAudio** asynchronous client.
   - Di macOS: Menggunakan **Core Audio AudioUnit / HAL**.
   - Di MitraOS: Menyiapkan trait `AudioDriver` yang akan memanggil DMA/Ring Buffer driver HDA saat kernel ASL telah mengembangkannya.
2. **Peningkatan Codec:**
   - Mempertahankan kompatibilitas mundur dengan streaming Linear PCM dan ADPCM.
   - Menambahkan dukungan codec modern **Opus** (ultra low-latency, bandwidth dinamis 6 kbps - 510 kbps, sangat tangguh terhadap packet loss).
3. **Jitter Buffer & Packet Loss Concealment (PLC):**
   - Menggantikan antrian blocking IOCP lama dengan adaptive circular jitter buffer yang secara dinamis menyesuaikan latency (5ms - 50ms) berdasarkan kondisi statistik jaringan.
