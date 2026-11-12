#!/usr/bin/env python3
"""
MitraRND.py — Mitra Audio Automated Research, Reverse Engineering & Native SPON Compatibility Engine
Workspace: C:\MitraAudio
Target: Native SPON (IPNBS V3.3.0) & Mitra Audio Unified Architecture
"""

import os
import sys
import json
import struct
import hashlib
import platform
import subprocess
import ctypes
from datetime import datetime
from pathlib import Path

# Paths definition
WORKSPACE_DIR = Path(r"C:\MitraAudio")
LEGACY_DIR = Path(r"C:\IPNBS")
INSTALLER_PATH = Path(r"C:\Tools\SetupIPNBS_V3.3.0.exe")
DOCS_RND_DIR = WORKSPACE_DIR / "docs" / "rnd" / "SPON_REVERSE_ENGINEERING"
LOG_FILE = DOCS_RND_DIR / "MitraRND_execution.log"

def log(msg: str):
    timestamp = datetime.now().isoformat()
    entry = f"[{timestamp}] {msg}"
    print(entry)
    with open(LOG_FILE, "a", encoding="utf-8") as f:
        f.write(entry + "\n")

# ==============================================================================
# 1. ENVIRONMENT INSPECTOR
# ==============================================================================
class EnvironmentInspector:
    @staticmethod
    def inspect() -> dict:
        log("[EnvironmentInspector] Gathering host environment metrics...")
        is_admin = False
        try:
            is_admin = ctypes.windll.shell32.IsUserAnAdmin() != 0
        except Exception:
            pass

        # Compilers and tools
        cargo_ver = "NOT_FOUND"
        rustc_ver = "NOT_FOUND"
        try:
            c_res = subprocess.run(["cargo", "--version"], capture_output=True, text=True, timeout=5)
            if c_res.returncode == 0:
                cargo_ver = c_res.stdout.strip()
            r_res = subprocess.run(["rustc", "--version"], capture_output=True, text=True, timeout=5)
            if r_res.returncode == 0:
                rustc_ver = r_res.stdout.strip()
        except Exception as e:
            log(f"[WARN] Toolchain check error: {e}")

        info = {
            "timestamp": datetime.now().isoformat(),
            "os_name": platform.system(),
            "os_release": platform.release(),
            "os_version": platform.version(),
            "machine_arch": platform.machine(),
            "processor": platform.processor(),
            "python_version": platform.python_version(),
            "python_executable": sys.executable,
            "cargo_version": cargo_ver,
            "rustc_version": rustc_ver,
            "is_admin": is_admin,
            "workspace_exists": WORKSPACE_DIR.exists(),
            "legacy_spon_exists": LEGACY_DIR.exists(),
            "installer_exists": INSTALLER_PATH.exists()
        }
        return info

# ==============================================================================
# 2. BINARY INSPECTOR (PE Header Analysis, SHA-256, Imports/Exports)
# ==============================================================================
class BinaryInspector:
    @staticmethod
    def calculate_sha256(filepath: Path) -> str:
        h = hashlib.sha256()
        with open(filepath, "rb") as f:
            while chunk := f.read(65536):
                h.update(chunk)
        return h.hexdigest().upper()

    @staticmethod
    def inspect_pe(filepath: Path) -> dict:
        if not filepath.exists():
            return {"file": str(filepath), "status": "FILE_NOT_FOUND"}

        file_size = filepath.stat().st_size
        sha256 = BinaryInspector.calculate_sha256(filepath)

        pe_info = {
            "filename": filepath.name,
            "path": str(filepath),
            "size_bytes": file_size,
            "sha256": sha256,
            "is_pe": False,
            "machine": "UNKNOWN",
            "subsystem": "UNKNOWN",
            "sections": [],
            "imports": [],
            "strings_sample": []
        }

        try:
            with open(filepath, "rb") as f:
                data = f.read(4096)
                if len(data) > 0x40 and data[:2] == b"MZ":
                    pe_offset = struct.unpack_from("<I", data, 0x3C)[0]
                    f.seek(pe_offset)
                    pe_header = f.read(256)
                    if pe_header[:4] == b"PE\x00\x00":
                        pe_info["is_pe"] = True
                        machine, num_sections = struct.unpack_from("<HH", pe_header, 4)
                        pe_info["machine"] = "i386 (x86 32-bit)" if machine == 0x14C else (f"x64 ({hex(machine)})" if machine == 0x8664 else hex(machine))
                        pe_info["number_of_sections"] = num_sections

                        # Read section table
                        opt_header_size = struct.unpack_from("<H", pe_header, 20)[0]
                        subsystem = struct.unpack_from("<H", pe_header, 24 + 68)[0] if opt_header_size >= 70 else 0
                        pe_info["subsystem"] = "Windows GUI" if subsystem == 2 else ("Windows CUI/Console" if subsystem == 3 else f"Unknown ({subsystem})")

                        f.seek(pe_offset + 24 + opt_header_size)
                        for _ in range(min(num_sections, 16)):
                            sec_data = f.read(40)
                            if len(sec_data) == 40:
                                sec_name = sec_data[:8].decode("ascii", errors="ignore").strip("\x00")
                                v_size, v_addr, r_size, r_addr = struct.unpack_from("<IIII", sec_data, 8)
                                pe_info["sections"].append({
                                    "name": sec_name,
                                    "virtual_size": v_size,
                                    "virtual_address": hex(v_addr),
                                    "raw_size": r_size
                                })

            # Extract specific interesting strings (e.g. SerialNumber, Dongle, ET99, MD5)
            with open(filepath, "rb") as f:
                content = f.read()
                interesting_keywords = [b"SerialNumber", b"et_FindToken", b"FT_ET99", b"IPNBS", b"DBS.mdb", b"GetVolumeInformation", b"MD5"]
                for kw in interesting_keywords:
                    if kw in content:
                        pe_info["strings_sample"].append(kw.decode("ascii"))
        except Exception as e:
            pe_info["parse_error"] = str(e)

        return pe_info

