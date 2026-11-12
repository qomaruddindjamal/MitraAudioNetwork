# WINDOWS NATIVE ID STABILITY & LICENSING REACTIVATION POLICY
**Project:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Area:** Hardware Identity Resolution, Cryptographic Binding & Tolerance Policy  
**Status:** PASS

---

## 1. Executive Summary
Sistem lisensi modern Mitra Audio Network System meninggalkan model usang USB Dongle (ET99) legacy yang rentan terhadap emulasi memori dan kegagalan fisik port USB. Penggantinya adalah **Native Hardware Identity (Native ID)** yang dikombinasikan dengan verifikasi tanda tangan kriptografi Ed25519.

Laporan ini memvalidasi:
1. Stabilitas Native ID terhadap restart, pembaruan minor, dan fluktuasi sementara.
2. Kebijakan toleransi berbobot (weighted similarity scoring) saat komponen perangkat keras diubah.
3. Prosedur Grace Period dan kebijakan reaktivasi lisensi.

---

## 2. Struktur Token Hardware & Bobot Penilaian (Weighted Scoring)

Mitra Windows Adapter mengumpulkan 3 komponen identitas perangkat keras fundamental:

| Komponen Hardware | Deskripsi | Bobot Skor | Sifat Toleransi |
|---|---|:---:|---|
| **Motherboard UUID** | Serial BIOS/UEFI SMBIOS | **40%** | Sangat statis, identitas mesin utama. |
| **CPU Processor ID** | CPUID instruction feature string | **35%** | Statis, berubah saat penggantian soket prosesor. |
| **System Drive Serial** | Serial fisik media boot `C:\` | **25%** | Dinamis (bisa diganti saat upgrade SSD). |
| **Total** | | **100%** | |

---

## 3. Matriks Keputusan & Ambang Batas (Thresholds)

Evaluasi kecocokan token dilakukan dengan fungsi `WindowsHardwareIdProvider::compute_similarity_score`:

```
+---------------------------------------------------------------------------------+
| Similarity Score | Status Lisensi    | Aksi Sistem                              |
+------------------+-------------------+------------------------------------------+
| >= 75%           | VALID (Auto-Pass) | Operasi normal tanpa gangguan user.      |
|                  |                   | Contoh: Upgrade SSD C:\ (skor 75%).      |
+------------------+-------------------+------------------------------------------+
| 50% - 74%        | WARNING           | Memulai Grace Period 7 hari. Notifikasi  |
|                  | (Grace Period)    | peringatan kepada admin stasiun untuk    |
|                  |                   | melakukan konfirmasi reaktivasi.         |
|                  |                   | Contoh: Upgrade CPU prosesor (skor 65%). |
+------------------+-------------------+------------------------------------------+
| < 50%            | INVALID           | Lisensi dicabut seketika. Operasi audio  |
|                  | (Revoked)         | diblokir. Wajib reaktivasi resmi.        |
|                  |                   | Contoh: Kloning disk ke PC baru (25%).   |
+------------------+-------------------+------------------------------------------+
```

---

## 4. Isolasi Versi Kebijakan (Policy Versioning)
Jika algoritma pengumpulan token diperbarui di masa mendatang (misalnya dari `WIN-HWID-V1` ke `WIN-HWID-V2`), hash Native ID diisolasi secara kriptografis menggunakan domain separation (`Sha256(policy_version : token_data)`).
Perubahan versi kebijakan tidak membatalkan lisensi yang ada secara otomatis sebelum migrasi terkoordinasi.

---

## 5. Bukti Hasil Uji Unit
Telah diverifikasi pada `crates/mitra-adapter-windows/tests/windows_tests.rs`:
```
test test_windows_hardware_id_hash ... ok
test test_windows_native_id_stability_and_similarity_scoring ... ok
```
- Determinisme 100% pada restart/reboot berulang (SHA-256 len 64 identik).
- Penggantian System Drive Serial menghasilkan skor 75% -> Auto-pass valid.
- Penggantian CPU menghasilkan skor 65% -> Grace period aktif.
- Kloning SSD ke motherboard dan CPU berbeda menghasilkan skor 25% -> Revocation berhasil.
- Domain separation `WIN-HWID-V1` vs `WIN-HWID-V2` menghasilkan hash berbeda secara kriptografis.

## 6. Kesimpulan
Mekanisme Native ID Windows terbukti stabil, deterministik, adil terhadap penggantian komponen pemeliharaan (SSD upgrade), dan tegas dalam mencegah kloning lisensi. Status: **PASS**.
