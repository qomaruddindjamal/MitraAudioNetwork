# PHASE 0.3 — LICENSING ARCHITECTURE & SECURITY SUBSYSTEM DESIGN

**Project:** Mitra Audio Network System  
**Document Code:** `ARCH-LICENSE-SEC-005`  
**Status:** VALIDATED (Phase 0.3 Blueprint)  

---

## 1. Arsitektur Lisensi Asimetris Baru (Ed25519)

Untuk melepaskan ketergantungan dari USB dongle proprietary x86 lama (`FT_ET99_API.dll`) tanpa mengorbankan keamanan komersial, dirancang sistem lisensi asimetris berbasis **Ed25519 (Edwards-curve Digital Signature Algorithm)**.

```mermaid
flowchart TD
    subgraph Secure Issuer Environment (Offline / Portal Server)
        MASTER_KEY[Private Signing Key: Ed25519\nStrictly Kept in HSM / Vault]
        LICENSE_TOOL[Mitra License Issuer Tool]
        MASTER_KEY --> LICENSE_TOOL
    end

    subgraph Customer Machine (Target Host)
        APP[Mitra Audio Network System Core]
        PUB_KEY[Public Verification Key: Ed25519\nEmbedded Inside App Core Binary]
        NATIVE_ID_MOD[Native ID Fingerprint Module]
    end

    NATIVE_ID_MOD -- "1. Generate Hardware Hash" --> MACHINE_ID[Customer Native Machine ID]
    MACHINE_ID -- "2. Kirim Serial ID untuk Registrasi" --> LICENSE_TOOL
    LICENSE_TOOL -- "3. Generate & Sign License File (.lic)" --> LIC_FILE[Signed License Payload]
    LIC_FILE -- "4. Import Berkas Lisensi" --> APP
    PUB_KEY --> APP
    APP -- "5. Validasi Tanda Tangan & Match Hardware" --> UNLOCK{Lisensi Sah?}
    UNLOCK -- Ya --> ACTIVE[Aktifkan Lisensi Sesuai Tier: 50/100/500 Terminal]
    UNLOCK -- Tidak --> DEMO[Mode Demo Terbatas: Max 5 Terminal]
```

> [!CAUTION]
> **ATURAN KEAMANAN KRITIS:**
> Private Signing Key Ed25519 **HANYA** disimpan pada server penerbit lisensi (Issuer Portal / HSM) dan **TIDAK BOLEH** didistribusikan ke dalam repositori kode sumber atau biner aplikasi yang dikirimkan ke pelanggan. Aplikasi hanya memuat Public Key untuk verifikasi tanda tangan matematis.

---

## 2. Struktur Payload Berkas Lisensi (`.lic`)

Berkas lisensi berupa payload JSON yang ditandatangani secara kriptografis dan dikemas dalam format Base64 yang aman:

```json
{
  "header": {
    "version": "1.0",
    "algorithm": "Ed25519",
    "issued_at": "2026-11-11T00:00:00Z"
  },
  "claims": {
    "license_id": "MITRA-2026-LIC-00892",
    "customer": {
      "organization": "PT Audio Network Nusantara",
      "contact_email": "admin@audionet.co.id"
    },
    "entitlements": {
      "tier": "ENTERPRISE",
      "max_terminals": 250,
      "max_broadcast_channels": 16,
      "features": [
        "udp_multicast",
        "intercom_duplex",
        "fire_alarm_override",
        "audio_recording",
        "opus_codec",
        "rest_api_access"
      ]
    },
    "validity": {
      "not_before": "2026-11-11T00:00:00Z",
      "expires_at": "2028-11-11T23:59:59Z",
      "is_perpetual": false
    },
    "binding": {
      "platform": "windows",
      "native_id_hash": "a8f5c2e17d98b04a5e3c12f4b7a8c9e0d1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6",
      "tolerance_score": 1
    }
  },
  "signature": "8f3a1b...[64-byte Ed25519 signature in Hex / Base64]..."
}
```

---

## 3. Desain Native ID Multi-Platform & Toleransi Perubahan Hardware

Native ID dirancang untuk menyeimbangkan **stabilitas identitas**, **perlindungan privasi**, dan **toleransi perubahan perangkat keras wajar** (misalnya pergantian kartu jaringan atau update driver):

| Platform | Komponen Hardware yang Di-query | Metode Pengambilan Data | Kebijakan Toleransi |
|---|---|---|---|
| **Windows** | 1. Motherboard BIOS UUID<br>2. CPU Processor ID<br>3. Primary System Drive Serial | Win32 WMI / `DeviceIoControl` | Menggunakan fuzzy hash (Score 2 dari 3 komponen identik tetap dianggap sah). |
| **Linux** | 1. `/etc/machine-id`<br>2. `/sys/class/dmi/id/product_uuid`<br>3. Root Block Device UUID | Sysfs read direct | Pergantian adapter jaringan (NIC) tidak merusak lisensi. |
| **macOS** | 1. `IOPlatformUUID`<br>2. Hardware Model & Serial | IOKit Registry API | Aman terhadap instal ulang OS / upgrade macOS. |
| **MitraOS** | 1. ASL Kernel Machine UUID<br>2. System Board ID | Native Syscall (Stage A/B) | Bebas dependensi POSIX / driver eksternal. |

### Kebijakan Virtual Machine & Container:
- Pada lingkungan virtualisasi (VMware, Hyper-V, KVM, Proxmox), Native ID mengekstrak hypervisor UUID binding.
- Lisensi Enterprise mendukung opsi lisensi floating atau lisensi cluster khusus VM.

---

## 4. Mekanisme Anti-Rollback Jam Sistem (Clock Tamper Protection)
Untuk mencegah manipulasi waktu komputer agar lisensi tidak pernah expired:
1. **Monotonic High-Resolution Clock:** Menggunakan monotonic timestamp kernel yang tidak dapat dimundurkan oleh perubahan waktu kalender pengguna.
2. **Encrypted Last-Run Watermark:** Setiap kali server beroperasi, timestamp terakhir disimpan dalam file state terenkripsi. Jika waktu sistem sekarang lebih awal daripada timestamp operasional terakhir, sistem mendeteksi manipulasi waktu dan mengunci sementara fitur siaran.
3. **NTP Network Time Verification:** Jika jaringan internet atau local NTP server tersedia, sistem memvalidasi waktu resmi secara independen.
