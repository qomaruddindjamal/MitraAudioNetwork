# BATCH 2 VALIDATION FILE MANIFEST
**Project:** Mitra Audio Network System  
**Workspace:** `C:\IPNBS`  
**Checkpoint:** `CP-PHASE-1-BATCH-2-CLOSURE-VALIDATION`

---

## 1. Daftar Dokumen Laporan Validasi Batch 2
Semua dokumen validasi tersimpan di bawah folder `C:\IPNBS\docs\validation\batch-2-closure\`:

1. `BATCH_2_CLOSURE_EXECUTIVE_REPORT.md`: Laporan eksekutif menyeluruh, keputusan teknis final (PASS), dan jawaban eksplisit atas seluruh pertanyaan verifikasi.
2. `SDK_37_ABI_VALIDATION.md`: Bukti validasi ABI 37 fungsi ekspor legacy vs modern (ordinal 1..37, calling convention `__cdecl`, parameter types, perbandingan `dumpbin.exe`).
3. `TCP_UDP_INTEGRATION_VALIDATION.md`: Bukti validasi TCP Control Port 2046 dan UDP Multicast `tagDGRAMHEADER` Port 2048.
4. `WINDOWS_NATIVE_ID_STABILITY.md`: Bukti stabilitas Native ID Windows, bobot penilaian kesamaan hardware, dan kebijakan toleransi reaktivasi.
5. `WASAPI_PERFORMANCE_BENCHMARK.md`: Hasil benchmark performa audio Windows WASAPI, metrik latensi buffer 10 ms / 5 ms, dan analisis latensi end-to-end.
6. `BATCH_2_REMEDIATION_REPORT.md`: Laporan detail perbaikan kekurangan teknis yang ditemukan selama validasi.
7. `BATCH_2_VALIDATION_TEST_RESULTS.md`: Catatan komprehensif eksekusi 17 pengujian unit dan integrasi Rust (100% PASS).
8. `BATCH_2_VALIDATION_FILE_MANIFEST.md`: File ini — manifes inventaris file yang dibuat dan diperbarui.
9. `BATCH_2_VALIDATION_CHECKPOINT.md`: Deklarasi checkpoint formal `CP-PHASE-1-BATCH-2-CLOSURE-VALIDATION`.

---

## 2. File Kode Sumber yang Dibuat & Dimodifikasi Selama Validasi
- `crates/mitra-sdk-c/IPNBSSDK.def`: Disesuaikan agar memuat tepat 37 fungsi dengan ordinal 1..37 identik dengan biner legacy.
- `crates/mitra-sdk-c/src/lib.rs`: Dihapus 2 fungsi non-legacy eksternal.
- `crates/mitra-adapter-windows/src/hw_windows.rs`: Ditambahkan metode `compute_similarity_score` dan dukungan kustomisasi token.
- `crates/mitra-adapter-windows/tests/tcp_2046_tests.rs`: Dibuat test suite baru untuk siklus integrasi TCP Port 2046.
- `crates/mitra-adapter-windows/tests/windows_tests.rs`: Ditambahkan pengujian scoring Native ID dan benchmark durasi buffer WASAPI.

---

## 3. Artefak Biner Kompatibilitas 32-bit yang Divalidasi
- `target/i686-pc-windows-msvc/debug/IPNBSSDK.dll`: Biner drop-in x86 yang diverifikasi dengan `dumpbin.exe` (37 exports, ordinal 1..37).

---

## 4. Biner Legacy yang Diproteksi (Preservation Lock)
- `C:\IPNBS\IPNBS_Server.exe` (SHA-256: `7F910FAFC2D66BF0FC7488F95204C9119AB9022347CF7175E4A578C2103E8AE9`)
- `C:\IPNBS\IPNBS_Station.exe` (SHA-256: `61286ABAD5EFCA2AA7721415D870DF2F60EEC0FB9CF3B2D1ADE6A2F74904C0E2`)
- `C:\IPNBS\IPNBSSDK.dll` (SHA-256: `FBDC81009FF4306504C6374F6C3D18A54663EF699900A93C8A7489B30B904211`)
- `C:\IPNBS\FT_ET99_API.dll` (SHA-256: `7DCBC2C408060B7C5E9A7EAF46E177F647F818D2C963847B7D6D78F34AB33BDE`)

Semua biner asli terbukti 100% utuh tanpa modifikasi apa pun.
