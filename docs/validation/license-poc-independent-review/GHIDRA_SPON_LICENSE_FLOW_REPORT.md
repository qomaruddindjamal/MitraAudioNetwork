# GHIDRA SPON LICENSE FLOW & BINARY INVESTIGATION REPORT
**Tool:** NSA Ghidra 12.1.4 PUBLIC, Temurin JDK 21 LTS  
**Target:** `C:\IPNBS\analysis_work\IPNBS_Server.exe` & `FT_ET99_API.dll`  
**Status:** CONFIRMED

---

## 1. Alur Verifikasi Lisensi Biner Legacy `IPNBS_Server.exe`

Investigasi disassembler dan dekompiler Ghidra memetakan alur startup lisensi pada modul `0x00418E00` - `0x00419200`:

```
[Start IPNBS_Server.exe]
           |
           v
[RVA 0x00418F62: Call 0x00499A02 (et_FindToken in FT_ET99_API.dll)]
           |
     +-----+-----+
     |           |
 (Token Ada)  (Token Tidak Ada / Gagal)
     |           |
     |           v
     |   [RVA 0x00418F77: Lanjut ke Registrasi SerialNumber]
     |           |
     |           v
     |   [RVA 0x00418F8D: Call 0x0043C190 (GetModuleFileNameA)]
     |   Membentuk path direktori kerja: "C:\IPNBS\IPNBS_Server.ini"
     |           |
     |           v
     |   [RVA 0x004190DC: Call 0x004A9138 (GetPrivateProfileStringA)]
     |   Section: "BASE" (0x4C4868)
     |   Key:     "SerialNumber" (0x4C4870)
     |   File:    "IPNBS_Server.ini" (0x4C4880)
     |           |
     |           v
     |   [RVA 0x004190FE: Call 0x00439360 (Verifikasi Format SN 29 Karakter)]
     |           |
     |     +-----+-----+
     |     |           |
     |  (Cocok)     (Tidak Cocok / Kosong)
     |     |           |
     |     |           v
     |     |    [RVA 0x004183F7: Set Error Flag 0x9C42]
     |     v
     +----> Flag Stasiun Aktif (offset esi+0x1D640)
```

---

## 2. Struktur Verifikasi Serial Number Legacy (Fungsi `0x00439360`)

1. **Pemeriksaan Panjang String:**
   - Instruksi `cmp dword ptr [eax-8], 1Ch` memeriksa panjang string harus tepat `0x1C` (28 karakter) atau `0x1D` (29 karakter).
2. **Pemeriksaan Delimiter Tanda Hubung (`-`):**
   - Karakter pada indeks 5, 10, 15, dan 20 diverifikasi bernilai ASCII `0x2D` (`-`).
   - Format: `5 Karakter` - `5 Karakter` - `5 Karakter` - `5 Karakter` - `5 Karakter`.
3. **Inisialisasi Hashing Kriptografis MD5 (Fungsi `0x0043DB00`):**
   - Ditemukan inisialisasi state MD5 klasik:
     - `A = 0x67452301`
     - `B = 0xEFCDAB89`
     - `C = 0x98BADCFE`
     - `D = 0x10325476`
   - Hash dihitung atas kombinasi segmen serial number dan dibandingkan dengan checksum blok terakhir.

---

## 3. Temuan Kritis: Mengapa ET99 Bukan Kebutuhan Mutlak
- Fungsi `et_FindToken` hanya dipanggil **sekali** saat startup server untuk mendeteksi apakah dongle master terpasang.
- Jika dongle tidak ada, fungsi membaca `IPNBS_Server.ini` kunci `SerialNumber`. Jika nilai serial valid, server tetap berjalan dalam mode fungsional tanpa dongle fisik.
