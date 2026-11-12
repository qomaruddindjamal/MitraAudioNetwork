# FEITIAN ET99 DEPENDENCY & ARCHITECTURE ANALYSIS
**Biner Terkait:** `FT_ET99_API.dll` (SHA-256: `7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE`)  
**Host Executable:** `IPNBS_Server.exe`  
**Status:** CONFIRMED

---

## 1. Analisis Impor Fungsi ET99 pada Seluruh Solusi

Pemeriksaan menyeluruh terhadap seluruh modul biner di workspace `C:\IPNBS`:

| File Biner | Mengimpor `FT_ET99_API.dll`? | Fungsi yang Diimpor | Frekuensi Pemanggilan |
|---|:---:|---|:---:|
| `IPNBS_Server.exe` | **YA** | Tepat 1 fungsi: `et_FindToken` | 1 kali saat startup (0x00418F62) |
| `IPNBS_Station.exe` | **TIDAK** | Nol (0) | Tidak ada |
| `IPNBSSDK.dll` | **TIDAK** | Nol (0) | Tidak ada |
| `IPNBS_TalkRecorder.exe`| **TIDAK** | Nol (0) | Tidak ada |
| `IPNBS_MP3Tools.exe` | **TIDAK** | Nol (0) | Tidak ada |

---

## 2. Karakteristik Fungsi `et_FindToken`

1. **Deklarasi C ABI:**  
   ```c
   DWORD et_FindToken(PID_STRUCT* pPid, DWORD* pCount);
   ```
2. **Perilaku Eksekusi di `IPNBS_Server.exe`:**
   - Server memanggil fungsi ini dengan pointer buffer PID dan pointer variabel count `dword ptr [esp+44h]`.
   - Jika `eax == 0` dan count > 0, server menyetel flag stasiun master berlisensi hardware (`mov dword ptr [esi+0x1D640], edi`).
   - Jika `eax != 0` (misal return code token not found `0x00000001` atau `0xFFFFFFFF`) atau count <= 0:
     - Eksekusi melompat ke `0x00418F77`.
     - Melanjutkan pembacaan berkas konfigurasi `IPNBS_Server.ini`.
     - Jika serial number valid ditemukan di INI, server tetap berjalan normal.

---

## 3. Kesimpulan Ketergantungan ET99
- Feitian ET99 **BUKAN persyaratan mutlak** agar perangkat lunak IPNBS dapat beroperasi.
- ET99 adalah opsi otentikasi alternatif *plug-and-play* bagi pelanggan yang tidak ingin memasukkan string Serial Number manual pada berkas INI stasiun.
- Penolakan terhadap dongle fisik pada arsitektur modern Mitra Audio adalah langkah yang tepat secara teknis, meningkatkan reliabilitas stasiun, dan menghilangkan ketergantungan pada rantai pasok mikrokontroler USB vendor asing.
