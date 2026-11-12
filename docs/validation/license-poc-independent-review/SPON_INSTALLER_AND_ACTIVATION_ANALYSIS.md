# SPON INSTALLER AND ACTIVATION ANALYSIS
**Konteks:** Perbedaan Antara Media Instalasi, Lokasi Aplikasi, dan Media Lisensi  
**Status:** CONFIRMED

---

## 1. Pemisahan Tegas Komponen Sistem

Berdasarkan hasil investigasi forensik, dilakukan pemisahan tegas antara komponen-komponen lingkungan sistem legacy:

```
+-----------------------------------------------------------------------------------------------+
| 1. MEDIA INSTALLER                                                                            |
|    - Berkas: C:\Tools\SetupIPNBS_V3.3.0.exe                                                   |
|    - Peran: Arsip distribusi & installer wizard (Setup Factory 9).                            |
|    - Status Lisensi: Bebas lisensi, tidak memerlukan dongle atau SN saat instalasi.           |
+-----------------------------------------------------------------------------------------------+
| 2. LOKASI INSTALASI APLIKASI RUNTIME                                                          |
|    - Direktori: C:\IPNBS\                                                                     |
|    - Eksekusi: IPNBS_Server.exe, IPNBS_Station.exe, IPNBSSDK.dll, dll.                        |
|    - Basis Data: C:\IPNBS\DATABASE\DBS.mdb                                                    |
|    - Konfigurasi: C:\IPNBS\IPNBS_Server.ini, IPNBS_Station.ini                                |
+-----------------------------------------------------------------------------------------------+
| 3. MEDIA PENYIMPANAN LISENSI                                                                  |
|    - File Lokal: Kunci "SerialNumber" di dalam C:\IPNBS\IPNBS_Server.ini                       |
|    - Token Fisik Opsional: Feitian ET99 USB Dongle (Diperiksa oleh et_FindToken)              |
|    - Hardware Binding: Motherboard / Drive Serial via Win32 API                               |
+-----------------------------------------------------------------------------------------------+
```

---

## 2. Analisis Pertanyaan Kunci Evaluasi

### A. Apakah aplikasi harus dijalankan dari USB?
> **TIDAK.** Aplikasi diinstal dan berjalan langsung dari hard disk sistem lokal (`C:\IPNBS`). USB hanya menjadi antarmuka port untuk dongle token Feitian ET99 opsional.

### B. Apakah instalasi membutuhkan dongle?
> **TIDAK.** Installer Setup Factory mengekstrak seluruh biner aplikasi tanpa memeriksa keberadaan USB dongle.

### C. Kapan Serial Number dimasukkan?
> **PASCA INSTALASI (SAAT APLIKASI DIJALANKAN).** Pengguna memasukkan Serial Number melalui antarmuka registrasi `IPNBS_Server.exe`, yang kemudian disimpan secara permanen di berkas `IPNBS_Server.ini`.

### D. Apakah Station dan SDK membutuhkan dongle?
> **TIDAK.** `IPNBS_Station.exe` dan `IPNBSSDK.dll` sama sekali tidak memiliki impor ke `FT_ET99_API.dll`.
