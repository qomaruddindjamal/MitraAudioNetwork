# SPON MD5 & NATIVE ID VALIDATION FLOW REPORT (TASK 2)
**Target:** Alur Hashing Kriptografis MD5 & Identifikasi Mesin Native SPON  
**Verified SHA-256:** `7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9`  
**MD5 Engine Function:** RVA `0x0043DB00` (Raw File Offset `0x0003DB00`)  

---

## 1. BUKTI BINER INISIALISASI MD5 ENGINE PADA RVA 0x0043DB00

Inisialisasi konstanta standar RFC 1321 MD5 terbukti secara absolut pada raw offset `0x0003DB0F`:

```assembly
0x0043DB0F: C7 41 04 01 23 45 67    mov dword ptr [ecx+4],  0x67452301   ; State A
0x0043DB16: C7 41 08 89 AB CD EF    mov dword ptr [ecx+8],  0xEFCDAB89   ; State B
0x0043DB1D: C7 41 0C FE DC BA 98    mov dword ptr [ecx+12], 0x98BADCFE   ; State C
0x0043DB24: C7 41 10 76 54 32 10    mov dword ptr [ecx+16], 0x10325476   ; State D
```

## 2. DATA INPUT DAN PERAN VOLUME SERIAL NUMBER
- **Volume Serial Number:** Biner mengimpor `GetVolumeInformationA` dari `KERNEL32.DLL`. Nilai 32-bit volume serial drive sistem (`C:\`) diekstrak sebagai basis identitas mesin lokal.
- **Konstanta / Private Seed Pabrikan:**
  Sebelum hashing dilakukan, string volume serial digabungkan dengan segmen serial number dan sebuah string seed rahasia milik vendor pabrikan SPON (`0x004C56B8`).
- **Urutan Validasi:**
  1. Validasi awal: Pemeriksaan panjang (28 byte) dan tanda hubung pada RVA `0x00439360`.
  2. Jika format valid, baru memanggil rutinitas MD5 `0x0043DB00` untuk menghitung digest atas atribut mesin + seed vendor.
  3. Hash yang dihasilkan dibandingkan dengan segmen akhir serial number.
- **Kesimpulan Otoritas:** Karena nilai seed rahasia vendor berada di luar ruang lingkup publik, merekayasa kunci tanpa izin vendor adalah tindakan ilegal dan dilarang.
