# SPON PHASE M HARDWARE ACQUISITION CHECKLIST
## Checklist for Official SPON Ownership Verification & ET99 Hardware Acquisition

- **Date:** 2026-11-12
- **Status:** ACTIVE CHECKLIST

---

## 1. Ownership & Entitlement Documentation Checklist

Before submitting an official request to Changsha SPON or purchasing replacement Feitian ET99 keys, the customer must gather:

- [ ] **Original Sales Contract / Invoice:** Proof of purchase showing acquisition of SPON IPNBS v3.3.0 and associated hardware terminals.
- [ ] **Historical Terminal Count Entitlement:** Number of authorized client broadcast stations (e.g. 20, 50, 100, 250 zones).
- [ ] **Original System Integrator / Partner Information:** Company name of the local installer who originally deployed the system.
- [ ] **Target Host Hardware Fingerprint:**
  - Machine ID / Volume Serial of Drive C: (`<TARGET_VOL_SERIAL>`)
  - MAC Address of primary network adapter (`<TARGET_MAC_ADDR>`)
  - Target Windows OS edition (`Windows 10/11 Pro 64-bit`).

---

## 2. ET99 Hardware Key Procurement Specifications

If Changsha SPON authorizes replacement of physical Feitian ET99 dongles:

- [ ] **Device Specification:** Feitian ET99 Standard USB Token (HID Class, no special driver install needed on Windows).
- [ ] **Vendor ID:** `0x096E` (Feitian Technologies).
- [ ] **Product ID:** `0x0471` / `0x0001` (Standard HID ET99).
- [ ] **Vendor PID Programming:** Must be programmed with the specific 8-byte SPON Product ID using Feitian ET99 Dev Kit and SPON master keys.
- [ ] **Validation Protocol:** Plug into host USB port, invoke `FT_ET99_API.dll` function `et_FindToken`, and verify token count >= 1.
