# PHASE N VENDOR RECOVERY STATUS
## Official Changsha SPON & Feitian Technologies Recovery Status & Action Dossier

- **Project:** Mitra Audio Network System
- **Legacy Subsystem:** SPON IPNBS v3.3.0 & Feitian ET99 USB Key
- **Primary Vendor:** Changsha SPON Communication Technology Co., Ltd. (长沙世邦通信技术有限公司)
- **Security Key Vendor:** Feitian Technologies Co., Ltd. (飞天诚信科技股份有限公司)
- **Status:** READY FOR STAKEHOLDER SUBMISSION (EXTERNAL TRANSMISSION PENDING USER APPROVAL)
- **Date:** 2026-11-12

---

## 1. Verified Vendor Contact Channels

The following corporate contact points were verified via official registry and web assets (verification date: 2026-11-12):

| Organization | Channel / Role | Endpoint / Address | Verification Source |
| :--- | :--- | :--- | :--- |
| **Changsha SPON** | Corporate HQ | SPON Science and Technology Park, High-Tech Industrial Development Zone, Changsha, Hunan, China | Official Portal (`http://www.spon.com.cn`) |
| **Changsha SPON** | Technical Support Desk | `support@spon.com.cn` / Online Service Desk | Official Product Documentation |
| **Feitian Technologies** | Global Headquarters | Tower B, Huizhi Building, No.9 Xueqing Road, Haidian District, Beijing, China | Official Portal (`https://www.ftsafe.com`) |
| **Feitian Technologies** | Hardware Support / Sales | `sales@ftsafe.com` / `support@ftsafe.com` | Product Spec Sheet (`ET99 USB Token`) |

---

## 2. Formal Vendor Recovery Package

### A. Executive System Description
- **Software Suite:** IP Network Broadcast System (IPNBS) v3.3.0.
- **Architectural Platform:** 32-bit Windows PE, Microsoft Visual C++ 6.0 / MFC.
- **Components:** `IPNBS_Server.exe`, `IPNBS_Station.exe`, `IPNBSSDK.dll`, `FT_ET99_API.dll`, `DBS.mdb`.
- **Licensing Mechanism:** Hardware dongle (Feitian ET99 USB Token) with software fallback serial verification (`5-5-5-5-4` 28-character serial hashed with MD5 at RVA `0x0043DB00`).

### B. Ownership & Entitlement Evidence Checklist
- [ ] Historical equipment procurement contract or commissioning certificate.
- [ ] Original station capacity entitlement (authorized zone/terminal count).
- [ ] System integrator identification and deployment year.
- [ ] Technical contact designation.

### C. Secure Hardware Fingerprint Transfer Procedure
To prevent exposure of operational network parameters:
- Machine identifiers are referenced in documentation via secure placeholders (`<TARGET_VOL_SERIAL>`, `<TARGET_MAC_ADDR>`).
- Actual hardware hashes must be transmitted exclusively via encrypted PGP email or authorized vendor portal.

---

## 3. Vendor Response Tracking Register

| Interaction ID | Target Vendor | Channel | Submission Date | Response Date | Status | Next Step |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `REC-SPON-001` | Changsha SPON | Technical Support Email | Pending Approval | — | `STAGED` | Awaiting customer authorization to dispatch application letter. |
| `REC-FEITIAN-001`| Feitian Tech | Hardware Sales Desk | Pending Approval | — | `STAGED` | Awaiting customer authorization for ET99 replacement inquiry. |

---

## 4. Recovery Decision Matrix

```text
                                  [ Recovery Action Initiated ]
                                                |
                     +--------------------------+--------------------------+
                     |                                                     |
         [ Official SPON Support ]                               [ Feitian Hardware ]
                     |                                                     |
         +-----------+-----------+                             +-----------+-----------+
         |                       |                             |                       |
   (Serial Issued)        (Legacy Deprecated)            (Genuine ET99 Stock)    (ET99 Discontinued)
         |                       |                             |                       |
   Apply Serial to        Transition Station             Program with SPON       Standardize fully
   Clone & Verify         Management to Mitra            PID & Verify with       on Mitra Audio Modern
   Native Activation      Audio Modern Platform          FT_ET99_API.dll         Native Stack
```

---

## 5. Security & Compliance Affirmation

In strict accordance with Phase N directives:
- Zero automated emails or web form submissions were executed.
- Zero customer machine identifiers or credentials were disclosed to public repositories.
- No claim of recovered license has been fabricated.
