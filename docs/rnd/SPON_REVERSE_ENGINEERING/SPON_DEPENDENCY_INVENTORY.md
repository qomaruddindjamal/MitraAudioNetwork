# SPON DEPENDENCY INVENTORY & RUNTIME MAPPING
**Engine:** `MitraRND.py`  
**Host Environment:** Windows 10/11 x64  

---

## 1. DAFTAR DEPENDENCY RUNTIME

Biner legacy SPON memerlukan sejumlah pustaka pendukung untuk eksekusi yang stabil:

1. **OLE DB Jet 4.0 Components:**
   - `msjet40.dll`, `mswstr10.dll`, `msjter40.dll`, `msjint40.dll` (Tersedia secara bawaan pada Windows 32-bit subsystem `SysWOW64`).
2. **Microsoft Visual C++ Runtime (Legacy):**
   - `MSVCRT.DLL`, `MFC42.DLL`.
3. **Komponen ActiveX / OCX:**
   - `MSCOMM32.OCX` (Diperlukan untuk komunikasi port serial COM jika diaktifkan).
4. **Third-Party Compression & Utility:**
   - `UNZDLL.DLL` (Pustaka unzipping arsip musik MP3).
   - `lua5.1.dll` (Scripting engine internal).
   - `xcacls.exe` (Utilitas penyesuaian hak akses file NTFS Windows).
5. **Pustaka Enkripsi Khusus SPON:**
   - `DBSEnc.dll`, `DBSEnc1.dll`, `DBSEnc2.dll`, `DBSEnc3.dll`, `DBSEnc4.dll`.
