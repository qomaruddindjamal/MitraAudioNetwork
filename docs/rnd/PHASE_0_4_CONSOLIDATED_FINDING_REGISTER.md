# PHASE 0.4 — CONSOLIDATED FINDING REGISTER & DISPOSITION MATRIX

**Project:** Mitra Audio Network System  
**Document Code:** `REG-FINDINGS-FINAL-002`  
**Status:** COMPLETE & CONSOLIDATED (Phase 0.4 Final Gate)  

---

## 1. Finding Classification Framework

Setiap temuan dalam siklus R&D (Phase 0 s/d Phase 0.4) diklasifikasikan ke dalam salah satu dari tiga tingkat keparahan:
- **BLOCKER:** Masalah tak terpecahkan yang membuat implementasi tidak aman atau ambigu. (Harus dipecahkan sebelum Phase 1).
- **CONDITIONAL:** Keterbatasan atau dependensi masa depan yang diketahui dan tidak mencegah implementasi awal, asalkan memiliki mitigasi, pemilik (*owner*), dan *acceptance criteria* terdefinisi.
- **NON-BLOCKING:** Penyempurnaan dokumentasi, optimasi minor, atau fitur yang masuk ke milestone implementasi berikutnya.

---

## 2. Consolidated Finding Register

| Finding ID | Subsystem Terkait | Deskripsi Temuan / Keterbatasan | Klasifikasi | Status / Disposisi | Pemilik (Owner) | Milestone Implementasi & Acceptance Criteria |
|---|---|---|---|---|---|---|
| **FND-01** | Licensing | Driver USB Dongle Feitian ET99 (`FT_ET99_API.dll`) tidak tersedia di Linux, macOS, dan MitraOS. | **CONDITIONAL** | Mitigasi Disetujui (ADR-003 & ADR-009) | Cryptography Engineer | **Milestone M1.5:** Lisensi digital asimetris Ed25519 menggantikan dongle fisik. AC: Verifikasi signature lolos < 5ms tanpa dongle USB. |
| **FND-02** | Database | Microsoft Jet 4.0 (`DBS.mdb`) rentan korupsi file dan tidak portabel ke non-Windows. | **CONDITIONAL** | Mitigasi Disetujui (ADR-002 & ADR-010) | Database Engineer | **Milestone M1.4:** Migrasi ke SQLite 3 WAL Mode. AC: Ekstraktor otomatis mengonversi 14 tabel tanpa kehilangan data. |
| **FND-03** | Audio Engine | Terminal speaker analog lawas tidak mendukung codec modern Opus. | **CONDITIONAL** | Mitigasi Disetujui (ADR-008) | Audio Systems Engineer | **Milestone M1.2:** Dual-stack codec engine. AC: Terminal lama menerima IMA-ADPCM 22.05kHz, terminal baru menerima Opus 48kHz. |
| **FND-04** | Windows SDK | Aplikasi client legacy (`IPNBS_Station.exe`) membutuhkan biner 32-bit x86 `__cdecl` eksak. | **CONDITIONAL** | Mitigasi Disetujui (ADR-007) | Cross-Platform Systems Engineer | **Milestone M1.3:** Kompilasi `cdylib` target `i686-pc-windows-msvc` dengan berkas `.def`. AC: 37 fungsi ekspor dipanggil sukses via test harness. |
| **FND-05** | ASL Kernel (MitraOS)| ASL Kernel bare-metal belum memiliki driver audio soundcard dan socket networking. | **CONDITIONAL** | Mitigasi Disetujui (ADR-005 & ADR-011) | ASL Kernel Integration Engineer | **Milestone M4.1 - M4.4 (Stage A s/d D):** Pengembangan bertahap. AC: Driver DMA audio dan smoltcp socket lolos uji loopback. |
| **FND-06** | Network Engine | Error IOCP legacy 995 (`WSA_OPERATION_ABORTED`) saat disconnect socket membingungkan operator. | **NON-BLOCKING** | Terselesaikan Secara Arsitektural (ADR-004) | Network Audio Engineer | **Milestone M1.1:** Tokio async non-blocking socket loop menangani graceful teardown. AC: Zero error 995 pada log disconnect normal. |
| **FND-07** | Network Engine | Beberapa switch/router WAN memblokir UDP Multicast group `234.0.0.254`. | **CONDITIONAL** | Mitigasi Disetujui (Spec Net 004) | Network Audio Engineer | **Milestone M1.1:** Implementasi otomatis fallback ke Unicast Multi-Stream UDP jika multicast probe timeout (> 3 detik). |
| **FND-08** | Security | Risiko manipulasi jam sistem (clock rollback) untuk memperpanjang lisensi trial. | **NON-BLOCKING** | Mitigasi Disetujui (Spec Lic 005) | Cryptography Engineer | **Milestone M1.5:** Watermarking timestamp monotonic kernel terenkripsi mendeteksi rollback tanggal kalender. |
| **FND-09** | UI / Frontend | Dukungan layar sentuh (touchscreen) pada panel paging mic desktop. | **NON-BLOCKING** | Masuk Backlog UI Phase 2 | UI/UX Product Designer | **Milestone M2.2:** Penyesuaian hit-target PTT button minimal 48x48 px untuk layar sentuh operator. |

---

## 3. Evaluasi Status Blocker

- **Jumlah Temuan Teridentifikasi:** 9 temuan.
- **Temuan Berstatus BLOCKER:** **0 (NOL)**.
- **Temuan Berstatus CONDITIONAL:** **6** (Seluruhnya memiliki mitigasi teknis, pemilik tugas, dan milestone implementasi yang terdefinisi).
- **Temuan Berstatus NON-BLOCKING:** **3** (Telah terjadwal dalam backlog).

**Kesimpulan:** Tidak ada rintangan arsitektural yang menghalangi eksekusi implementasi Phase 1.
