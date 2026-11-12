# SPON SERIAL NUMBER VALIDATION FLOW & REVERSE ENGINEERING
**Target Binary:** `IPNBS_Server.exe`  
**Toolchain:** NSA Ghidra 12.1.4 PUBLIC, MitraRND.py  

---

## 1. ALUR LENGKAP VALIDASI LISENSI STARTUP

```
   [Inisialisasi IPNBS_Server.exe]
                 |
                 v
   [RVA 0x00418F62: et_FindToken()]  <--- Cek Token Dongle ET99
                 |
       +---------+---------+
       |                   |
   (Token Ada)        (Token Tidak Ada / Gagal)
       |                   |
       |                   v
       |         [RVA 0x00418F77: Lanjut ke Registrasi Serial]
       |                   |
       |                   v
       |         [RVA 0x004190DC: GetPrivateProfileStringA]
       |         File: IPNBS_Server.ini, Key: SerialNumber
       |                   |
       |                   v
       |         [RVA 0x004190FE: Fungsi 0x00439360]
       |         - Cek Panjang: 29 Karakter (0x1D)
       |         - Cek Tanda Hubung '-' di indeks 5, 10, 15, 20
       |         - Panggil MD5 Hash Checksum (0x0043DB00)
       |                   |
       |             +-----+-----+
       |             |           |
       |          (Cocok)     (Tidak Cocok / Kosong)
       |             |           |
       |             |           v
       |             |    [RVA 0x004183F7: Error 0x9C42]
       v             v
   [Flag Lisensi Berhasil Diaktifkan: offset esi+0x1D640]
```

## 2. SPESIFIKASI FORMAT KUNCI
- Pola: `XXXXX-XXXXX-XXXXX-XXXXX-XXXXX` (5 segmen $\times$ 5 karakter alfanumerik = 29 karakter).
- Kunci historis referensi yang pernah diingat pengguna (`64E8A-A8F0C-2B1EC-BC7DC-0B05`) hanya memiliki 28 karakter (segmen kelima hanya 4 karakter: `0B05`). Hal ini secara otomatis ditolak oleh instruksi validasi panjang `cmp dword ptr [eax-8], 1Ch` / `1Dh`.
- **Status Otoritas:** Karena rumus hashing MD5 vendor bersifat rahasia dan proprietary, pembuatan serial secara sembarangan dilarang keras.
