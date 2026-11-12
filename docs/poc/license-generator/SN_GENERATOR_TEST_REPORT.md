# SN GENERATOR TEST REPORT
**Proyek:** Mitra Audio Network System  
**Modul:** `SerialNumberGenerator`  
**Status:** PASS (100% Skenario Pengujian Lulus)

---

## 1. Spesifikasi Format Serial Number (SN)

Format Serial Number yang dibangkitkan oleh sistem dirancang agar mudah dibaca, diketikkan oleh operator stasiun, dan memuat informasi produk serta validasi integritas checksum:

```text
MANS - <PROD4> - <HASH4> - <HASH4> - <CHK4>
(4B)    (4B)      (4B)      (4B)      (4B)   => Total 24 karakter (termasuk 4 tanda hubung '-')
```

1. **Prefix Organisasi:** Selalu `MANS` (*Mitra Audio Network System*).
2. **Product Identifier:** 4 huruf alfanumerik pertama dari `product_id` (misal `MITR` untuk `MITRA-AUDIO-SERVER`).
3. **Hardware Binding Chunk 1:** 4 karakter hex diambil dari byte `0..1` HMAC-like SHA-256 hash.
4. **Hardware Binding Chunk 2:** 4 karakter hex diambil dari byte `2..3` HMAC-like SHA-256 hash.
5. **Integrity Checksum:** 4 karakter hex diambil dari byte `4..5` HMAC-like SHA-256 hash.

---

## 2. Hasil Pengujian Unit

Pengujian dieksekusi pada `crates/mitra-core/tests/core_tests.rs`:

| Test Case | Deskripsi Pengujian | Hasil Aktual | Status |
|---|---|---|:---:|
| **TC-SN-01** | Pembangkitan SN dengan Product ID dan Native ID valid | Menghasilkan SN valid: `MANS-MITR-xxxx-xxxx-xxxx` | **PASS** |
| **TC-SN-02** | Validasi pola format (`validate_sn_format`) | Panjang 24 karakter, 5 segmen terpisah tanda hubung | **PASS** |
| **TC-SN-03** | Determinisme hash | Input yang sama menghasilkan SN yang persis sama | **PASS** |
| **TC-SN-04** | Uniqueness atas variasi Native ID | Penggantian Native ID menghasilkan SN yang sama sekali berbeda | **PASS** |
| **TC-SN-05** | Penolakan input Product ID kosong | Mengembalikan `Err(LicenseError::ProductMismatch)` | **PASS** |
| **TC-SN-06** | Penolakan input Native ID kosong | Mengembalikan `Err(LicenseError::InvalidNativeIdFormat)` | **PASS** |

---

## 3. Catatan Keamanan SN
- Serial Number adalah **token identitas registrasi**, bukan pengganti tanda tangan kriptografi.
- Memasukkan Serial Number ke dalam aplikasi klien tanpa menyertakan berkas signed license bertanda tangan Ed25519 **tidak akan mengaktifkan aplikasi**.
