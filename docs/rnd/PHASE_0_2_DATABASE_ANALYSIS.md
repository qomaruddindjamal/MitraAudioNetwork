# PHASE 0.2 — BATCH F: DATABASE & JET/DAO PERSISTENCE AUDIT

**Target Subsystem:** Database Persistence Engine (`CDaoManage`, `CDaoRecordset*`)  
**Storage File:** `DATABASE\DBS.mdb` (Microsoft Jet Database Engine 4.0 / Access MDB)  
**Host Binary:** `IPNBS_Server.exe`  
**Analysis Provider:** Ghidra 12.1.4 + `IPNBS_Server.map`

---

## 1. Arsitektur Database Legacy (DAO / Jet Engine)

Server legacy menggunakan Data Access Objects (DAO) dan mesin database Microsoft Jet 4.0 32-bit (`dao360.dll` / `msjet40.dll`):
- Koneksi dibuka pada `CDLSApp::InitInstance` menuju file path relatif `DATABASE\DBS.mdb`.
- Seluruh query dan modifikasi data dilakukan melalui representasi class wrapper MFC `CDaoRecordset`.

---

## 2. Inventaris Tabel Database (Berdasarkan Rekonstruksi Simbol)

Dari tabel simbol `IPNBS_Server.map` (`CDaoRecordset*`), sistem memetakan 14 tabel utama:

| Nama Recordset | Nama Tabel Database | Fungsi & Peran dalam Sistem |
|---|---|---|
| `CDaoRecordsetMember` | `tbl_Member` / `Member` | Menyimpan daftar terminal IP audio (ID, Name, IP, Port, MAC, Zone ID, Volume, Status). |
| `CDaoRecordsetGroup` | `tbl_Group` / `Group` | Menyimpan definisi grup/zona terminal (e.g. Lantai 1, Gedung A, Lapangan). |
| `CDaoRecordsetGroupMember` | `tbl_GroupMember` | Relasi many-to-many antara Group dan Member terminal. |
| `CDaoRecordsetRingTask` | `tbl_RingTask` | Jadwal tugas bel otomatis harian/mingguan (jam, menit, file audio, zona target). |
| `CDaoRecordsetRingPlayFile` | `tbl_RingPlayFile` | Daftar file audio nada bel (WAV/MP3) yang terdaftar di sistem. |
| `CDaoRecordsetRingMember` | `tbl_RingMember` | Terminal-terminal target untuk setiap tugas Ring Task. |
| `CDaoRecordsetTeachTask` | `tbl_TeachTask` | Jadwal siaran pendidikan/pengajaran otomatis berbasis waktu. |
| `CDaoRecordsetTeachMember` | `tbl_TeachMember` | Terminal target untuk tugas Teach Task. |
| `CDaoRecordsetBroadcastTask`| `tbl_BroadcastTask` | Sesi siaran manual atau terjadwal. |
| `CDaoRecordsetBroadcastMember`| `tbl_BroadcastMember`| Anggota terminal target sesi broadcast. |
| `CDaoRecordsetFileTask` | `tbl_FileTask` | Tugas pemutaran file musik latar belakang (BGM). |
| `CDaoRecordsetFilePlayFile` | `tbl_FilePlayFile` | Playlist file MP3 untuk pemutaran berulang. |
| `CDaoRecordsetAlarmTask` | `tbl_AlarmTask` | Konfigurasi aksi otomatis alarm darurat kebakaran (Fire Alarm Matrix). |
| `CDaoRecordsetMemberMemberIPRule`| `tbl_IPRule` | Aturan filter IP address, whitelist/blacklist terminal di jaringan. |

---

## 3. Masalah Utama Jet MDB Legacy
1. **Windows 32-bit Dependency:** Microsoft DAO 3.6 / Jet 4.0 tidak tersedia di lingkungan 64-bit murni tanpa driver OLEDB khusus, dan **sama sekali tidak tersedia di Linux, macOS, dan MitraOS**.
2. **Kerapuhan Database (Corruption Risk):** Berkas `.mdb` sangat rentan rusak jika terjadi pemadaman listrik mendadak atau aplikasi terhenti secara tidak normal (`.ldb` lockfile tertinggal).
3. **Konkurensi Rendah:** Jet engine mengunci file pada level halaman, membatasi skalabilitas multi-terminal berkapasitas besar.

---

## 4. Rencana Migrasi ke SQLite Modern (Cross-Platform)

Shared Core modern akan menggunakan **SQLite 3** dengan mode WAL (Write-Ahead Logging) via `rusqlite` / `sqlx` di Rust:

```mermaid
flowchart LR
    LEGACY[DATABASE\\DBS.mdb\nJet/DAO 32-bit MDB] -- "One-Time Automatic Migration Tool\n(mdb-tools / OleDb Extractor)" --> SQLITE[database/mitra_audio.db\nModern SQLite 3 (WAL Mode)]
    
    SQLITE --> SHARED_CORE[Rust Shared Core Database Layer]
    SHARED_CORE --> WIN[Windows Platform Adapter]
    SHARED_CORE --> LIN[Linux Platform Adapter]
    SHARED_CORE --> MAC[macOS Platform Adapter]
    SHARED_CORE --> MITRA[MitraOS Platform Adapter]
```

### Keuntungan Migrasi SQLite:
- **100% Standar Lintas Platform:** Format biner database `.db` identik di semua platform (Windows, Linux, macOS, MitraOS).
- **Zero Configuration:** Tanpa perlu install DBMS server tambahan.
- **ACID Compliant & Crash-Safe:** Aman dari kerusakan data akibat mati lampu atau reboot mendadak.
- **Migrasi Otomatis Transparan:** Sistem installer baru akan menyertakan skrip ekstraksi data dari `DBS.mdb` lama ke `mitra_audio.db` baru sehingga pengguna tidak kehilangan konfigurasi terminal dan jadwal bel yang sudah ada.