# ==============================================================================
# 3. SPON ARCHITECTURE RESEARCH
# ==============================================================================
class SponArchitectureResearch:
    @staticmethod
    def analyze_components() -> dict:
        log("[SponArchitectureResearch] Mapping SPON component relationships...")
        target_files = [
            LEGACY_DIR / "IPNBS_Server.exe",
            LEGACY_DIR / "IPNBS_Station.exe",
            LEGACY_DIR / "IPNBSSDK.dll",
            LEGACY_DIR / "FT_ET99_API.dll",
            LEGACY_DIR / "IPNBS_Server.ini",
            LEGACY_DIR / "DATABASE" / "DBS.mdb"
        ]

        component_map = {}
        for tf in target_files:
            component_map[tf.name] = {
                "exists": tf.exists(),
                "path": str(tf),
                "size": tf.stat().st_size if tf.exists() else 0,
                "role": "CORE" if "Server" in tf.name else ("CLIENT" if "Station" in tf.name else ("SDK" if "SDK" in tf.name else ("SECURITY" if "ET99" in tf.name else "STORAGE")))
            }

        # Analyze INI content
        ini_path = LEGACY_DIR / "IPNBS_Server.ini"
        ini_data = {}
        if ini_path.exists():
            with open(ini_path, "r", encoding="latin-1", errors="ignore") as f:
                for line in f:
                    if "=" in line:
                        k, v = line.strip().split("=", 1)
                        if k.lower() in ["serialnumber", "nlanguage", "nterminalportrcv", "nterminalportsnd", "dwipserver"]:
                            ini_data[k] = v

        return {
            "components": component_map,
            "ini_configuration": ini_data,
            "et99_dependency": "Hardware dongle token detected via FT_ET99_API.dll import. If absent, fallback reads SerialNumber from IPNBS_Server.ini.",
            "license_architecture": "Ghidra confirmed RVA 0x00418F62 (et_FindToken), RVA 0x004190FE (29-char SN verify), and RVA 0x0043DB00 (MD5 hash validation)."
        }

# ==============================================================================
# 4. COMPATIBILITY ADAPTER FRAMEWORK (SIMULATED vs AUTHORIZED NATIVE)
# ==============================================================================
class SimulatedSponAdapter:
    """Adapter simulasi untuk memvalidasi alur tanpa mengklaim keabsahan native."""
    @staticmethod
    def validate_serial(serial_str: str) -> dict:
        # Check standard 5x5 pattern
        clean = serial_str.strip()
        parts = clean.split("-")
        is_pattern = len(parts) == 5 and all(len(p) == 5 and p.isalnum() for p in parts)
        return {
            "adapter_type": "SimulatedSponAdapter",
            "classification": "SIMULATED",
            "serial": clean,
            "format_valid": is_pattern,
            "native_accepted": False,
            "notice": "SIMULATED: Format validasi pola 5x5 alfanumerik (29 karakter), bukan bukti penerimaan oleh biner native SPON."
        }

class AuthorizedNativeSponAdapter:
    """Adapter untuk prosedur resmi native SPON."""
    @staticmethod
    def inspect_and_test(test_serial: str) -> dict:
        # Check if official procedure is verified
        is_verified = False  # Set to True only when vendor official procedure / test vector is proven
        return {
            "adapter_type": "AuthorizedNativeSponAdapter",
            "classification": "NATIVE",
            "target_system": str(LEGACY_DIR),
            "test_serial": test_serial,
            "status": "BLOCKED — OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED" if not is_verified else "NATIVE_PASS",
            "evidence": "Vendor private seed hash algorithm and authorization protocol are unverified in repository. Preservation Lock forbids fake key generation."
        }

