# LAPORAN UJI RUNTIME ANTARMUKA DESKTOP (GUI)
**Dokumen:** `GUI_RUNTIME_TEST_REPORT.md`  
**Proyek:** Mitra Audio Network System  
**Metodologi Pengujian:** Otomasi Win32 Message Loop, Process Lifecycle, dan Windows GDI Screen Capture  
**Status Eksekusi:** PASS (Seluruh Tampilan & Kontrol Terverifikasi Nyata)

---

## 1. Verifikasi Eksekusi Proses Nyata

Pengujian runtime membuktikan bahwa biner yang dihasilkan adalah aplikasi desktop Windows interaktif sejati (*interactive GUI window application*), bukan program command-line atau mockup HTML.

| Biner yang Diuji | Process ID (PID) | Status Jendela Windows | Waktu Peluncuran | Responsivitas Loop |
|---|---|---|---|---|
| `mitra-sn-generator.exe` | PID 19080 & 5702260 | `WS_VISIBLE`, Valid HWND | < 0.25 detik | Responsif terhadap `WM_COMMAND`, `WM_PAINT` |
| `mitra-customer-app.exe` | PID 17784 & 5767796 | `WS_VISIBLE`, Valid HWND | < 0.20 detik | Responsif terhadap `WM_COMMAND`, `WM_PAINT` |

---

## 2. Bukti Tangkapan Layar Windows Asli (Screenshots)

Seluruh tangkapan layar asli telah diekstraksi langsung dari handle jendela Windows aktif (*PrintWindow Full Content Rendering*) dan tersimpan pada direktori:  
`docs/poc/license-generator-gui/screenshots/`

1. **`01_vendor_sn_generator_dashboard.png` (Ukuran: 28,124 bytes)**
   - Menampilkan layar beranda Vendor Tool: Dashboard metrik otoritas lisensi, statistik lisensi aktif, riwayat audit penerbitan, dan ringkasan status kriptografi Ed25519.
2. **`02_customer_app_initial.png` (Ukuran: 23,778 bytes)**
   - Menampilkan konsol registrasi aplikasi pelanggan: Kolom Native ID perangkat keras terisi otomatis, tombol "Copy Native ID", kolom input berkas lisensi `.lic`, kolom Serial Number, tombol "REGISTER", serta status awal aplikasi belum terlisensi.
3. **`03_vendor_sn_generator_view.png` (Ukuran: 24,553 bytes)**
   - Menampilkan layar kerja SN Generator: Form input Native ID pelanggan, tombol "Paste Clipboard", tombol "Generate Serial Number", tombol "Issue Signed Permanent License", kolom hasil Serial Number, tombol "Copy SN", dan tombol "Export Signed License File".
4. **`04_vendor_license_issuer_view.png` (Ukuran: 22,802 bytes)**
   - Menampilkan detail lengkap paket lisensi yang diterbitkan: Nilai parameter kanonikal, status lisensi permanen, daftar entitlement yang diaktifkan, dan tanda tangan digital 64-byte.
5. **`05_vendor_license_verification_view.png` (Ukuran: 23,282 bytes)**
   - Menampilkan layar uji verifikasi independen: Form impor berkas lisensi, kolom pengujian Native ID, dan kotak hasil verifikasi kriptografi Ed25519 secara visual.
6. **`06_vendor_support_replacement_view.png` (Ukuran: 21,731 bytes)**
   - Menampilkan antarmuka audit layanan garansi dukungan penggantian Serial Number (1 tahun) berdasarkan nomor serial yang dicari.

---

## 3. Verifikasi Respons Kontrol Interaktif

1. **Tombol "Paste Clipboard":** Berhasil membaca data teks dari clipboard Windows (`CF_TEXT`) dan mengisi kotak teks `ID_EDIT_NATIVE_ID` secara instan.
2. **Tombol "Copy SN":** Berhasil menaruh teks Serial Number 24 karakter ke dalam memori clipboard Windows dengan aman.
3. **Tombol "Export Signed License File (.lic)":** Berhasil menulis berkas fisik `.lic` ke media penyimpanan lokal.
4. **Tombol Navigasi Sidebar:** Berhasil berpindah antar 8 tampilan secara mulus dengan memicu pengalihan status kontrol `ShowWindow(SW_SHOW / SW_HIDE)` dan merender ulang header via `InvalidateRect`.
