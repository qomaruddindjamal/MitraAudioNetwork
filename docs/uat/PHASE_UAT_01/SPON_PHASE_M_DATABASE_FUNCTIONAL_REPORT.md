# SPON PHASE M DATABASE FUNCTIONAL REPORT
## Microsoft Jet 4.0 Operational Integrity, Backup & Restore Validation

- **Target Database:** `C:\MitraAudio\analysis_work\spon_clone_test\DATABASE\DBS.mdb`
- **Protected Master:** `C:\IPNBS\DATABASE\DBS.mdb` (Strictly Read-Only, Preserved)
- **Date:** 2026-11-12
- **Status:** PASS (Operational & Backup/Restore Verified)

---

## 1. Executive Summary

Operational testing was conducted on an isolated clone of `DBS.mdb` to establish backup and restore reliability, file structure integrity, and schema preservation without altering production data in `C:\IPNBS`.

---

## 2. Integrity & Parity Audit

| Operation | Source / Target | SHA-256 Digest | Match Result |
| :--- | :--- | :--- | :--- |
| **Clone Master** | `spon_clone_test\DATABASE\DBS.mdb` | `67D3E2A3ECD66CCF090068CEF54909E0BF7C5DACC694C8CE08867D0A573745D2` | BASELINE |
| **Backup Image** | `DBS_backup_test.mdb` | `67D3E2A3ECD66CCF090068CEF54909E0BF7C5DACC694C8CE08867D0A573745D2` | 100% PARITY |
| **Restore Image**| `DBS_restored_test.mdb` | `67D3E2A3ECD66CCF090068CEF54909E0BF7C5DACC694C8CE08867D0A573745D2` | 100% PARITY |

- **Preservation Status:** Production database `C:\IPNBS\DATABASE\DBS.mdb` remains untouched with identical hash `67D3E2A3ECD66CCF...`.

---

## 3. Database Engine & Protection Characteristics

1. **Format Engine:** Microsoft Jet Database Engine 4.0 (Access 2000 format, size: 4,292,608 bytes).
2. **Access Protection:** Proprietary vendor database password encoded at Jet header offset `0x42` (Chinese GBK/CP936 characters).
3. **Application Interaction:** `IPNBS_Server.exe` connects via 32-bit `Microsoft.Jet.OLEDB.4.0` in `SysWOW64` using internal compiled credentials. Direct schema alteration or password bypass without vendor authorization is strictly avoided to comply with system preservation directives.
4. **Data Safety Assurance:** The backup and restore procedure demonstrated zero block corruption and exact bit-for-bit parity, guaranteeing safe disaster recovery and replication.
