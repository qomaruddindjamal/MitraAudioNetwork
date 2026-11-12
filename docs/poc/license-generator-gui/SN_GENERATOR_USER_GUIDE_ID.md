# PANDUAN PENGGUNA — MITRA AUDIO SN GENERATOR
**Aplikasi:** Mitra Audio SN Generator (Vendor Tool)  
**Dokumen:** `SN_GENERATOR_USER_GUIDE_ID.md`  
**Bahasa:** Bahasa Indonesia  
**Versi:** 1.0 (Proof of Concept)  
**Target Pengguna:** Operator Lisensi / Administrator Sistem Vendor Mitra Audio

---

## 1. Pendahuluan

Aplikasi **Mitra Audio SN Generator** adalah perangkat lunak desktop mandiri khusus internal vendor yang digunakan untuk:
1. Menghasilkan Serial Number (SN) resmi berdasarkan data perangkat keras unik pelanggan (*Native ID*).
2. Menerbitkan berkas lisensi digital permanen bertanda tangan kriptografi Ed25519 (`.lic`).
3. Mengelola riwayat penerbitan dan masa garansi dukungan penggantian serial number (1 tahun).

Aplikasi ini **TIDAK BOLEH** diberikan atau dibagikan kepada pelanggan umum.

---

## 2. Cara Menjalankan Aplikasi

Aplikasi telah dikompilasi secara mandiri (*standalone executable*) dan tidak membutuhkan instalasi tambahan.

1. Buka folder biner vendor:
   ```text
   C:\IPNBS\dist\vendor\MitraAudio-SN-Generator\
   ```
2. Klik dua kali pada berkas:
   ```text
   MitraAudio-SN-Generator.exe
   ```
3. Jendela aplikasi akan terbuka dengan resolusi standar `960 x 700` piksel dengan navigasi samping (*Sidebar*) berwarna biru dongker gelap (*Deep Navy*).

---

## 3. Langkah-demi-Langkah Penerbitan Lisensi Baru

### Langkah 1: Meminta Native ID dari Pelanggan
Minta pelanggan membuka aplikasi **Mitra Audio Network System** di komputernya dan mengklik tombol **"Copy Native ID"**. Pelanggan akan mengirimkan teks Native ID mereka (contoh format: `WIN-HWID-V1:MB-...:CPU-...:DISK-...`).

### Langkah 2: Membuka Menu SN Generator
1. Pada aplikasi vendor, klik tombol menu **"2. SN Generator"** di bilah kiri.
2. Klik tombol **"Paste Clipboard"** untuk menempelkan Native ID pelanggan ke dalam kolom input.
3. Klik tombol **"Generate Serial Number (SN)"**.
   - Sistem akan menghitung Serial Number 24 karakter dengan format baku `MANS-MITR-XXXX-XXXX-YYYY`.
4. Klik tombol **"Issue Signed Permanent License"**.
   - Sistem akan menandatangani paket lisensi dengan kunci privat Ed25519 dan mencatatnya ke dalam basis data otoritas lisensi.
   - Kotak dialog konfirmasi "Penerbitan Berhasil" akan muncul di layar.
5. Klik tombol **"Export Signed License File (.lic)"**.
   - Sistem akan menyimpan berkas lisensi ke direktori:
     `C:\IPNBS\mitra_license_<SERIAL_NUMBER>.lic`.
6. Klik tombol **"Copy SN"** untuk menyalin Serial Number ke clipboard.

### Langkah 3: Mengirimkan Data ke Pelanggan
Kirimkan dua komponen ini kepada pelanggan:
1. String **Serial Number** (misalnya via email atau chat resmi).
2. Berkas lisensi digital **`mitra_license_<SERIAL_NUMBER>.lic`**.

---

## 4. Cara Melakukan Uji Verifikasi Lisensi (Internal Testing)

Sebelum mengirimkan berkas lisensi kepada pelanggan, operator dapat menguji validitasnya secara mandiri:

1. Klik tombol menu **"4. License Verification"**.
2. Masukkan path berkas lisensi pada kolom yang tersedia, atau biarkan kosong untuk menggunakan berkas default, lalu klik **"Import .lic File"**.
3. Kolom Serial Number dan Native ID akan terisi secara otomatis dari berkas lisensi.
4. Klik tombol **"Verify Digital Signature & Hardware Binding"**.
5. Periksa kotak hasil verifikasi:
   - Jika valid, akan muncul teks: `STATUS: VALID (Permanent License)` dengan hasil `PASS`.
   - Jika signature rusak atau Native ID tidak cocok, akan muncul teks `STATUS: REJECTED` dengan rincian kegagalan.

---

## 5. Layanan Dukungan Penggantian SN (Masa 1 Tahun)

Kebijakan lisensi Mitra Audio Network System menetapkan:
- **Lisensi Perangkat Bersifat Permanen:** Aplikasi pelanggan yang sudah berlisensi akan tetap aktif seumur hidup tanpa masa kedaluwarsa.
- **Layanan Penggantian SN Gratis (1 Tahun):** Penggantian lisensi akibat kerusakan perangkat keras hanya dilayani gratis selama 365 hari sejak tanggal pembelian awal.

### Prosedur Pengecekan Dukungan:
1. Klik menu **"7. Support & Replacement"**.
2. Masukkan Serial Number lama pelanggan, lalu klik **"Lookup Record"**.
3. Sistem akan menampilkan rincian tanggal pembelian, batas tanggal dukungan, dan status kelayakan (*ELIGIBLE* atau *EXPIRED*).
4. Jika masa dukungan masih aktif, operator dapat memproses penerbitan lisensi baru untuk Native ID mesin pengganti.
