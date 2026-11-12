# SPON COMPATIBILITY ASSESSMENT
**Proyek:** Mitra Audio Network System  
**Konteks:** Evaluasi Kelayakan Adapter Kompatibilitas Lisensi Legacy IPNBS / SPON  
**Status Evaluasi:** **BELUM DAPAT DIBUKTIKAN (NOT PROVABLE / CONTRACT UNAVAILABLE)**

---

## 1. Latar Belakang & Investigasi Sistem Legacy

Sistem warisan (Legacy IPNBS/SPON) mengandalkan dua mekanisme proteksi lisensi:
1. **Feitian ePass ET99 USB Token (`FT_ET99_API.dll`):** Token enkripsi perangkat keras fisik yang mengakses memori chip dongle USB untuk memverifikasi lisensi biner `IPNBS_Server.exe`.
2. **Skema Identifikasi Registrasi SPON:** Kode aktivasi atau serial number internal yang dipakai oleh sistem SPON versi awal.

---

## 2. Analisis Perbedaan Arsitektur

| Aspek | Mitra Audio Network System | Legacy IPNBS / SPON | Kompatibilitas |
|---|---|---|:---:|
| **Metode Proteksi** | Kriptografi Asimetris Ed25519 | Dongle USB Fisik Feitian ET99 | **TIDAK KOMPATIBEL** |
| **Media Identitas** | Native Hardware ID (BIOS, CPU, Disk) | Chip Smartcard Dongle USB | **TIDAK KOMPATIBEL** |
| **Model Lisensi** | File Digital `.lic` Bertanda Tangan | Penyimpanan EEPROM Dongle | **TIDAK KOMPATIBEL** |
| **Format Serial** | `MANS-<PROD4>-<HASH4>-<HASH4>-<CHK4>` | Format Kode Hex/Desimal Tertutup | **TIDAK KOMPATIBEL** |

---

## 3. Batasan Hukum, Teknis & Preservation Lock

Sesuai aturan ketat proyek:
1. **Dilarang Reverse Engineering Proteksi Kriptografi ET99:** Membongkar algoritma rahasia di dalam microchip ET99 atau membuat emulasi memori tanpa izin resmi merupakan pelanggaran hukum dan melanggar aturan Preservation Lock.
2. **Ketiadaan Kontrak Integrasi Resmi:** Tidak ada spesifikasi resmi, API publik, atau dokumentasi kontrak vendor yang menyediakan tata cara bridging antara format registrasi SPON dengan sistem lisensi pihak ketiga.

---

## 4. Kesimpulan & Rekomendasi Teknis

- **Kesimpulan Resmi:** Kelayakan pembuatan adapter lisensi transparan dua arah antara SPON legacy dan Mitra Audio Ed25519 **BELUM DAPAT DIBUKTIKAN (NOT PROVABLE)** karena ketiadaan kontrak resmi dan perbedaan mendasar arsitektur proteksi.
- **Rekomendasi Migrasi:**
  - Untuk sistem baru: Wajib menggunakan sistem lisensi asimetris Ed25519 berbasis Native ID Mitra Audio Network System.
  - Untuk sistem legacy yang masih berjalan: Pertahankan biner asli `FT_ET99_API.dll` dan dongle USB fisik tanpa modifikasi hingga stasiun lama dimigrasikan secara penuh ke Mitra Audio Station modern.
