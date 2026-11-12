# MITRA AUDIO LICENSE ARCHITECTURE PoC
**Proyek:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Crate:** `crates/mitra-core` & `crates/mitra-adapter-windows`  
**Status:** DIVERIFIKASI (PASS)

---

## 1. Prinsip Arsitektur Lisensi

Sistem lisensi modern Mitra Audio Network System dibangun di atas prinsip pemisahan tanggung jawab (*Separation of Concerns*), kriptografi modern berbasis kurva eliptik Ed25519, dan integritas identitas perangkat keras (*Native Hardware Binding*).

Arsitektur ini sepenuhnya meniadakan ketergantungan pada dongle USB fisik (seperti Feitian ET99 legacy) yang rentan terhadap kerusakan mekanis dan pembajakan memori OS, serta menggantikan skema lisensi berbasis masa berlaku (*subscription/time-bomb*) dengan **Lisensi Permanen Berbasis Kelayakan Perangkat**.

---

## 2. Diagram Alir Arsitektur Lisensi

```
+---------------------------------------------------------------------------------------------------+
|                                  LINGKUNGAN LICENSE AUTHORITY (ISSUER)                            |
|                                                                                                   |
|  [Operator Input]                                                                                 |
|   - Product ID ("MITRA-AUDIO-SERVER")                                                            |
|   - Native ID ("WIN-HWID-V1:MB...:CPU...:DISK...")                                                |
|   - Entitlements (["BASIC_AUDIO", "WEB_MANAGEMENT"])                                              |
|            |                                                                                      |
|            v                                                                                      |
|   [SN Generator] ----------> Menghasilkan format terikat: MANS-MITR-xxxx-xxxx-xxxx                |
|            |                                                                                      |
|            v                                                                                      |
|   [Authority Storage] -----> Mencatat: LicenseID, SN, PurchaseDate, SupportUntil (+1 Tahun)       |
|            |                                                                                      |
|            v                                                                                      |
|   [License Issuer] --------> Membuat Payload Kanonikal Permanen                                   |
|   (Private Key Ed25519)      Menandatangani -> SignedMitraLicense (Payload + Signature 64B)       |
+------------|--------------------------------------------------------------------------------------+
             | (Distribusi Berkas .lic / Registrasi)
             v
+---------------------------------------------------------------------------------------------------+
|                                   LINGKUNGAN KLIEN / APLIKASI RUNTIME                             |
|                                                                                                   |
|  [Input Pengguna]                                                                                 |
|   - Serial Number (MANS-...)                                                                      |
|   - Berkas Lisensi (.lic)                                                                         |
|            |                                                                                      |
|            v                                                                                      |
|  [License Verifier Engine] <--- Menggunakan Public Verification Key (Tersimpan aman di biner)    |
|            |                                                                                      |
|            +---> 1. Verifikasi Ed25519 Signature atas Canonical Payload                          |
|            +---> 2. Validasi Product ID cocok dengan biner yang berjalan                          |
|            +---> 3. Validasi Native ID cocok dengan pembacaan Windows Adapter lokal               |
|            +---> 4. Validasi Status Revocation / Blacklist                                        |
|            +---> 5. Pemeriksaan Entitlement di Backend (contoh: WEB_MANAGEMENT)                   |
|            |                                                                                      |
|            v                                                                                      |
|  [Aplikasi Berjalan Penuh / Fitur Diaktifkan]                                                     |
+---------------------------------------------------------------------------------------------------+
```

---

## 3. Komponen-Komponen Utama

1. **Native ID Provider (`crates/mitra-adapter-windows/src/hw_windows.rs`):**
   - Mengumpulkan data identitas fisik komputer: Motherboard BIOS UUID, CPU Processor ID, System Drive Serial.
   - Menghasilkan token terstruktur berversi `WIN-HWID-V1`.
2. **SN Generator (`crates/mitra-core/src/license.rs`):**
   - Menerima Product ID, Native ID, dan Authority Salt rahasia.
   - Menghasilkan string SN 24 karakter berformat baku: `MANS-<PROD4>-<HASH4>-<HASH4>-<CHK4>`.
   - SN bertindak sebagai bukti registrasi komersial dan referensi pencarian database, **bukan** pengganti tanda tangan digital.
3. **License Issuer (`crates/mitra-core/src/license.rs`):**
   - Beroperasi hanya di server License Authority.
   - Memegang `SigningKey` Ed25519.
   - Menghasilkan `SignedMitraLicense` yang memuat payload terkanonisasi dan tanda tangan 64-byte.
4. **License Verifier (`crates/mitra-core/src/license.rs`):**
   - Berada di dalam aplikasi stasiun/server klien.
   - Hanya memegang `VerifyingKey` 32-byte (kunci publik).
   - Memvalidasi keaslian berkas lisensi dan memaksakan kepatuhan fitur (*entitlement enforcement*).
5. **License Authority Registry / Storage:**
   - Menyimpan audit log, tanggal pembelian, dan batas garansi dukungan 1 tahun (*Support Until*).
   - Menjamin tidak ada duplikasi SN atau License ID yang diterbitkan.
