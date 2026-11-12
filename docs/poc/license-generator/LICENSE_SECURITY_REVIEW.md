# LICENSE SECURITY & CRYPTOGRAPHY REVIEW
**Proyek:** Mitra Audio Network System  
**Fokus:** Evaluasi Keamanan Kriptografi, Manajemen Kunci, dan Ketahanan Terhadap Serangan  
**Status:** SECURE & APPROVED

---

## 1. Analisis Kriptografi Asimetris Ed25519

Mitra Audio Network System mengadopsi standar **Ed25519 (Edwards-curve Digital Signature Algorithm)**:
- **Tingkat Keamanan Tinggi:** Memberikan tingkat keamanan setara ~128-bit simetris dengan kunci publik ringkas (32 byte) dan tanda tangan padat (64 byte).
- **Resisten Terhadap Side-Channel Attacks:** Tidak menggunakan percabangan kode berbasis rahasia (*secret-dependent branches*) atau *variable-time lookups*, menjadikannya tahan terhadap serangan *timing attack*.
- **Immunity Terhadap Signature Malleability:** Implementasi melalui crate terpercaya `ed25519-dalek` v2.2 dengan pemeriksaan kanonikasi ketat.

---

## 2. Pemisahan Lingkungan Kunci (Key Management Policy)

| Kunci | Lokasi Penyimpanan | Akses Klien | Risiko Kebocoran | Mitigasi |
|---|---|:---:|:---:|---|
| **Private Signing Key** | Lingkungan Server / HSM License Authority | **DILARANG KERAS** | Kritis (Bisa menerbitkan lisensi palsu) | Kunci privat tidak pernah disertakan dalam source code klien, biner aplikasi, atau repositori Git publik. Hanya kunci uji coba deterministik yang dipakai pada unit test. |
| **Public Verification Key** | Biner Aplikasi Klien (`mitra-core`) | Terdistribusi (Read-Only) | Nol | Kunci publik aman didistribusikan secara publik karena secara matematis tidak dapat diturunkan menjadi kunci privat. |

---

## 3. Ketahanan Terhadap Skenario Serangan Umum

1. **Serangan Kloning Hard Disk (Disk Cloning):**
   - *Vektor:* Menjiplak image hard disk dari PC berlisensi ke PC lain di jaringan.
   - *Pertahanan:* Native ID memeriksa kombinasi Motherboard BIOS UUID dan CPU Processor ID. Pada mesin kloning, `actual_native_id` berbeda sehingga verifier langsung menolak (`LicenseError::HardwareMismatch`).
2. **Serangan Manipulasi Berkas Lisensi (Tampering Attack):**
   - *Vektor:* Mengubah teks `license_id` atau menambahkan entitlement `"WEB_MANAGEMENT"` langsung pada file `.lic`.
   - *Pertahanan:* Serialisasi kanonikal mendeteksi perubahan sekecil apa pun; verifikasi tanda tangan Ed25519 gagal seketika (`LicenseError::InvalidSignature`).
3. **Serangan Pemalsuan Serial Number (Keygen Attack):**
   - *Vektor:* Mencoba membuat SN sendiri menggunakan rumus format.
   - *Pertahanan:* SN di-hash bersama *Authority Secret Salt* yang dirahasiakan, dan yang terpenting: **aplikasi tidak menganggap SN sebagai bukti lisensi**. Hanya berkas bertanda tangan privat yang diterima oleh verifier.
4. **Serangan Manipulasi Tanggal Sistem (System Clock Tampering):**
   - *Pertahanan:* Karena lisensi bersifat **permanen**, sistem klien tidak perlu melakukan validasi tanggal kedaluwarsa sistem secara terus-menerus yang rentan terhadap manipulasi jam CMOS.
