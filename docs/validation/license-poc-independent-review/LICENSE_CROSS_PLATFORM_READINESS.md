# LICENSE CROSS-PLATFORM READINESS ASSESSMENT
**Target Platform:** Windows (P1), Linux (P2), macOS (P3), MitraOS berbasis ASL Kernel (P4)  
**Status Kesiapan:** DESIGNED & COMPLIANT

---

## 1. Pemisahan Shared Core vs Platform Abstraction Layer (PAL)

Arsitektur lisensi Mitra Audio memisahkan secara ketat logika verifikasi dan pengumpulan data hardware:

```
+---------------------------------------------------------------------------------------------------+
| SHARED CORE (Portable Rust - crates/mitra-core)                                                   |
| - MitraLicensePayload, SignedMitraLicense                                                         |
| - SerialNumberGenerator (HMAC-SHA256)                                                             |
| - LicenseIssuer & LicenseVerifier (Ed25519)                                                       |
| - Backend Entitlement Enforcement (WEB_MANAGEMENT)                                                |
+---------------------------------------------------------------------------------------------------+
                                       ^
                                       | Trait: HardwareIdProvider
                                       v
+---------------------------------------------------------------------------------------------------+
| PLATFORM ADAPTERS (crates/mitra-pal & Platform Crates)                                            |
|                                                                                                   |
| [Windows Adapter - P1]    : WIN-HWID-V1 (SMBIOS BIOS UUID, CPUID, System Drive Serial)            |
| [Linux Adapter - P2]      : LIN-HWID-V1 (/etc/machine-id, DMI sysfs, root block serial)           |
| [macOS Adapter - P3]      : MAC-HWID-V1 (IOPlatformUUID, IOKit storage serial)                    |
| [MitraOS Adapter - P4]    : MITRAOS-HWID-V1 (Syscall ASL Kernel Machine GUID / ASL-HAL)          |
+---------------------------------------------------------------------------------------------------+
```

---

## 2. Kesiapan Spesifik Tiap Platform

### A. Windows (Priority 1) — SELESAI & DIVERIFIKASI
- Menggunakan `WindowsHardwareIdProvider` (`WIN-HWID-V1`) dengan bobot Motherboard (40%), CPU (35%), dan System Drive (25%).
- 100% stabil dan teruji di workspace.

### B. Linux (Priority 2) — SIAP DIIMPLEMENTASIKAN (Batch 5)
- Membaca `/etc/machine-id` atau `/var/lib/dbus/machine-id` ditambah serial root block device (`/sys/block/sda/device/serial`).
- Crate `mitra-pal` telah menyediakan `HardwareTokenType::MachineIdFile`.

### C. macOS (Priority 3) — SIAP DIIMPLEMENTASIKAN (Batch 6)
- Membaca `IOPlatformUUID` melalui IOKit registry.

### D. MitraOS Berbasis ASL Kernel (Priority 4) — KONTRAK ASL NATIVE
- **Prinsip Utama:** MitraOS **BUKAN Linux**, tidak memiliki glibc, POSIX penuh, atau `/etc/machine-id`.
- Native ID MitraOS akan membaca machine UUID langsung melalui antarmuka native kernel ASL (`HardwareTokenType::AslKernelMachineGuid`).
- Menjaga direktori `C:\asl` dan `C:\mitraOS` tetap terlindungi dan bebas dari asumsi emulator Linux.
