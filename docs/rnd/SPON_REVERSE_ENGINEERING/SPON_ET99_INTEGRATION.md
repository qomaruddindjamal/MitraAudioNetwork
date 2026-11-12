# SPON FT_ET99 DONGLE INTEGRATION & TOKEN ANALYSIS
**Target:** `FT_ET99_API.dll` & Hardware Dongle Protection  
**Pabrikan Hardware:** Feitian Technologies Co., Ltd. (ePass ET99 USB Token)  

---

## 1. INTEGRASI API ET99

Biner `IPNBS_Server.exe` mengimpor fungsi berikut dari `FT_ET99_API.dll`:
- `et_FindToken`: Mencari keberadaan USB hardware dongle yang terpasang pada port USB host.
- `et_OpenToken`: Membuka sesi komunikasi aman dengan smart card mikro-kontroler token.
- `et_CloseToken`: Menutup sesi komunikasi dongle.

## 2. DONGLE SEBAGAI MASTER HARDWARE KEY
- Jika dongle ET99 fisik terdeteksi saat server pertama kali dijalankan, sistem secara otomatis mengaktifkan seluruh fitur broadcast komersial tanpa menuntut serial number dari berkas INI.
- Ketiadaan dongle fisik mengalihkan alur eksekusi ke pembacaan kunci teks di `IPNBS_Server.ini`.
- Oleh karena itu, server SPON memiliki dua mode otorisasi:
  1. **Hardware Mode:** Memerlukan USB Dongle ET99.
  2. **Software Registration Mode:** Memerlukan Serial Number 29 karakter yang valid.
