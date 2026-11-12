# MITRA AUDIO LICENSE PAYLOAD SPECIFICATION
**Dokumen:** Spesifikasi Teknis Format Payload Lisensi Digital  
**Versi Skema:** `SCHEMA_VERSION = 1` (`V1`)  
**Tipe Lisensi:** `permanent` (Permanen Seumur Hidup Perangkat)

---

## 1. Struktur Field Payload

Payload lisensi `MitraLicensePayload` memiliki definisi bidang sebagai berikut:

| Nama Field | Tipe Data | Keterangan & Batasan | Contoh Nilai |
|---|---|---|---|
| `schema_version` | `u32` | Versi skema serialisasi payload (Wajib `1`) | `1` |
| `license_id` | `String` | Identifier unik lisensi yang diterbitkan Authority | `"LIC-2026-0001"` |
| `product_id` | `String` | Identifier aplikasi target yang mengikat lisensi | `"MITRA-AUDIO-SERVER"` |
| `native_id` | `String` | Identitas hardware unik mesin target | `"WIN-HWID-V1:MB-...:CPU-...:DISK-..."` |
| `serial_number` | `String` | Serial Number 24 karakter terikat | `"MANS-MITR-A1B2-C3D4-E5F6"` |
| `issued_at` | `u64` | Waktu UNIX timestamp saat lisensi diterbitkan | `1731370000` |
| `license_type` | `String` | Tipe lisensi (Harus bernilai `"permanent"`) | `"permanent"` |
| `entitlements` | `Vec<String>` | Daftar hak fitur aplikasi yang diizinkan | `["BASIC_AUDIO", "WEB_MANAGEMENT"]` |
| `issuer_id` | `String` | Identitas entitas otoritas penandatangan lisensi | `"MITRA-AUTHORITY-ROOT"` |

---

## 2. Aturan Kanonisasi Serialisasi (Canonical Representation)

Untuk menjamin tanda tangan digital kriptografi Ed25519 deterministik dan kebal terhadap perbedaan representasi teks (seperti spasi acak atau urutan array JSON yang berubah-ubah), diterapkan aturan **Kanonisasi Ketat**:

1. Array `entitlements` wajib diurutkan secara leksikografis (*alphabetical sort*).
2. Elemen array digabungkan dengan pemisah koma `,`.
3. Seluruh bidang digabungkan dengan pemisah *pipe* `|` dalam urutan tetap:
   ```text
   V<schema_version>|<license_id>|<product_id>|<native_id>|<serial_number>|<issued_at>|<license_type>|<sorted_entitlements>|<issuer_id>
   ```

### Contoh String Kanonikal:
```text
V1|LIC-2026-0001|MITRA-AUDIO-SERVER|WIN-HWID-V1:MB-UUID-4A71:CPU-INTEL-X86:DISK-SAMSUNG-SSD|MANS-MITR-A1B2-C3D4-E5F6|1731370000|permanent|BASIC_AUDIO,WEB_MANAGEMENT|MITRA-AUTHORITY-ROOT
```

String UTF-8 di atas diubah menjadi byte array (`Vec<u8>`) untuk ditandatangani menggunakan fungsi `SigningKey::sign(&canonical_bytes)`.

---

## 3. Format Berkas Lisensi Distribusi (`.lic`)

Paket lisensi yang didistribusikan ke pengguna (`SignedMitraLicense`) terdiri atas:
- Payload terkanonisasi (dalam format teks terstruktur atau JSON terkompresi).
- Tanda tangan Ed25519 sebesar 64 byte (direpresentasikan dalam format Hexadecimal 128 karakter atau Base64).
