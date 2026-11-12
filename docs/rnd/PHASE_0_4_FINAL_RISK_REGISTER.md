# PHASE 0.4 — FINAL RISK REGISTER & MITIGATION STRATEGY

**Project:** Mitra Audio Network System  
**Document Code:** `RISK-REGISTER-FINAL-008`  
**Status:** READY FOR PHASE 1 IMPLEMENTATION (Phase 0.4 Final Gate)  

---

## 1. Comprehensive Risk Assessment

Register ini mengonsolidasi seluruh risiko teknis dan operasional yang dihadapi proyek, mengevaluasi tingkat keparahan setelah penerapan kontrol mitigasi arsitektur:

| ID Risiko | Deskripsi Risiko | Dampak Awal | Probabilitas Awal | Mitigasi Arsitektural / Kontrol Rekayasa | Dampak Residu | Probabilitas Residu | Tingkat Risiko Residu |
|---|---|---|---|---|---|---|---|
| **RSK-01** | Ketidakcocokan ABI x86 32-bit `IPNBSSDK.dll` dengan aplikasi stasiun lama. | Kritis | Sedang | - Kompilasi target `i686-pc-windows-msvc`.<br>- Module Definition (`.def`) file mengunci ordinal 1 s/d 37.<br>- Automated C test harness memverifikasi pemanggilan fungsi sebelum rilis. | Rendah | Sangat Rendah | **LOW** |
| **RSK-02** | Terminal speaker horn lawas tidak mampu membunyikan stream audio Opus modern. | Kritis | Tinggi | - Diterapkan kebijakan **Dual-Stack Codec** non-destruktif.<br>- Transcoding adaptif: menyiarkan IMA-ADPCM 22.05 kHz ke hardware lawas dan Opus ke stasiun baru. | Rendah | Sangat Rendah | **LOW** |
| **RSK-03** | Kerusakan data saat konversi berkas database Access Jet MDB ke SQLite. | Tinggi | Sedang | - Berkas `DBS.mdb` dibuka secara strictly read-only.<br>- Sanitasi trailing null bytes dan parsing timestamp OLE float.<br>- Verifikasi hash dan jumlah record sebelum database baru diaktifkan. | Sangat Rendah | Rendah | **LOW** |
| **RSK-04** | Jaringan router/switch di lokasi pelanggan memblokir paket UDP Multicast. | Tinggi | Sedang | - Implementasi deteksi otomatis (multicast probe ping).<br>- Fallback transparan ke Unicast Multi-Stream UDP / TCP streaming. | Sedang | Rendah | **LOW** |
| **RSK-05** | Kebocoran master private key lisensi Ed25519 ke pelanggan. | Kritis | Rendah | - Master Private Key diisolasi di Issuer Portal / HSM aman.<br>- Biner aplikasi hanya memuat Public Key untuk verifikasi. | Sangat Rendah | Sangat Rendah | **LOW** |
| **RSK-06** | Perubahan hardware komputer pelanggan membatalkan lisensi sah (false license invalidation). | Tinggi | Sedang | - Menggunakan fuzzy scoring Native ID (2 dari 3 komponen identik tetap valid).<br>- Menghindari dependensi pada MAC address adapter virtual/VPN. | Rendah | Rendah | **LOW** |
| **RSK-07** | Ketidaksiapan subsistem audio dan networking pada ASL Kernel (MitraOS). | Tinggi | Pasti | - Isolasi kode kernel di `C:\asl` (Preservation Lock).<br>- Eksekusi bertahap melalui 8-Stage Roadmap (Stage A s/d H) tanpa memblokir Windows/Linux/macOS. | Rendah | Rendah | **LOW** |
| **RSK-08** | Konflik hak akses audio mikrofon pada sistem operasi Windows 10/11. | Sedang | Sedang | - First-launch wizard mendeteksi status Windows Microphone Privacy Settings.<br>- Memberikan panduan visual kepada administrator untuk mengizinkan akses. | Rendah | Rendah | **LOW** |

---

## 2. Kesimpulan Manajemen Risiko

Seluruh risiko teknis berkategori **Kritis** dan **Tinggi** telah berhasil diturunkan tingkat keparahannya menjadi **LOW** melalui kontrol rekayasa arsitektural yang terbukti. Tidak ada risiko tak terkelola yang dapat menghentikan transisi menuju implementasi Phase 1.
