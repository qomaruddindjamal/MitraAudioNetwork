# SPON INSTALLER V3.3.0 FORENSIC REPORT
**Target:** `C:\Tools\SetupIPNBS_V3.3.0.exe`  
**Ukuran:** 6,292,807 Bytes (~6.00 MB)  
**SHA-256:** `3BC466FB6004F5AC297FC7173B14C38F7380AA8F6B62FA53082D18415C8A82B8`  
**Status:** CONFIRMED

---

## 1. Identifikasi Teknologi & Metadata Installer

- **Engine:** **Setup Factory 9.0.0.0** (Indigo Rose Corporation).
- **Internal Name:** `suf_launch.exe` (Setup Application).
- **Arsitektur:** PE32 x86 GUI (Subsystem Windows GUI 5.01 / XP).
- **Kompilasi Timestamp:** `Fri Jan 7 23:05:53 2011` (Linker version 10.00).
- **Struktur Section:**
  - `.text`: Code execution RVA `0x1000` - `0x66F7`
  - `.rdata`: Read-only data RVA `0x7000` - `0x9E81`
  - `.data`: Initialized data RVA `0xA000` - `0xB967`
  - `.rsrc`: Resource RVA `0xC000` - `0x12D6F` (memuat metadata Setup Factory)
  - `.reloc`: Relocations RVA `0x13000` - `0x1408F`
- **Data Payload Setup:** Mulai dari offset `0x15000` hingga akhir berkas (ukuran ~6.2 MB) memuat arsip terkompresi dari file-file instalasi IPNBS.

---

## 2. Alur Ekstraksi & Eksekusi Installer

Berdasarkan string yang ditemukan pada RVA `0x4071C8` - `0x407480`:
```text
0x4071c8: Could not find setup size
0x4072b8: Failed to read setup engine
0x4072d4: Unable to open setup file
0x4073f8: irsetup.exe
0x407408: Could not determine a temp directory name. Try running setup.exe /T:<Path>
0x40745c: %s\irsetup.exe
0x407480: Could not start the setup
```

1. Stub loader mengekstrak runtime setup utama `irsetup.exe` dan berkas script konfigurasi Lua (`setup.dat`) ke temporary directory Windows.
2. `irsetup.exe` dieksekusi secara otomatis untuk menampilkan antarmuka wizard instalasi Setup Factory.
3. Seluruh berkas diekstrak ke target instalasi default: `C:\IPNBS`.

---

## 3. Investigasi Ketergantungan Dongle / Lisensi pada Installer
- **Pemeriksaan String:** Tidak ditemukan string `ET99`, `Feitian`, `dongle`, atau `token` pada stub installer.
- **Pemeriksaan Dialog:** Installer Setup Factory versi 9.0 mendistribusikan berkas biner secara langsung tanpa mewajibkan pengguna menancapkan dongle USB fisik selama proses penyalinan berkas berlangsung.
