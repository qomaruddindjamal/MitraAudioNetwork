# SPON PHASE L ET99 HARDWARE REPORT
## Feitian ET99 USB Dongle Inspection & Hardware Dependency Assessment

- **Target Component:** Feitian ET99 USB Security Key Subsystem
- **Driver DLL:** `C:\MitraAudio\analysis_work\spon_clone_test\FT_ET99_API.dll`
- **Original Source File:** `C:\IPNBS\FT_ET99_API.dll` (Size: 226,304 bytes)
- **Date:** 2026-11-12
- **Status:** BLOCKED — PHYSICAL ET99 DONGLE REQUIRED

---

## 1. Hardware Enumeration Audit

Windows PnP subsystem enumeration (`Get-PnpDevice`) was conducted across all connected USB classes and hardware IDs:

- **Target Vendor IDs for Feitian Technologies:** `VID_096E`, `VID_0471`
- **PnP Query Result:** `0 devices found matching Feitian Hardware IDs`
- **Physical dongle state:** Not connected to host system.

---

## 2. API Driver Inspection (`FT_ET99_API.dll`)

Direct dynamic testing of `FT_ET99_API.dll` via ctypes/P-Invoke:
- Function called: `et_FindToken(pid, &count)`
- Return Value: `0` (Success execution of API query)
- Dongle Count Output: `0` (Zero tokens detected)
- Exported API Set:
  - `et_FindToken`
  - `et_OpenToken`
  - `et_CloseToken`
  - `et_Verify`
  - `et_GenPID`
  - `et_Read`
  - `et_Write`
  - `et_Reset`

---

## 3. Dependency Impact & Native SPON Behavior

1. **Hardware Mode vs Software INI Mode:**
   - When a physical Feitian ET99 dongle is present, SPON reads the hardware vendor token ID directly from the crypto-chip.
   - When the ET99 dongle is absent (current operational state), `IPNBS_Server.exe` falls back to the INI-based software registration flow (`SPON_SN.ini` / Machine ID check).
2. **Registration Gate (RVA `0x00439360`):**
   - Software fallback performs the 28-character formatted serial verification (`5-5-5-5-4`), hashing against MD5 at RVA `0x0043DB00`.
3. **Conclusion:**
   - Native SPON hardware token recovery is classified as:
     `BLOCKED — PHYSICAL ET99 DONGLE REQUIRED`
   - Operational recovery must either rely on original hardware dongle recovery or authorized vendor software license issuance.
