# SPON NATIVE ID FINDINGS & LEGACY HARDWARE IDENTIFIERS
**Target:** Mekanisme Identifikasi Perangkat Keras Legacy IPNBS  
**Status:** CONFIRMED & INFERRED

---

## 1. Win32 API Hardware Discovery pada Legacy `IPNBS_Server.exe`

Dari tabel impor kernel biner legacy `IPNBS_Server.exe`:
- `GetVolumeInformationA`: Membaca Volume Serial Number drive lokal.
- `GetSystemInfo`: Membaca arsitektur CPU, tipe prosesor, dan jumlah core.
- `GetComputerNameA`: Membaca nama hostname NetBIOS komputer.
- `DeviceIoControl`: Mengirimkan control code IOCTL (seperti `SMART_RCV_DRIVE_DATA` atau `IOCTL_STORAGE_QUERY_PROPERTY`) untuk membaca serial number fisik disk ATA/SCSI.

---

## 2. Pemisahan Hardware Identifier Internal vs Native ID Tampilan UI

1. **Hardware Identifier Internal:**  
   Biner legacy mengumpulkan atribut perangkat keras mentah untuk mengkalkulasi seed hash.
2. **Native ID Tampilan UI (Registration Code):**  
   Pada antarmuka registrasi server, aplikasi menampilkan kode hex format ringkas yang diturunkan dari Volume Serial atau MAC Address kartu jaringan Ethernet stasiun.

---

## 3. Komparasi Terhadap Native ID Mitra Audio (`WIN-HWID-V1`)

| Parameter | Mitra Audio (`WIN-HWID-V1`) | Legacy IPNBS / SPON |
|---|---|---|
| **Komponen** | Motherboard UUID + CPUID + Drive Serial | Volume Serial / MAC / Disk IOCTL |
| **Kriptografi** | SHA-256 terstandarisasi dengan Domain Separation | Hashing MD5 internal |
| **Toleransi** | Weighted Similarity Scoring (MB 40%, CPU 35%, Disk 25%) | Exact match (skema biner kaku) |
| **Layanan Lisensi** | Terikat Public Key Ed25519 | Terikat Algoritma SN Tertutup |
