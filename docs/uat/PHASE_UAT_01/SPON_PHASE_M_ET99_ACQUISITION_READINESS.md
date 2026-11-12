# SPON PHASE M ET99 ACQUISITION READINESS
## Feitian ET99 Hardware Dependency & Procurement Readiness

- **Subsystem Target:** Feitian ET99 USB Hardware Key Subsystem
- **Current Execution Environment:** Windows 10/11 x64
- **Date:** 2026-11-12
- **Hardware Status:** BLOCKED — PHYSICAL ET99 DONGLE REQUIRED

---

## 1. Inventory & Verification Status

Dynamic PnP device enumeration and API queries against `FT_ET99_API.dll` in Phase L and Phase M confirm:
- **Connected Feitian Dongles:** 0
- **`et_FindToken` Return Value:** 0 (Zero tokens found)
- **Current Operational Mode:** Software INI fallback mode (`SPON_SN.ini` / Machine ID check).

---

## 2. Hardware Specification Baseline

| Specification Item | Verified Baseline |
| :--- | :--- |
| **Manufacturer** | Feitian Technologies Co., Ltd. (Beijing, China) |
| **Model** | ET99 Token (USB HID Class Dongle) |
| **Vendor ID (VID)** | `0x096E` |
| **Standard Product ID (PID)** | `0x0471` / `0x0001` |
| **Interface Class** | USB Human Interface Device (HID) — Driverless under Windows |
| **Host Subsystem** | 32-bit user-mode DLL (`FT_ET99_API.dll`, 226,304 bytes) |
| **Supported OS** | Windows 98/2000/XP/7/8/10/11 (32-bit & WOW64) |

---

## 3. Strict Compliance Policy

- **No Emulators:** In strict observance of Phase M directives, no virtual USB emulators, driver shims, or spoofed API intercepts have been or will be deployed.
- **Official Path:** Hardware recovery must proceed exclusively through official acquisition of authorized ET99 dongles from Feitian Technologies or Changsha SPON.
- **Classification:** **`BLOCKED — PHYSICAL ET99 DONGLE REQUIRED`**.
