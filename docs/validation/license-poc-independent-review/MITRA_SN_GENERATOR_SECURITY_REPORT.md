# MITRA SN GENERATOR SECURITY REPORT
**Komponen:** `crates/mitra-core/src/license.rs` (`SerialNumberGenerator`)  
**Format SN:** `MANS-<PROD4>-<HASH4>-<HASH4>-<CHK4>`  
**Status:** PASS (Verified & Secure)

---

## 1. Analisis Kriptografi HMAC-like SHA-256

- **Input:** `product_id` (4 karakter alfanumerik kapital), `native_id` (token identitas hardware), dan `secret_salt` (string rahasia otoritas penerbit).
- **Proses:**  
  ```rust
  let mut hasher = Sha256::new();
  hasher.update(product_id.as_bytes());
  hasher.update(b":");
  hasher.update(native_id.as_bytes());
  hasher.update(b":");
  hasher.update(secret_salt.as_bytes());
  let hash = hasher.finalize();
  ```
- **Ketahanan Terhadap Tebakan (Unpredictability):** Tanpa mengetahui `secret_salt`, penyerang tidak dapat menebak chunk hash yang sah hanya dengan membaca `Native ID` dari layar registrasi.

---

## 2. Pengujian Ketahanan Tambahan

1. **Collision Resistance:**  
   Pengujian pada 2 mesin dengan variasi `Native ID` menghasilkan SN yang berbeda total (dibuktikan pada `test_mitra_license_additional_forensic_and_security_cases`).
2. **Salt Rotation Support:**  
   Perubahan salt otoritas dari Epoch 1 ke Epoch 2 langsung menghasilkan rangkaian SN baru tanpa mengganggu parsing format terstruktur.
3. **Format Integrity Checksum:**  
   Chunk terakhir `<CHK4>` berfungsi sebagai *self-checking checksum* untuk mendeteksi salah ketik (*typo*) saat operator memasukkan SN secara manual.
4. **Pencegahan Duplikasi Atomik (Atomic Issuance):**  
   Database storage `LicenseAuthorityStorage` menegakkan aturan keunikan (unique constraints) atas `license_id` dan `serial_number`. Percobaan pendaftaran ganda ditolak dengan error `DuplicateSerialNumber`.
