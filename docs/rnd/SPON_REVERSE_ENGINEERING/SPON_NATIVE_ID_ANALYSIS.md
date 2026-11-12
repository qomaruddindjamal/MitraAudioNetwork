# SPON NATIVE ID & HARDWARE IDENTIFIERS ANALYSIS
**Target:** Mekanisme Identifikasi Perangkat Keras SPON vs Mitra Audio  
**Workspace:** `C:\MitraAudio`  

---

## 1. MEKANISME HARDWARE IDENTIFIER SPON

Dari tabel import dan dekompilasi `IPNBS_Server.exe`:
- **Volume Serial Number:** Menggunakan `GetVolumeInformationA` untuk membaca serial partisi disk Windows lokal.
- **CPU & Arsitektur:** Menggunakan `GetSystemInfo` untuk membaca karakteristik prosesor.
- **Kelemahan Legacy:** Skema lama sangat kaku (fragile). Penggantian harddisk atau format partisi langsung membatalkan serial registrasi.

## 2. PERBANDINGAN TERHADAP MITRA AUDIO NATIVE ID (`WIN-HWID-V1`)

| Aspek | Legacy SPON Native ID | Mitra Audio Native ID (`WIN-HWID-V1`) |
| :--- | :--- | :--- |
| **Komponen Hardware** | Volume Serial Number lokal | Motherboard UUID + CPUID + Disk Physical Serial |
| **Kriptografi** | Checksum internal MD5 | SHA-256 dengan Domain Separation |
| **Toleransi Kerusakan** | 0% (Exact Match) | 70% Weighted Similarity Matching (Anti-Drift) |
| **Integritas Lisensi** | Serial string tertutup | Tanda Tangan Kriptografi Ed25519 Asimetris |
