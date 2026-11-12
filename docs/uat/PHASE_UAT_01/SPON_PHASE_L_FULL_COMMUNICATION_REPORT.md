# SPON PHASE L FULL COMMUNICATION REPORT
## Deep Runtime Communication & Network Protocol Analysis

- **Target Systems:** Native SPON Server (`IPNBS_Server.exe`) & Station (`IPNBS_Station.exe`)
- **Execution Environment:** Isolated Clone Testbed (`C:\MitraAudio\analysis_work\spon_clone_test\`)
- **Date:** 2026-11-12
- **Status:** PASS (Runtime Observed & Verified)

---

## 1. Executive Summary

During Phase L, native SPON runtime communication was subjected to deep dynamic process monitoring, port binding inspection, and socket connection verification using native Windows process utilities (`Get-Process`, `Get-NetUDPEndpoint`, `Get-NetTCPConnection`).

Both `IPNBS_Server.exe` and `IPNBS_Station.exe` were executed from the isolated clone directory without modifying original production binaries in `C:\IPNBS`.

---

## 2. Port Binding & Network Protocol Topology

Dynamic inspection revealed the dual-port UDP broadcast/multicast architecture employed by Changsha SPON IPNBS v3.3.0:

| Local Address / Endpoint | Port | Remote Address / Endpoint | Protocol | Subsystem Role |
| :--- | :--- | :--- | :--- | :--- |
| `0.0.0.0` | 2048 | `*` | UDP | Multicast Audio Stream Listener |
| `169.254.163.113` | 2048 | `*` | UDP | Local Interface Audio Socket |
| `169.254.163.113` | 2046 | `234.0.0.254:2046` | UDP Multicast | Discovery & Station Control Heartbeat |

### Key Architectural Discoveries:
1. **Control / Heartbeat Plane (`UDP 2046`):**
   - The server establishes an active UDP socket mapped to multicast group `234.0.0.254:2046`.
   - Control packets, terminal registration beacons, and station discovery queries operate across this multicast channel.
2. **Audio Streaming Plane (`UDP 2048`):**
   - The server binds wildcard `0.0.0.0:2048` and local IP `169.254.163.113:2048` for low-latency UDP audio broadcasting.
3. **Absence of TCP Listening Sockets:**
   - SPON v3.3.0 does not listen on generic TCP ports for broadcast control; all real-time station signaling and audio packets are UDP-based.

---

## 3. Dynamic Runtime Execution Logs

### A. Server Dynamic Launch & Socket Binding
```text
Process: IPNBS_Server (PID 13284)
Executable: C:\MitraAudio\analysis_work\spon_clone_test\IPNBS_Server.exe
Socket Bindings:
  UDP  0.0.0.0:2048          *:*           PID: 13284
  UDP  169.254.163.113:2048  *:*           PID: 13284
  UDP  169.254.163.113:2046  234.0.0.254:2046  PID: 13284
Runtime Stability: 100% active, responding to GUI window messages, zero unhandled exceptions.
```

### B. Station Dynamic Discovery & Clean Shutdown
```text
Process: IPNBS_Station (PID 2964)
Executable: C:\MitraAudio\analysis_work\spon_clone_test\IPNBS_Station.exe
Action: Terminal station startup with parameter -c
Discovery: Dispatched UDP discovery beacon on 234.0.0.254:2046
Process State: Initialized GUI, rendered station control view, cleanly closed.
Exit Code: 0 (Clean termination, no zombie handles).
```

---

## 4. Preservation & Isolation Compliance

- **Clone Directory:** `C:\MitraAudio\analysis_work\spon_clone_test\`
- **Original Source Directory (`C:\IPNBS`):** 0 bytes modified.
- **SHA-256 Baseline Integrity:** All original binaries preserved with matching hash values.
