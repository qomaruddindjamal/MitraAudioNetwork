# LAPORAN UJI INTEGRASI REGISTRASI PELANGGAN
**Dokumen:** `CUSTOMER_REGISTRATION_TEST_REPORT.md`  
**Proyek:** Mitra Audio Network System  
**Test Suite:** `crates/mitra-core/tests/e2e_gui_poc_tests.rs`  
**Status Eksekusi:** PASS (100% Skenario Pengujian Berhasil)

---

## 1. Lingkungan & Parameter Pengujian

- **Sistem Operasi:** Windows 10/11 x64 Native Session
- **Toolchain:** Rust 1.99.0 / MSVC Linker
- **Target Uji Coba:**
  1. `mitra-sn-generator.exe` (Vendor Authority Tool)
  2. `mitra-customer-app.exe` (Customer-facing Application)
- **Mesin Uji Native ID Sintetis:**
  `WIN-HWID-V1:MB-TEST-DELL-OPTIPLEX:CPU-I7-12700:DISK-NVME-SAMSUNG`
- **Product ID Uji:** `MITRA-AUDIO-SERVER`

---

## 2. Matriks Hasil Pengujian Alur Registrasi

| ID Kasus | Deskripsi Pengujian | Kondisi Masukan | Hasil Diharapkan | Hasil Aktual | Status |
|---|---|---|---|---|---|
| **E2E-01** | Pembangkitan Serial Number | Native ID valid + Product ID valid | SN 24 karakter format `MANS-MITR-XXXX-XXXX-YYYY` | SN terbentuk: `MANS-MITR-...` | **PASS** |
| **E2E-02** | Penerbitan Lisensi Permanen | Payload valid + Entitlements ganda | Paket lisensi bertanda tangan Ed25519 64-byte | Terbit sukses (`license_type: permanent`) | **PASS** |
| **E2E-03** | Ekspor Format Berkas `.lic` | Lisensi yang baru terbit | Format teks kanonikal lengkap dengan header `V1` | Berkas terbentuk & parsed cleanly | **PASS** |
| **E2E-04** | Verifikasi Sisi Klien Mandiri | Public Key 32-byte saja (tanpa private key) | Verifikasi digital signature & Native ID cocok | STATUS: VALID (PASS) | **PASS** |
| **E2E-05** | Penegakan Hak Fitur (Entitlements) | Cek hak akses `WEB_MANAGEMENT` | Fitur diaktifkan | `check_entitlement` PASS | **PASS** |
| **E2E-06** | Penolakan Perangkat Berbeda (Cloning) | Berkas lisensi dicoba di Native ID asing | Verifikasi ditolak seketika | Error: `HardwareMismatch` | **PASS** |
| **E2E-07** | Penolakan Produk Berbeda | Berkas lisensi dicoba pada produk lain | Verifikasi ditolak seketika | Error: `ProductMismatch` | **PASS** |
| **E2E-08** | Penolakan Modifikasi Teks Payload | License ID / SN diedit secara manual | Tanda tangan Ed25519 tidak valid | Error: `InvalidSignature` | **PASS** |
| **E2E-09** | Penolakan Lisensi yang Dicabut (Revoked) | License ID dimasukkan ke daftar pencabutan | Lisensi ditolak aktif | Error: `LicenseRevoked` | **PASS** |
| **E2E-10** | Pencegahan Duplikasi Penerbitan | Pendaftaran License ID ganda pada Authority | Basis data otoritas menolak duplikasi | Error: `DuplicateLicenseId` | **PASS** |
| **E2E-11** | Pembuktian Permanen Lewat 1 Tahun | Waktu simulasi 2 tahun setelah pembelian | Lisensi tetap aktif dan valid di klien | Validasi Klien PASS (Permanen Utuh) | **PASS** |
| **E2E-12** | Pembuktian Dukungan SN Terbatas 1 Tahun | Waktu simulasi 2 tahun setelah pembelian | Hak penggantian SN gratis tidak aktif | Layanan Dukungan Penggantian EXPIRED | **PASS** |

---

## 3. Pengujian Persistensi Registrasi

1. Lisensi berhasil disimpan secara permanen di:
   `C:\IPNBS\mitra_customer_registration.dat`.
2. Saat aplikasi pelanggan ditutup paksa (`kill process`) dan dibuka kembali:
   - Fungsi `load_persisted_registration()` membaca berkas tersebut.
   - Verifikasi tanda tangan digital dijalankan ulang secara luring (*offline*).
   - Status aplikasi langsung menyala dengan status: `TERLISENSI PERMANEN (AKTIF)` tanpa memerlukan input ulang Serial Number ataupun koneksi internet.
