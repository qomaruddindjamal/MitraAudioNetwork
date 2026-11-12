# PHASE 0.3 — NETWORK PROTOCOL SPECIFICATION & PACKET FRAMING

**Project:** Mitra Audio Network System  
**Document Code:** `SPEC-NET-PROTO-004`  
**Status:** VALIDATED (Phase 0.3 Blueprint)  

---

## 1. Topologi Jaringan & Port Allocation

Sistem IP Audio Network beroperasi menggunakan pemisahan kanal kendali (control) dan kanal media (audio streaming):

| Kanal Jaringan | Protokol Transport | Alamat Default | Port Default | Peran & Karakteristik |
|---|---|---|---|---|
| **Control Channel (Station-Server)** | TCP | Server Unicast IP | `2046` | Manajemen sesi, autentikasi, status terminal, trigger siaran. |
| **Multicast Audio Stream** | UDP Multicast | `234.0.0.254` | `2048` | Pengiriman stream audio 1-ke-banyak (One-to-Many broadcast). |
| **Unicast Audio Stream (Fallback)** | UDP Unicast | Terminal Target IP | `2048` - `2060` | Fallback siaran saat jaringan WAN memblokir multicast routing. |
| **P2P Direct Intercom** | UDP Unicast | Terminal-to-Terminal | `2050` | Panggilan audio dua arah full-duplex antar terminal. |
| **State & Heartbeat Ping** | UDP / TCP | Server IP / Multicast | `2047` | Pelaporan kondisi perangkat (Online, Battery, Signal, Error). |

---

## 2. Spesifikasi Biner Datagram Audio UDP (`tagDGRAMHEADER`)

Header biner berukuran tepat **16 bytes** diposisikan pada awal setiap paket UDP audio:

```text
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   SyncByte    |  PacketType   |          ChannelID            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        SequenceNumber                         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          PayloadSize          |   AudioCodec  | SampleRateCode|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Priority    |   Reserved    |           Checksum            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Audio Payload Data ...                 |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Rincian Field & Status Validasi:

| Byte Offset | Field Name | Tipe Data | Definisi Nilai & Makna | Status Validasi |
|---|---|---|---|---|
| `0x00` | `SyncByte` | `uint8` | Magic sync constant: `0x5A` (atau `0xA5`). Memvalidasi integritas awal frame. | `VERIFIED` |
| `0x01` | `PacketType` | `uint8` | `0x01`: Audio Payload, `0x02`: Control Header, `0x03`: Heartbeat, `0x04`: Emergency Alarm. | `VERIFIED` |
| `0x02 - 0x03` | `ChannelID` | `uint16_le` | Nomor channel siaran (1 s/d 65.535). Terminal hanya memainkan audio channel miliknya. | `VERIFIED` |
| `0x04 - 0x07` | `SequenceNumber` | `uint32_le` | Counter paket berurutan untuk mendeteksi frame hilang, duplikat, atau out-of-order. | `VERIFIED` |
| `0x08 - 0x09` | `PayloadSize` | `uint16_le` | Ukuran data audio payload dalam bytes (biasanya 512, 1024, atau 1400 bytes). | `VERIFIED` |
| `0x0A` | `AudioCodec` | `uint8` | `0x01`: PCM 16-bit, `0x02`: IMA-ADPCM, `0x03`: MP3, `0x04`: Opus (Modern Proposed). | `VERIFIED` |
| `0x0B` | `SampleRateCode`| `uint8` | `0x01`: 11.025 Hz, `0x02`: 22.050 Hz, `0x03`: 44.100 Hz, `0x04`: 48.000 Hz. | `VERIFIED` |
| `0x0C` | `Priority` | `uint8` | `0x01`: BGM (Rendah), `0x02`: Panggilan Biasa (Normal), `0x03`: Fire Alarm (Darurat). | `VERIFIED` |
| `0x0D` | `Reserved` | `uint8` | Padding alignment 32-bit (saat ini diset `0x00`). Ditandai: | `PARTIALLY VERIFIED` |
| `0x0E - 0x0F` | `Checksum` | `uint16_le` | 16-bit CRC / Additive Checksum atas seluruh paket. | `VERIFIED` |

---

## 3. TCP Control Channel Framing

Komunikasi kendali menggunakan format length-prefixed binary stream untuk mencegah fragmentation / packet boundary issues pada protokol stream TCP:

```text
[Header 4 Bytes LE: Total Packet Length] + [Command Code 2 Bytes LE] + [Session Token 4 Bytes] + [Payload N Bytes]
```

### Lifecycle Koneksi & Graceful Teardown:
- Saat terminal atau station melakukan disconnect, sesi TCP ditutup dengan mengirimkan paket `CMD_LOGOUT` diikuti `FIN` handshake standar TCP.
- Shared Core menangani penutupan soket tanpa melempar error false-positive (seperti error IOCP 995 pada server legacy).
- Jika koneksi terputus secara mendadak (kabel putus/reboot), mekanisme Keep-Alive Heartbeat (timeout 9 detik) akan mendeteksi status offline dan melepaskan seluruh alokasi sumber daya audio secara aman.

---

## 4. Evaluasi Asynchronous Networking Runtime

| Engine Async | Kelebihan | Kekurangan | Rekomendasi Arsitektur |
|---|---|---|---|
| **Tokio (Rust)** | Ekosistem sangat matang, performa I/O completion port tinggi, mendukung ribuan koneksi konkuren. | Ukuran binary bertambah ~1-2 MB, memerlukan runtime allocator. | **Primary Engine untuk Windows, Linux, dan macOS.** |
| **Mio (Metal I/O)** | Sangat ringan, low-level event loop langsung di atas IOCP/epoll/kqueue. | Harus menulis state machine async secara manual. | Alternatif jika constraint memori sangat ketat. |
| **smoltcp / Bare-Metal Socket** | Dapat berjalan di lingkungan `no_std` tanpa kernel OS. | Tidak memiliki abstraksi thread bawaan. | **Primary Engine untuk MitraOS (Stage D).** |
