# AUDIT KEAMANAN LISENSI & ISOLASI KRIPTOGRAFIS
**Dokumen:** `SECURITY_REVIEW.md`  
**Proyek:** Mitra Audio Network System  
**Standar Keamanan:** Asymmetric Ed25519 (RFC 8032) & Keyed HMAC-SHA256  
**Status Evaluasi:** PASS (Zero Critical Vulnerabilities)

---

## 1. Tinjauan Aspek Keamanan Kriptografi

Sistem lisensi Mitra Audio Network System dirancang dengan prinsip pertahanan berlapis (*Defense-in-Depth*):

1. **Keamanan Asimetris (Ed25519):**
   - Penandatanganan lisensi menggunakan kurva eliptik Ed25519 yang terbukti kebal terhadap serangan kebocoran nonce dan tahan terhadap serangan timing channel.
   - Sisi aplikasi pelanggan **sama sekali tidak memiliki kemampuan menerbitkan lisensi sendiri**, karena hanya dibekali 32-byte public key.
2. **Serial Number Deterministik Terikat Hardware (HMAC-SHA256):**
   - Serial Number dibangkitkan dari kombinasi:
     `Sha256(ProductID + ":" + NativeID + ":" + AuthoritySalt)`.
   - Tanpa mengetahui `AuthoritySalt` rahasia vendor, pihak ketiga tidak dapat merekayasa Serial Number yang valid untuk suatu mesin.
   - Serial Number diformat menjadi 24 karakter dengan chunk checksum khusus (`MANS-<PROD>-<HASH>-<HASH>-<CHKS>`) untuk mencegah kesalahan ketik manual oleh pengguna.
3. **Serial Number Bukan Bukti Kriptografi Tunggal:**
   - Aplikasi pelanggan secara eksplisit dirancang agar **TIDAK DAPAT** diaktifkan hanya dengan memasukkan teks Serial Number semata.
   - Aktivasi mewajibkan verifikasi tanda tangan digital 64-byte yang sah atas seluruh isi payload kanonikal.
4. **Offline Independent Verification:**
   - Klien tidak melakukan panggilan balik jaringan (*no telemetry, no phone-home*) saat memvalidasi lisensi. Hal ini menjamin privasi penuh dan operasional 100% stabil pada jaringan audio tertutup (*air-gapped broadcast network*).

---

## 2. Analisis Model Ancaman (Threat Modeling)

| Skenario Ancaman | Vektor Serangan | Mekanisme Pertahanan Mitra Audio | Hasil Uji Keamanan |
|---|---|---|---|
| **Penyalinan Lisensi (Cloning)** | Menyalin file `.lic` dari satu server ke server lain | Verifikator memeriksa kesesuaian Native ID mesin lokal terhadap string `native_id` pada payload | **Ditolak Seketika** (`HardwareMismatch`) |
| **Penyuntingan Payload** | Mengubah nama produk, ID lisensi, atau hak fitur secara teks | Merusak validitas tanda tangan kriptografi Ed25519 | **Ditolak Seketika** (`InvalidSignature`) |
| **Pemalsuan Serial Number** | Membuat Serial Number acak atau format lama SPON | Serial Number diuji format dan dicocokkan dengan payload lisensi yang ditandatangani | **Ditolak Seketika** (`InvalidSignature` / Format Gagal) |
| **Pencabutan Lisensi (Revocation)** | Lisensi yang telah dibatalkan dicoba dipasang ulang | Klien memeriksa daftar ID lisensi tercabut sebelum memberikan hak operasional | **Ditolak Seketika** (`LicenseRevoked`) |
| **Dekompilasi Klien** | Membongkar biner klien untuk mencari kunci privat | Biner klien hanya memuat public verifying key (32 bytes) | **Aman** (Tidak ada materi rahasia yang dapat diekstraksi) |

---

## 3. Penegasan Status Kunci Pengujian (Test Keys Only)

> [!WARNING]
> Kunci privat yang digunakan dalam implementasi Proof of Concept (PoC) ini adalah **Kunci Pengujian Sintetis (Synthetic Test Key)** berbasis seed statis `[0x55; 32]`.
> Kunci ini secara sengaja digunakan agar seluruh pengujian regresi otomatis dapat berjalan secara deterministik.
> 
> Saat sistem ini dinaikkan ke tingkat produksi komersial, kunci privat penandatangan wajib dibangkitkan secara aman menggunakan HSM (*Hardware Security Module*) atau KMS (*Key Management Service*) internal vendor dan tidak boleh disimpan di dalam kode sumber aplikasi apapun.
