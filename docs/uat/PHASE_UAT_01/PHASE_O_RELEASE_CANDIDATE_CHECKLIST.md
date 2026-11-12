# PHASE O RELEASE CANDIDATE CHECKLIST
## Mitra Audio Network System — Release Candidate v0.1.0-RC1

- **Product:** Mitra Audio Network System
- **Version:** `0.1.0-RC1`
- **Build Timestamp:** 2026-11-13
- **Commit Target:** Phase O Hardened Baseline
- **Release Status:** STAGED RELEASE CANDIDATE (CONTROLLED DEPLOYMENT ONLY)

---

## 1. Release Candidate Verification Checklist

- [x] **Compiler & Linter Pass:** `cargo fmt --check` clean, `cargo clippy` 0 warnings.
- [x] **Unit & Integration Tests:** 21/21 workspace cargo tests 100% passing.
- [x] **Release Binaries Compiled:** `mitra-customer-app.exe`, `mitra-sn-generator.exe`, `IPNBSSDK.dll` built in `target\release`.
- [x] **Cryptographic Hash Verification:** SHA-256 computed and documented for all release binaries.
- [x] **Key Isolation Verified:** Zero private keys packaged in customer artifacts.
- [x] **Entitlement Enforcement Verified:** Demo/Trial restricted from `WEB_MANAGEMENT`.
- [x] **Concurrent Load Validated:** 5 concurrent stations tested with <15 ms reconnect latency.
- [x] **Disaster Recovery Validated:** Interrupted shutdown and database backup/restore tested with 100% hash parity.
- [x] **Preservation Lock Untouched:** All 6 protected legacy files 100% hash match.
- [x] **Batch 3 Strictly Locked:** `BATCH_3_STATUS = LOCKED` maintained without exception.

---

## 2. Release Artifacts & SHA-256 Checksums

```text
464EF97741625EB03AC16AFDFFBB219D093290C12AD7638A545688DB25C231B8  mitra-customer-app.exe
7E9612194A15A28F30093863907FE7DF46FEA85442413E4BB7446531B1225C67  mitra-sn-generator.exe
2DBF8F8B5B858BA0290D9B55B4855106E961929913A26E43C9EA6952D5316E4E  IPNBSSDK.dll
```

---

## 3. Known Limitations & Production Guidance

1. **Hardware Dongle Dependency for Legacy SPON:**
   - Legacy SPON IPNBS v3.3.0 broadcast audio streaming requires physical Feitian ET99 USB Dongle or authorized vendor serial (`BLOCKED — PHYSICAL ET99 DONGLE REQUIRED`).
2. **Mitra Audio Native Operation:**
   - Fully independent; uses modern Ed25519 asymmetric cryptography and does not require legacy hardware keys.
