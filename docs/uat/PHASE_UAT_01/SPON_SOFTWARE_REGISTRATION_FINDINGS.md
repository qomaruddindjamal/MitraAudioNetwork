# SPON SOFTWARE REGISTRATION MODE FINDINGS (TASK 3)
**Target File:** `C:\IPNBS\IPNBS_Server.ini`  
**Seksi:** `[BASE]`  
**Kunci:** `SerialNumber`  
**Status Audit:** **READ-ONLY INSPECTION (PRESERVED)**  

---

## 1. PEMERIKSAAN BERKAS KONFIGURASI `IPNBS_Server.ini`

Pemeriksaan read-only terhadap berkas konfigurasi pada seksi `[BASE]`:

```ini
[BASE]
nSpy=0
nCom2=0
nCom=0
bShutDownEnable=0
wShutDownHour=0
wShutDownMinute=0
wShutDownSecond=0
bStepResumeVolume=0
nTaskPriority1=0
nTaskPriority2=1
nTaskPriority3=2
nTaskPriority4=3
SerialNumber=64E8A-A8F0C-2B1EC-BC7DC-0B05
```

## 2. POLA SERIAL YANG TERSIMPAN
- Nilai yang tersimpan: `64E8A-A8F0C-2B1EC-BC7DC-0B05`
- Analisis Panjang & Format:
  - Blok 1: `64E8A` (5 char)
  - Delimiter: `-` (1 char)
  - Blok 2: `A8F0C` (5 char)
  - Delimiter: `-` (1 char)
  - Blok 3: `2B1EC` (5 char)
  - Delimiter: `-` (1 char)
  - Blok 4: `BC7DC` (5 char)
  - Delimiter: `-` (1 char)
  - Blok 5: `0B05` (4 char)
  - Total Panjang: 28 karakter data buffer.
- Kesimpulan Teknis: Nilai historis ini tepat memenuhi panjang 28 byte yang diuji oleh instruksi `cmp dword ptr [eax-8], 1Ch`. Namun, karena string ini terikat pada Volume Serial perangkat keras stasiun masa lalu, pada mesin saat ini ia gagal pada tahap validasi MD5 `0x0043DB00` (Mismatch checksum mesin).

## 3. RESPON RUNTIME DAN PERBEDAAN ERROR
- **Jika Serial Kosong / Dihapus:** Fungsi langsung mendeteksi panjang 0 $\rightarrow$ Server mengaktifkan flag unactivated stasiun (Error 0x9C42).
- **Jika Format Salah (Panjang Bukan 28 Byte atau Delimiter Bukan `-`):** Instruksi `jne 0x004394FB` langsung melompat keluar tanpa mengeksekusi MD5 hashing.
- **Jika Format Tepat (28 Byte Pola 5-5-5-5-4) Tetapi Mesin Berbeda:** Fungsi mengeksekusi MD5, gagal mencocokkan digest dengan blok kelima, dan memicu error registrasi yang sama (Error 0x9C42).
