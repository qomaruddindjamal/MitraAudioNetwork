# AUDIT PEMISAHAN PAKET DISTRIBUSI (VENDOR VS CUSTOMER)
**Dokumen:** `PACKAGE_SEPARATION_AUDIT.md`  
**Proyek:** Mitra Audio Network System  
**Direktori Distribusi:** `C:\IPNBS\dist\`  
**Status Audit:** PASS (Pemisahan Mutlak Terbukti)

---

## 1. Tujuan Audit

Audit ini dilakukan untuk membuktikan kepatuhan mutlak terhadap aturan arsitektur keamanan:
1. Paket aplikasi pelanggan **TIDAK BOLEH** memuat biner SN Generator, source code GUI generator, kunci privat penandatangan Ed25519, berkas konfigurasi otoritas, maupun basis data penerbitan vendor.
2. Paket internal vendor didistribusikan secara terisolasi khusus untuk staf berwenang.

---

## 2. Struktur Pohon Berkas Paket Distribusi Aktual

Hasil inspeksi pohon berkas pada direktori `C:\IPNBS\dist`:

```text
C:\IPNBS\dist\
├── customer/
│   └── MitraAudio-Network-System/
│       └── MitraAudio-Network-System.exe   (266,240 bytes)
└── vendor/
    └── MitraAudio-SN-Generator/
        └── MitraAudio-SN-Generator.exe     (330,752 bytes)
```

### Rincian Hash SHA-256 Biner:
- **`MitraAudio-SN-Generator.exe` (Vendor):**  
  `6623769A4146E539BBD0252729EE255A2DE25B34E6DE9EC33D31ABA631BC7103`
- **`MitraAudio-Network-System.exe` (Customer):**  
  `DF55901EBC2E7568A7AF190415197060C15F38DFC066CD1FEFE0A8CB3E8F4874`

---

## 3. Hasil Inspeksi Biner & Tabel Impor (`dumpbin.exe`)

Inspeksi biner dilakukan terhadap paket pelanggan `MitraAudio-Network-System.exe` menggunakan Microsoft Visual Studio Binary Dump Tool:

1. **Inspeksi String & Simbol:**
   - Tidak ditemukan fungsi atau simbol `SigningKey`, `sign`, `generate_sn`, atau `authority_salt`.
   - Hanya ditemukan struktur verifikator publik: `from_public_key_bytes`, `verify_mitra_license`, dan konstanta kunci publik 32-byte `VENDOR_PUBLIC_KEY_BYTES`.
2. **Inspeksi Dependensi DLL Windows:**
   - Klien hanya mengimpor fungsi dasar sistem Windows:
     - `KERNEL32.dll` (Memory allocation, module loader).
     - `USER32.dll` (Windows message loop, window controls, clipboard).
     - `GDI32.dll` (Color brushes, text painting).
3. **Ketiadaan Komponen Tambahan:**
   - Tidak ada file `.ini`, `.db`, atau berkas rahasia yang disertakan di dalam direktori `dist/customer/`.

---

## 4. Evaluasi Toolchain Pembuat Installer (Setup Compiler)

Sesuai instruksi Section 7, dilakukan audit ketersediaan compiler installer pada lingkungan komputasi saat ini:
- Compiler Inno Setup (`iscc.exe`): Tidak terpasang pada PATH sistem.
- WiX Toolset (`wix.exe`): Tidak terpasang pada PATH sistem.

**Keputusan Distribusi PoC:**  
Sesuai arahan teknis, karena compiler installer pihak ketiga belum diinstal, distribusi disediakan dalam bentuk **Standalone Executable Portable Packages** pada direktori `dist/vendor/` dan `dist/customer/`. Hal ini memungkinkan pengujian langsung dan instan oleh User tanpa perlu menjalankan wizard instalasi yang memodifikasi registry sistem.
