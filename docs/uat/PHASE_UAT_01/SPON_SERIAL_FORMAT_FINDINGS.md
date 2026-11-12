# SPON SERIAL FORMAT AUDIT & DISASSEMBLER FINDINGS (RVA 0x00439360)
**Execution Phase:** Phase UAT — Audit C  
**Target:** `C:\IPNBS\IPNBS_Server.exe`  
**Disassembler Target:** RVA `0x00439360` & `0x0043DB00`  

---

## 1. INVESTIGASI KEDALAMAN INSTRUKSI RVA 0x00439360

1. **Perhitungan Panjang String:**
   - Instruksi pembacaan memeriksa panjang string keseluruhan yang dimuat dari berkas `IPNBS_Server.ini` kunci `[BASE] -> SerialNumber`.
   - Instruksi `cmp dword ptr [eax-8], 1Ch` / `1Dh` memvalidasi bahwa panjang total adalah **28 atau 29 byte**.
2. **Karakter Pemisah Tanda Hubung (`-`):**
   - Karakter pada indeks 5, 10, 15, dan 20 diverifikasi bernilai ASCII `0x2D` (`-`).
   - Format wajib: `5 Karakter` - `5 Karakter` - `5 Karakter` - `5 Karakter` - `5 Karakter` (Total 29 karakter alfanumerik kapital dan delimiter).
3. **Verifikasi Checksum Lanjutan (Fungsi 0x0043DB00):**
   - Fungsi 0x00439360 **BUKAN** sekadar pemformat string; ia adalah *entry point validator lisensi*.
   - Fungsi ini memanggil rutinitas hashing MD5 (`0x0043DB00`) yang menghitung hash atas kombinasi atribut perangkat keras (Volume Serial disk) dengan 4 blok pertama serial, kemudian membandingkannya dengan blok ke-5 sebagai checksum terverifikasi.

## 2. STATUS SERIAL HISTORIS PENGGUNA
- **Serial Terdaftar:** `64E8A-A8F0C-2B1EC-BC7DC-0B05`
- **Hasil Pemeriksaan UAT:**
  - Segmen 1 (`64E8A`): 5 karakter
  - Segmen 2 (`A8F0C`): 5 karakter
  - Segmen 3 (`2B1EC`): 5 karakter
  - Segmen 4 (`BC7DC`): 5 karakter
  - Segmen 5 (`0B05`): **4 karakter (HANYA 4 CHAR, KURANG 1 CHAR)**
  - Total Panjang: 28 karakter.
- **Klasifikasi Resmi:**
  `UNVERIFIED_HISTORICAL_REFERENCE`
  *(Serial ini tidak pernah terbukti diterima oleh biner resmi SPON dan tidak boleh diperlakukan sebagai test vector resmi).*
