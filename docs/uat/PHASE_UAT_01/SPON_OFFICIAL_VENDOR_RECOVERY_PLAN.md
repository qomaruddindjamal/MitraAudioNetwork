# SPON OFFICIAL VENDOR RECOVERY PLAN & ROADMAP (TASK 4)
**Target Sistem:** IP Network Broadcasting System (IPNBS V3.3.0 / SPON)  
**Otoritas Asli:** Changsha SPON Communication Technology Co., Ltd. (SPON)  
**Tujuan Dokumen:** Jalur Pemulihan Lisensi Resmi Tanpa Merusak Validasi  

---

## 1. IDENTIFIKASI VENDOR & PERANGKAT
- **Pabrikan Sistem:** Changsha SPON Communication Technology Co., Ltd. (Sistem Siaran Audio IPNBS V3.3.0).
- **Penyedia Hardware Token:** Feitian Technologies Co., Ltd. (ePass ET99 USB Smart Card Token).
- **Arsitektur Lisensi Asli:**
  1. *Hardware Master Key:* USB Dongle Feitian ET99 yang memuat sertifikat pabrikan SPON.
  2. *Software Machine-Locked Key:* Serial Number 28 byte (pola 5-5-5-5-4) terikat Volume Serial Number disk lokal dan dihitung melalui private seed MD5 vendor.

---

## 2. PROSEDUR RESMI PEMULIHAN & TRANSFER LISENSI VENDOR

Untuk memulihkan sistem SPON secara sah saat migrasi perangkat keras:

### Langkah 1: Pengumpulan Data Mesin Target
Vendor SPON memerlukan data otentik perangkat target dari stasiun:
1. **Machine ID / Registration Code:** Kode heksadesimal yang ditampilkan pada antarmuka registrasi server SPON (diturunkan dari Volume Serial disk sistem).
2. **Nomor Seri Perangkat Keras / Invoice Pembelian:** Bukti kepemilikan unit stasiun siaran atau terminal audio (misal: seri 8000DA / 8000RM).
3. **Versi Master:** Installer resmi `SetupIPNBS_V3.3.0.exe`.

### Langkah 2: Pengajuan Re-aktivasi ke Vendor
Pengguna atau perwakilan teknis mengajukan permohonan ke saluran dukungan resmi SPON:
- Permohonan: *License Re-issuance due to Server Hardware Replacement*.
- Lampiran: Data Machine ID baru dan nomor kontrak/lisensi historis (`64E8A-A8F0C-2B1EC-BC7DC-0B05`).

### Langkah 3: Penerapan Kunci Resmi
1. Vendor menerbitkan string Serial Number 28-karakter baru yang sah.
2. String tersebut dimasukkan ke dalam `IPNBS_Server.ini` seksi `[BASE]` kunci `SerialNumber`.
3. Biner `IPNBS_Server.exe` memvalidasi kunci tersebut pada startup via RVA `0x00439360` dan `0x0043DB00`, mengaktifkan stasiun secara resmi (`NATIVE PASS`).

---

## 3. DAFTAR PERTANYAAN TEKNIS UNTUK DUKUNGAN VENDOR SPON
1. Apakah pabrikan SPON masih melayani penerbitan ulang Serial Number software untuk versi IPNBS V3.3.0?
2. Jika dongle fisik ET99 hilang atau rusak, apakah tersedia program penggantian dongle fisik dengan pasangan PID/VID yang sesuai?
3. Apakah SPON menyediakan API atau SDK berlisensi untuk mengotomatiskan pendaftaran stasiun baru tanpa entri INI manual?
