# SPON NATIVE COMPATIBILITY STATUS & INTEROPERABILITY MATRIX
**Execution Date:** 12 November 2026  
**Phase:** Phase UAT — Audit G  
**Target:** Evaluasi Kompatibilitas Dua Jalur (Hardware vs Software Registration)  

---

## 1. STATUS DUA JALUR VALIDASI NATIVE SPON

### A. Hardware Mode (USB Smart Card Dongle ET99)
- **Status Komponen Driver:** Pustaka `FT_ET99_API.dll` tersedia dan fungsional.
- **Status Perangkat Fisik:** **TIDAK TERPASANG (ABSENT)**. Port USB komputer fisik tidak memiliki token ET99.
- **Kebijakan Rekayasa:** Dilarang keras membuat emulator/dongle palsu yang memalsukan respons biner.
- **Status Evaluasi:** **BLOCKED (Requires Physical Hardware Token)**.

### B. Software Registration Mode (29-Character Serial Number)
- **Status Komponen Konfigurasi:** Berkas `IPNBS_Server.ini` memuat kunci `SerialNumber`.
- **Status Validasi Serial:** Kunci historis `64E8A-A8F0C-2B1EC-BC7DC-0B05` tidak valid (panjang 28 karakter, kurang 1 karakter).
- **Status Prosedur Penerbitan Resmi:** **TIDAK DITEMUKAN (UNVERIFIED)**. Vendor pabrikan SPON tidak menyediakan tool keygen atau API penerbitan kunci di lingkungan ini.
- **Kebijakan Rekayasa:** Dilarang menebak atau membuat algoritma palsu.
- **Status Evaluasi:** **BLOCKED — OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED**.

---

## 2. BUKTI YANG DIBUTUHKAN UNTUK MENCAPAI STATUS NATIVE PASS
Untuk mengubah status `BLOCKED` menjadi `NATIVE PASS`, diperlukan **salah satu** dari bukti otentik berikut:
1. Pemasangan fisik USB Hardware Token Feitian ET99 asli dari pabrikan SPON; ATAU
2. Dokumen spesifikasi resmi / SDK vendor SPON berlisensi sah yang memaparkan rumus seed MD5 dan prosedur penerbitan serial 29 karakter yang cocok dengan ID mesin stasiun.
