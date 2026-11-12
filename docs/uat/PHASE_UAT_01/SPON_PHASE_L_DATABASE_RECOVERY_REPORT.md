# SPON PHASE L DATABASE RECOVERY REPORT
## Microsoft Jet 4.0 Architecture, OLE DB Compatibility, and Encryption Analysis

- **Target File:** `C:\MitraAudio\analysis_work\spon_clone_test\DATABASE\DBS.mdb`
- **Original Source File:** `C:\IPNBS\DATABASE\DBS.mdb` (Size: 4,292,608 bytes)
- **Date:** 2026-11-12
- **Status:** PASS (Technical Audit & Compatibility Verified)

---

## 1. File Format & Engine Identification

Binary analysis of the database file header confirms:
- **Offset `0x04` - `0x12`:** ASCII magic string `Standard Jet DB\x00`
- **Offset `0x14` (Format Version Byte):** `0x01` -> Microsoft Jet Database Engine 4.0 (Access 2000 / XP / 2003 format)
- **Total Size:** 4,292,608 bytes (4.09 MB)
- **Compatibility Status:** Native 32-bit `Microsoft.Jet.OLEDB.4.0` driver available on Windows under `SysWOW64`. 64-bit processes require 32-bit subsystem bridge or Microsoft Access Database Engine redistributable (`Microsoft.ACE.OLEDB.12.0`).

---

## 2. Password & Encryption Analysis

Microsoft Jet 4.0 databases apply an XOR obfuscation mask at offset `0x42` across 40 bytes. Extraction and de-masking of `DBS.mdb` header revealed:
- Unicode password sequence: `\x14\x96p\x00\x08\x96n\x00_\x969\x00^\x966\x00U\x96`
- Character codepoints: `U+9614`, `U+0070`, `U+9608`, `U+006E`, `U+965F`, `U+0039`, `U+0036`, `U+9655`
- Encoding: Chinese simplified codepage (GBK / CP936) native to Changsha SPON Communication Technology Co., Ltd.
- Security Policy: The native database password is embedded internally within the compiled 32-bit binary `IPNBS_Server.exe`, allowing the server to automatically open, query, and populate terminal configuration tables upon launch.

---

## 3. Associated Audio & Configuration DLLs

1. **`DBSEnc.dll` Analysis:**
   - File Size: ~163 KB
   - Exported Symbols: `beInitStream`, `beEncodeChunk`, `beDeinitStream`, `beCloseStream`, `beVersion`, `beWriteInfoTag`, `lame_encode_buffer_interleaved`.
   - Identification: Modified LAME MP3 encoding library, responsible for compressing incoming audio streams prior to UDP broadcast transmission.
2. **`DBSEnc.dat` Analysis:**
   - File Content: `[MEO] Type=0`
   - Purpose: Encoding profile parameter specifying raw/standard MP3 frame structure.

---

## 4. Recovery & Migration Strategy for Mitra Audio

1. **Native SPON Recovery Mode:**
   - Use the intact clone copy of `DBS.mdb` for running `IPNBS_Server.exe`.
   - Do NOT attempt destructive schema migration directly on the original `C:\IPNBS\DATABASE\DBS.mdb`.
2. **Mitra Audio Integration Mode:**
   - Mitra Audio Network utilizes its own modern, standalone SQLite database (`mitra_audio.db`) with Ed25519 asymmetric signature licensing.
   - SPON terminal configurations can be extracted non-destructively through runtime memory dump or protocol sniffing on UDP `234.0.0.254:2046` without altering the proprietary Jet 4.0 database.
