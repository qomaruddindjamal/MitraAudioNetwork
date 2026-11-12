# LICENSE & NATIVE ID INVESTIGATION — PHASE 0.1
## MITRA AUDIO NETWORK SYSTEM

**Tanggal:** 2026-11-11  
**Topik:** Pemetaan Skema Identitas Mesin (Native ID), Serial Number, dan Transisi Lisensi Kriptografis Modern  

---

### 1. Perbandingan Skema Native ID Lintas 4 Platform

| Parameter | Windows (Priority 1) | Linux (Priority 2) | macOS (Priority 3) | MitraOS / ASL (Priority 4) |
|---|---|---|---|---|
| **Sumber Utama** | WMI / Win32_BaseBoard Serial + Disk Physical Serial | `/etc/machine-id` (systemd) + DMI UUID (`/sys/class/dmi/id/product_uuid`) | `IOPlatformUUID` via IOKit (`IOPlatformExpertDevice`) | Hardware Hash (CPU/Board Identity via ASL Security Vault) |
| **Ketahanan Reinstall OS** | Sangat Tinggi (karena berbasis serial motherboard/disk fisik) | Menengah (DMI UUID tahan reinstall, `/etc/machine-id` berubah jika format penuh) | Sangat Tinggi (`IOPlatformUUID` terikat pada NVRAM/logic board) | Sangat Tinggi (Terikat pada hardware hash firmware) |
| **Dampak Virtualisasi** | VM hypervisor men-generate UUID acak | VM menyajikan virtual DMI | macOS VM menyajikan static/generated IOPlatformUUID | Hypervisor guest identity |
| **Kebutuhan Privilege** | User standar / Query WMI | Read access `/etc/machine-id`, butuh root untuk raw DMI | User-level IOKit API (tanpa butuh sudo) | User-mode syscall request ke ASL Security Vault |

---

### 2. Dekoupling Arsitektur Lisensi Modern

Untuk menghindari kerapuhan dongle fisik USB ET99 (yang rawan hilang, rusak, atau tidak didukung di sistem operasi baru), dirancang pemisahan peran yang tegas:

1. **Product ID:** String pengenal aplikasi (`MITRA-AUDIO-NET-ENTERPRISE-v1`).
2. **Native ID:** Hash SHA-256 (64 hex characters) yang dihitung dari atribut perangkat keras lokal:
   $$\text{NativeID} = \text{HMAC-SHA256}(\text{Salt}, \text{MotherboardSerial} \parallel \text{CPUModel} \parallel \text{SystemUUID})$$
3. **Serial Number (SN):** Kunci aktivasi manusiawi yang diformat Base32 dengan checksum CRC32 (contoh: `MANS-7X9B-K21P-89WA-04ZF`). SN tidak membawa hak cipta kriptografis, melainkan menjadi nomor klaim aktivasi.
4. **Signed License Payload:** Berkas digital biner bertandatangan kriptografis **Ed25519**.
   - Berisi: `license_id`, `customer_name`, `edition`, `channel_quota`, `expiry_timestamp`, dan `allowed_native_id_hash`.
   - Ditandatangani menggunakan **Private Signing Key** di server vendor (Issuer).
5. **Public Verification Key:** Kunci publik Ed25519 (32-byte) yang di-embed langsung ke dalam aplikasi client Mitra Audio Network System untuk memverifikasi payload lisensi saat startup secara offline.
