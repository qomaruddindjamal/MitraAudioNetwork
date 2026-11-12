# SPON DATABASE (DBS.mdb) FORENSIC & SCHEMA ANALYSIS
**Target File:** `C:\IPNBS\DATABASE\DBS.mdb`  
**Format:** Microsoft Jet Database Engine 4.0 (Access MDB)  

---

## 1. STRUKTUR DAN AKSES DATABASE

1. **Penyedia Akses:**
   - `IPNBS_Server.exe` mengakses database menggunakan provider OLE DB: `Microsoft.Jet.OLEDB.4.0`.
   - Menggunakan enkripsi berkas data pendukung: `DBSEnc.dll`, `DBSEnc1.dll`, `DBSEnc.dat`.
2. **Entitas Data Utama:**
   - Tabel Terminal (`Terminals`): Menyimpan ID Terminal (1..1000), Alamat IP, Nama Terminal, Zona, dan Status Online.
   - Tabel Jadwal Siaran (`Tasks` / `Schedules`): Menyimpan waktu bel sekolah/pabrik harian.
   - Tabel Log Panggilan (`CallLogs`): Riwayat interkom antara mikrofon paging dan speaker terminal.
3. **Kebijakan Rekayasa & Preservation Lock:**
   - Database `DBS.mdb` asli TIDAK BOLEH dimodifikasi, ditimpa, atau dikonversi secara destruktif karena format Jet 4.0 diperlukan secara eksklusif oleh biner native `IPNBS_Server.exe`.
