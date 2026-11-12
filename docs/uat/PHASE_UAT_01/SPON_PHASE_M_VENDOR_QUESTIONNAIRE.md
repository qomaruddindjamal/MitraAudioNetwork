# SPON PHASE M VENDOR QUESTIONNAIRE
## Technical Questionnaire for Vendor License Recovery & Compatibility

- **Subject:** SPON IPNBS v3.3.0 & Feitian ET99 Licensing Subsystem
- **Target Vendors:** Changsha SPON Communication Technology Co., Ltd. & Feitian Technologies Co., Ltd.
- **Date:** 2026-11-12
- **Status:** READY FOR STAKEHOLDER SUBMISSION

---

## 1. Technical Inquiries for Changsha SPON

1. **Software Serial Regeneration:**
   - Does Changsha SPON maintain registration code generators for legacy IPNBS v3.3.0 (`5-5-5-5-4` 28-character serial verified against MD5 algorithm at RVA `0x0043DB00`)?
   - What machine parameters are required (e.g. Volume Serial Number, Primary NIC MAC address, CPUID)?

2. **Database Schema & Maintenance:**
   - Is there an official database maintenance or migration tool for `DBS.mdb` (Microsoft Jet 4.0 format) to migrate legacy terminal listings to modern SQLite or SQL Server databases?

3. **Protocol Interoperability:**
   - Are legacy IPNBS v3.3.0 terminals communicating on UDP multicast `234.0.0.254:2046` and TCP 1978 directly compatible with newer SPON audio servers (v4.x / v5.x)?

4. **Hardware Replacement vs Software Licensing:**
   - Does SPON recommend replacing the Feitian ET99 hardware dongle or issuing a permanent machine-locked software serial?

---

## 2. Technical Inquiries for Feitian Technologies

1. **ET99 Token Availability:**
   - Is the standard Feitian ET99 USB Token (`VID_096E`, `PID_0471` or `PID_0001`) still available for commercial order as replacement stock?
2. **Driver Compatibility:**
   - Does `FT_ET99_API.dll` (v1.x) operate natively under 64-bit Windows 10/11 through the `SysWOW64` driver subsystem, or is an updated kernel driver required?
3. **PID/Secret Key Re-flashing:**
   - Can replacement blank ET99 tokens be programmed with legacy vendor Product IDs (PID) if authorized by Changsha SPON?
