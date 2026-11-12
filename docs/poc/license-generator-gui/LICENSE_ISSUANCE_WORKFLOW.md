# ALUR KERJA PENERBITAN LISENSI — MITRA AUDIO NETWORK SYSTEM
**Dokumen:** `LICENSE_ISSUANCE_WORKFLOW.md`  
**Proyek:** Mitra Audio Network System  
**Format Serial Number:** `MANS-<PROD4>-<HASH4>-<HASH4>-<CHK4>` (24 Karakter)  
**Format Berkas Lisensi:** Text Kanonikal Bertanda Tangan Ed25519 (`.lic`)

---

## 1. Diagram Alir Penerbitan & Aktivasi End-to-End

```mermaid
sequenceDiagram
    autonumber
    actor Customer as Pelanggan / Teknisi
    participant CustApp as Mitra Audio Network System (Client GUI)
    actor Vendor as Operator Lisensi Vendor
    participant GenApp as Mitra Audio SN Generator (Vendor GUI)
    participant AuthStorage as License Authority Storage

    Note over Customer,CustApp: 1. Tahap Pembacaan Hardware
    Customer->>CustApp: Buka Aplikasi Pelanggan
    CustApp->>Customer: Tampilkan Perangkat Native ID
    Customer->>CustApp: Klik "Copy Native ID"
    Customer->>Vendor: Kirimkan String Native ID

    Note over Vendor,GenApp: 2. Tahap Penerbitan di Sisi Vendor
    Vendor->>GenApp: Buka SN Generator GUI (Menu 2)
    Vendor->>GenApp: Tempelkan Native ID & Pilih Produk
    Vendor->>GenApp: Klik "Generate Serial Number"
    GenApp->>GenApp: Hitung HMAC-SHA256 + Authority Salt
    Vendor->>GenApp: Klik "Issue Signed Permanent License"
    GenApp->>GenApp: Tandatangani Payload dengan Ed25519 Signing Key
    GenApp->>AuthStorage: Simpan Catatan Otoritas & Garansi 1 Tahun
    Vendor->>GenApp: Klik "Export Signed License File"
    GenApp->>Vendor: Simpan Berkas mitra_license_<SN>.lic
    Vendor->>Customer: Kirimkan Serial Number & Berkas .lic

    Note over Customer,CustApp: 3. Tahap Aktivasi di Sisi Pelanggan
    Customer->>CustApp: Masukkan Path Berkas .lic & Klik "Load .lic File"
    CustApp->>CustApp: Parse Payload Kanonikal & Ekstrak Serial Number
    Customer->>CustApp: Masukkan SN & Klik "REGISTER"
    CustApp->>CustApp: Verifikasi Offline Kunci Publik Ed25519 (32B)
    CustApp->>CustApp: Cocokkan Native ID Mesin Lokal vs Payload
    CustApp->>CustApp: Simpan Status ke mitra_customer_registration.dat
    CustApp->>Customer: Tampilkan "STATUS: TERLISENSI PERMANEN (AKTIF)"
    CustApp->>Customer: Aktifkan Fitur BASIC_AUDIO & WEB_MANAGEMENT
```

---

## 2. Struktur Payload Berkas Lisensi Distribusi

Berkas `.lic` yang diekspor oleh vendor memiliki format berikut:

```text
MITRA_AUDIO_LICENSE_V1
PAYLOAD=V1|LIC-1731380000|MITRA-AUDIO-SERVER|WIN-HWID-V1:MB-...:CPU-...:DISK-...|MANS-MITR-A1B2-C3D4-E5F6|1731380000|permanent|BASIC_AUDIO,WEB_MANAGEMENT|MITRA-AUTHORITY-ROOT
SIGNATURE=7C8F9E... (128 karakter Hexadecimal mewakili 64-byte Ed25519 Signature)
```

### Properti Utama:
1. **Permanen:** Bidang `license_type` wajib bernilai `"permanent"`. Tidak ada batasan tanggal kadaluarsa (*expiry timestamp*) di dalam payload.
2. **Keterikatan Kuat:** Payload secara eksplisit mengikat `product_id` dan `native_id`. Memodifikasi satu karakter pun akan menyebabkan verifikasi tanda tangan digital Ed25519 gagal total.
3. **Pemberian Hak Fitur (*Entitlements*):** Fitur seperti antarmuka web konsol (`WEB_MANAGEMENT`) hanya diizinkan aktif jika string tersebut tercantum secara eksplisit dalam daftar entitlement yang ditandatangani.
