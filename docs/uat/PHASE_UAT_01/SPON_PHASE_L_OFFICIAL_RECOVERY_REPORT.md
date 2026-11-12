# SPON PHASE L OFFICIAL RECOVERY REPORT
## Official Vendor Protocol & Authorization Roadmap

- **Vendor Target:** Changsha SPON Communication Technology Co., Ltd. (长沙世邦通信技术有限公司)
- **Co-Vendor (Hardware Token):** Feitian Technologies Co., Ltd. (飞天诚信科技股份有限公司)
- **Target Software:** IP Network Broadcast System (IPNBS v3.3.0)
- **Date:** 2026-11-12
- **Status:** READY FOR OFFICIAL SUBMISSION

---

## 1. System Identity Dossier

To execute authorized recovery without violating copyright or license terms, the following hardware and software parameters must be prepared for the vendor:

| Item | Parameter Value / Specification |
| :--- | :--- |
| **Product Name** | IP Network Broadcast System (IPNBS) |
| **Product Version** | Version 3.3.0 (Build 2011/2012) |
| **Server Executable** | `IPNBS_Server.exe` (PE32 GUI, MFC/MSVC++ 6.0) |
| **Server Binary SHA-256** | `6eb15234208a16dbab2f468dc2287968ff77c8e96bfadff3ea119865d2146985` |
| **Station Executable** | `IPNBS_Station.exe` (PE32 GUI, MFC/MSVC++ 6.0) |
| **Station Binary SHA-256** | `4f329971eeeb5df4545bc03975ba9550e58832a8dd9652a233b825313931be62` |
| **Primary Installer** | `SetupIPNBS_V3.3.0.exe` |
| **Installer SHA-256** | `f6a45d0ba96c738e4a9e87515f43db489e24fa547b7cffea99dfa8069502b489` |
| **Hardware Token Module** | `FT_ET99_API.dll` (Feitian ET99 USB Dongle Subsystem) |
| **Database Engine** | Microsoft Jet 4.0 (`DATABASE\DBS.mdb`) |
| **Audio Compression** | LAME MP3 Engine (`DBSEnc.dll`) |
| **Network Control Protocol**| UDP Multicast (`234.0.0.254:2046`) + Stream (`0.0.0.0:2048`) |

---

## 2. Official Vendor Recovery Questionnaire

When submitting a recovery or license re-issuance request to Changsha SPON Communication Technology Co., Ltd., the customer administrator must supply:

1. **Volume Serial Number & Machine ID:**
   - Drive C volume serial number as detected by `GetVolumeInformationA`.
   - Machine GUID / MAC Address of the primary network broadcast adapter.
2. **Historical Purchase Documentation:**
   - Original purchase invoice, equipment contract number, or project commissioning certificate for IPNBS v3.3.0.
3. **Physical Hardware Key Status:**
   - Physical Feitian ET99 dongle status: Lost, damaged, or transitioning to software license key.
4. **Target Station Capacity:**
   - Maximum registered terminal count authorized under the original project (e.g. 50, 100, 500 terminals).

---

## 3. Official Vendor Contact Details

- **Company Name:** Changsha SPON Communication Technology Co., Ltd.
- **Official Website:** `http://www.spon.com.cn`
- **Headquarters:** SPON Science and Technology Park, High-tech Industrial Development Zone, Changsha, Hunan, China
- **Domestic / International Support:** `support@spon.com.cn` / Technical Services Desk
- **Recommended Request Classification:** "Legacy IPNBS v3.3.0 System License Re-activation & Hardware Key Migration".
