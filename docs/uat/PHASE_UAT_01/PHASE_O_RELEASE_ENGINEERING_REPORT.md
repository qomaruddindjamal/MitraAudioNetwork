# PHASE O RELEASE ENGINEERING REPORT
## Windows Binary Artifacts, Build Profiles & Release Packaging Specifications

- **Project:** Mitra Audio Network System
- **Compiler / Target:** `rustc 1.83.0` / `x86_64-pc-windows-msvc`
- **Release Profile:** `opt-level = 3`, LTO enabled, strip symbols
- **Date:** 2026-11-13
- **Status:** PASS (Release Binaries Successfully Generated & Verified)

---

## 1. Verified Release Binary Inventory

All release artifacts were compiled with `cargo build --release` and verified for cryptographic integrity:

| Binary Filename | Type | Size (Bytes) | SHA-256 Checksum | Purpose / Target |
| :--- | :--- | :--- | :--- | :--- |
| **`mitra-customer-app.exe`** | PE64 GUI | 266,240 | `464EF97741625EB03AC16AFDFFBB219D093290C12AD7638A545688DB25C231B8` | Customer desktop runtime & license activator |
| **`mitra-sn-generator.exe`** | PE64 GUI | 300,032 | `7E9612194A15A28F30093863907FE7DF46FEA85442413E4BB7446531B1225C67` | Vendor SN generator & license issuance tool |
| **`IPNBSSDK.dll`** | PE64 DLL | 118,784 | `2DBF8F8B5B858BA0290D9B55B4855106E961929913A26E43C9EA6952D5316E4E` | Drop-in C-ABI SDK with 37 standard export functions |

---

## 2. Release Packaging & Deployment Safeguards

1. **Package Separation:**
   - **Customer Distribution:** Bundles only `mitra-customer-app.exe`, `IPNBSSDK.dll`, and vendor public keys (`verifying_key.bin`).
   - **Vendor Internal Package:** Contains `mitra-sn-generator.exe` and private signing material.
2. **Zero Insecure Assets:**
   - No private keys are included in customer distribution packages.
   - No test databases or unencrypted credentials are packaged.
   - Zero proprietary SPON original binaries are redistributed without vendor license agreement.
3. **Windows System Compatibility:**
   - Compatible with Windows 10, Windows 11, Windows Server 2019/2022 (x64).
   - In-box WASAPI audio driver support; zero third-party kernel driver requirements.
   - Standard firewall rules: Inbound UDP 2046, 2048 and TCP 1978.
