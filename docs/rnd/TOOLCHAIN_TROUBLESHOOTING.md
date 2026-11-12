# TOOLCHAIN TROUBLESHOOTING & ADAPTATION LOG
## MITRA AUDIO NETWORK SYSTEM

**Tanggal:** 2026-11-11  

---

### Masalah 1: Ketidakcocokan Versi Ghidra 12.2 terhadap REA 6.8.0
- **Gejala:** Saat menjalankan `rea analyze` dengan Ghidra 12.2, REA menolak dengan error `unsupported_version: 12.2; accepted releases are 12.1.x (verified build 12.1.4)`.
- **Akar Masalah:** REA 6.8.0 memiliki rule validasi ketat terhadap versi Ghidra yang kompatibel dan saat ini mendukung lini `12.1.x`. Selain itu, Ghidra 12.2 memerlukan JDK 25 minimum, sedangkan Ghidra 12.1.x berjalan stabil di JDK 21 LTS.
- **Tindakan Perbaikan:**
  1. Mengunduh dan mengekstrak rilis resmi **Ghidra 12.1.4 PUBLIC** ke `C:\Tools\Ghidra\ghidra_12.1.4_PUBLIC`.
  2. Mengonfigurasi `JAVA_HOME` ke **Eclipse Temurin JDK 21 LTS** (`C:\Tools\jdk-21\jdk-21.0.13+9`).
  3. Memperbarui `GHIDRA_INSTALL_DIR` di tingkat Machine Environment.
- **Hasil Verifikasi:** REA mendeteksi Ghidra 12.1.4 dengan status `healthy: true` pada pemeriksaan bridge Ghidra dan dekompilasi berhasil 100%.

---

### Masalah 2: Node.js Engine Warning pada npm global install
- **Gejala:** Muncul pesan `npm warn EBADENGINE Unsupported engine { package: 'rea-agents@6.8.0', required: { node: '^22.23.4 || ^24.22.1 || >=26.12.1' }, current: { node: 'v24.19.0' } }`.
- **Analisis Dampak:** Node.js yang terpasang di host adalah `v24.19.0`. Warning engine tidak menghalangi eksekusi script CLI maupun MCP server REA, karena modul inti JavaScript dan dependency native tetap berjalan normal pada Windows x64.
- **Mitigasi:** Tidak perlu melakukan upgrade global Node.js yang berisiko merusak environment proyek lain, karena REA berjalan sempurna tanpa hambatan runtime.