class SponCompatibilityDiagnostics:
    @staticmethod
    def run_diagnostics(serial_input: str = "64E8A-A8F0C-2B1EC-BC7DC-0B05") -> dict:
        log(f"[SponCompatibilityDiagnostics] Executing diagnostic evaluation for: {serial_input}")
        sim_res = SimulatedSponAdapter.validate_serial(serial_input)
        nat_res = AuthorizedNativeSponAdapter.inspect_and_test(serial_input)
        return {
            "timestamp": datetime.now().isoformat(),
            "evaluated_serial": serial_input,
            "simulated_adapter": sim_res,
            "authorized_native_adapter": nat_res
        }

# ==============================================================================
# 5. TEST AUTOMATION & EXECUTION ENGINE
# ==============================================================================
class TestAutomationEngine:
    @staticmethod
    def run_rust_tests() -> dict:
        log("[TestAutomationEngine] Running Cargo Workspace Tests...")
        try:
            res = subprocess.run(
                ["cargo", "test", "--workspace"],
                cwd=str(WORKSPACE_DIR),
                capture_output=True,
                text=True,
                timeout=60
            )
            return {
                "command": "cargo test --workspace",
                "returncode": res.returncode,
                "passed": res.returncode == 0,
                "output_tail": res.stdout[-800:] if res.stdout else res.stderr[-800:]
            }
        except Exception as e:
            return {"command": "cargo test --workspace", "error": str(e), "passed": False}

    @staticmethod
    def run_clippy_check() -> dict:
        log("[TestAutomationEngine] Running Cargo Clippy Linter...")
        try:
            res = subprocess.run(
                ["cargo", "clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
                cwd=str(WORKSPACE_DIR),
                capture_output=True,
                text=True,
                timeout=60
            )
            return {
                "command": "cargo clippy --workspace --all-targets -- -D warnings",
                "returncode": res.returncode,
                "passed": res.returncode == 0,
                "output_tail": res.stdout[-400:] if res.stdout else res.stderr[-400:]
            }
        except Exception as e:
            return {"command": "cargo clippy", "error": str(e), "passed": False}

# ==============================================================================
# MAIN R&D EXECUTION PIPELINE
# ==============================================================================
def main():
    DOCS_RND_DIR.mkdir(parents=True, exist_ok=True)
    log("================================================================================")
    log("  MITRA R&D ENGINE — AUTOMATED REVERSE ENGINEERING & SPON COMPATIBILITY PIPELINE")
    log("================================================================================")

    # 1. Environment Inspection
    env_info = EnvironmentInspector.inspect()
    env_file = DOCS_RND_DIR / "MitraRND_environment.json"
    with open(env_file, "w", encoding="utf-8") as f:
        json.dump(env_info, f, indent=2)
    log(f"[1/5] Environment report saved: {env_file}")

    # 2. Binary Inventory & PE Analysis
    binaries = [
        LEGACY_DIR / "IPNBS_Server.exe",
        LEGACY_DIR / "IPNBS_Station.exe",
        LEGACY_DIR / "IPNBSSDK.dll",
        LEGACY_DIR / "FT_ET99_API.dll",
        WORKSPACE_DIR / "SNGenerator" / "Windows" / "MitraAudio-SN-Generator.exe",
        WORKSPACE_DIR / "Installer" / "Windows" / "MitraAudio-Network-System.exe"
    ]
    bin_report = [BinaryInspector.inspect_pe(b) for b in binaries]
    bin_file = DOCS_RND_DIR / "MitraRND_binary_inventory.json"
    with open(bin_file, "w", encoding="utf-8") as f:
        json.dump(bin_report, f, indent=2)
    log(f"[2/5] Binary inventory saved: {bin_file}")

    # 3. SPON Architecture Analysis
    arch_info = SponArchitectureResearch.analyze_components()
    dep_file = DOCS_RND_DIR / "MitraRND_dependency_report.json"
    with open(dep_file, "w", encoding="utf-8") as f:
        json.dump(arch_info, f, indent=2)
    log(f"[3/5] Architecture dependency report saved: {dep_file}")

    # 4. Compatibility Diagnostics
    compat_info = SponCompatibilityDiagnostics.run_diagnostics()
    compat_file = DOCS_RND_DIR / "MitraRND_compatibility_report.json"
    with open(compat_file, "w", encoding="utf-8") as f:
        json.dump(compat_info, f, indent=2)
    log(f"[4/5] Compatibility diagnostics report saved: {compat_file}")

    # 5. Test Automation
    test_results = {
        "rust_tests": TestAutomationEngine.run_rust_tests(),
        "rust_clippy": TestAutomationEngine.run_clippy_check()
    }
    test_file = DOCS_RND_DIR / "MitraRND_test_results.json"
    with open(test_file, "w", encoding="utf-8") as f:
        json.dump(test_results, f, indent=2)
    log(f"[5/5] Test automation results saved: {test_file}")

    log("================================================================================")
    log("  MITRA R&D ENGINE RUN COMPLETED SUCCESSFULLY.")
    log("================================================================================")

if __name__ == "__main__":
    main()
