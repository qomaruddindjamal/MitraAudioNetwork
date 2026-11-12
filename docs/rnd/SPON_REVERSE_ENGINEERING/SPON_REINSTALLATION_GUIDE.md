# PANDUAN PEMULIHAN & INSTALASI ULANG NATIVE SPON (REINSTALLATION GUIDE)
**Installer Master:** `C:\Tools\SetupIPNBS_V3.3.0.exe`  
**Ukuran:** 6.303.472 bytes  
**SHA-256 Hash:** `3BC466FB6004F5AC297FC7173B14C38F7380AA8F6B62FA53082D18415C8A82B8`  

---

## 1. TAHAPAN PROSEDUR PEMULIHAN RESMI (RECOVERY WORKFLOW)

1. **Verifikasi Hash Master:**
   Pastikan hash SHA-256 berkas `SetupIPNBS_V3.3.0.exe` cocok 100% dengan baseline Preservation Lock.
2. **Pencadangan Konfigurasi Kerja:**
   Simpan salinan berkas konfigurasi lokal `IPNBS_Server.ini`, `IPNBS_Station.ini`, dan database `DBS.mdb`.
3. **Eksekusi Installer:**
   Jalankan `SetupIPNBS_V3.3.0.exe` dengan hak Administrator.
   - Lokasi instalasi default: `C:\IPNBS\`.
4. **Verifikasi Pustaka dan Registrasi OCX:**
   Jalankan registrasi komponen COM:
   ```cmd
   regsvr32 /s C:\IPNBS\MSCOMM32.OCX
   ```
5. **Konfigurasi Alamat IP & Port:**
   - Server IP: Atur pada subnet jaringan stasiun siaran lokal.
   - Port Kontrol: TCP 2046.
   - Port Audio Siaran: UDP Multicast 2048.
6. **Uji Fungsionalitas Operasional:**
   - Jalankan `IPNBS_Server.exe`.
   - Jalankan `IPNBS_Station.exe`.
   - Jalankan `MitraRND.py` untuk mengaudit kesiapan runtime.
