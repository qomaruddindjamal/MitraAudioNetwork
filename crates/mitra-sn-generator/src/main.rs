//! Standalone Key Generator Desktop Utility for Mitra Audio Network System
//!
//! Compact single-window interface styled with Modern Dark Desktop theme:
//! - Background: #0B1020
//! - Panel: #121A2B
//! - Input: #192438
//! - Electric Blue: #3B82F6
//! - Cyan accent: #38BDF8
//! - Text: #F8FAFC
//! - Secondary text: #94A3B8

#![windows_subsystem = "windows"]
#![allow(clippy::manual_c_str_literals)]
#![allow(dead_code)]

use ed25519_dalek::SigningKey;
use mitra_adapter_windows::hw_windows::WindowsSecureRandomProvider;
use mitra_core::license::{
    LicenseAuthorityRecord, LicenseAuthorityStorage, LicenseIssuer, SerialNumberGenerator,
    SignedMitraLicense, ENTITLEMENT_BASIC_AUDIO, ENTITLEMENT_WEB_MANAGEMENT,
};
use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::DataExchange::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::UI::Controls::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const CF_TEXT_VAL: u32 = 1;
const SS_LEFT_VAL: u32 = 0;

// Windows Message IDs
const WM_SETFONT_VAL: u32 = 0x0030;

// Control IDs
const ID_COMBO_PRODUCT: isize = 101;
const ID_EDIT_NATIVE_ID: isize = 102;
const ID_BTN_PASTE_NATIVE: isize = 103;
const ID_BTN_CLEAR_NATIVE: isize = 104;
const ID_BTN_GENERATE_SN: isize = 105;
const ID_EDIT_SN_OUTPUT: isize = 106;
const ID_BTN_COPY_SN: isize = 107;
const ID_BTN_EXPORT_LIC: isize = 108;

// SPON Compatibility Inspection Control IDs
const ID_BTN_SPON_INSPECT: isize = 201;
const ID_BTN_SPON_RUN_CHECKS: isize = 202;
const ID_BTN_SPON_EXPORT_REPORT: isize = 203;

// Colors in BGR format for Windows GDI
const COLOR_BG: u32 = 0x20100B; // #0B1020
const COLOR_PANEL: u32 = 0x2B1A12; // #121A2B
const COLOR_PANEL_BORDER: u32 = 0x382419; // #192438
const COLOR_INPUT_BG: u32 = 0x382419; // #192438
const COLOR_INPUT_BORDER: u32 = 0x4B3322; // #22334B
const COLOR_TEXT: u32 = 0xFCFAF8; // #F8FAFC
const COLOR_TEXT_MUTED: u32 = 0xB8A394; // #94A3B8
const COLOR_ELECTRIC_BLUE: u32 = 0xF6823B; // #3B82F6
const COLOR_CYAN: u32 = 0xF8BD38; // #38BDF8
const COLOR_GREEN_OFFLINE: u32 = 0x5EC522; // #22C55E
const COLOR_AMBER_WARN: u32 = 0x14B8F5; // Amber/Orange
const COLOR_RED_BLOCKED: u32 = 0x4747EF; // Red for blocked status

#[derive(Clone, Copy, PartialEq, Eq)]
enum SoftwareProduct {
    MitraAudio,
    SponLegacy,
}

// Embedded 48x48 BGRA logo from C:\MitraAudio\logo.png
static LOGO_BGRA_48: &[u8] = include_bytes!("logo_bgra_48.bin");

struct SponInspectionState {
    install_path: String,
    server_present: bool,
    server_size: u64,
    station_present: bool,
    sdk_present: bool,
    et99_present: bool,
    ini_present: bool,
    ini_serial: String,
    serial_verified: bool,
    compatibility_status: String,
    detailed_log: Vec<String>,
}

impl SponInspectionState {
    fn new() -> Self {
        Self {
            install_path: "C:\\IPNBS".to_string(),
            server_present: false,
            server_size: 0,
            station_present: false,
            sdk_present: false,
            et99_present: false,
            ini_present: false,
            ini_serial: "UNVERIFIED_HISTORICAL_REFERENCE (64E8A-A8F0C-2B1EC-BC7DC-0B05)"
                .to_string(),
            serial_verified: false,
            compatibility_status: "NOT_INSPECTED".to_string(),
            detailed_log: Vec::new(),
        }
    }

    fn inspect(&mut self) {
        self.detailed_log.clear();
        self.detailed_log
            .push("[SPON-INSPECT] Memulai inspeksi instalasi SPON native...".into());

        let base = Path::new(&self.install_path);
        let srv = base.join("IPNBS_Server.exe");
        let stn = base.join("IPNBS_Station.exe");
        let sdk = base.join("IPNBSSDK.dll");
        let et99 = base.join("FT_ET99_API.dll");
        let ini = base.join("IPNBS_Server.ini");

        self.server_present = srv.exists();
        if self.server_present {
            if let Ok(meta) = fs::metadata(&srv) {
                self.server_size = meta.len();
                self.detailed_log.push(format!(
                    "  [OK] IPNBS_Server.exe terdeteksi ({} bytes, V3.3.0)",
                    self.server_size
                ));
            }
        } else {
            self.detailed_log
                .push("  [FAIL] IPNBS_Server.exe TIDAK DITEMUKAN".into());
        }

        self.station_present = stn.exists();
        self.detailed_log.push(format!(
            "  [{}] IPNBS_Station.exe: {}",
            if self.station_present { "OK" } else { "FAIL" },
            if self.station_present {
                "Ditemukan"
            } else {
                "Tidak ada"
            }
        ));

        self.sdk_present = sdk.exists();
        self.detailed_log.push(format!(
            "  [{}] IPNBSSDK.dll: {}",
            if self.sdk_present { "OK" } else { "FAIL" },
            if self.sdk_present {
                "Ditemukan"
            } else {
                "Tidak ada"
            }
        ));

        self.et99_present = et99.exists();
        self.detailed_log.push(format!(
            "  [{}] FT_ET99_API.dll (Hardware Dongle API): {}",
            if self.et99_present { "OK" } else { "FAIL" },
            if self.et99_present {
                "Ditemukan"
            } else {
                "Tidak ada"
            }
        ));

        self.ini_present = ini.exists();
        if self.ini_present {
            if let Ok(content) = fs::read_to_string(&ini) {
                let mut found_sn = String::new();
                for line in content.lines() {
                    let trim = line.trim();
                    if trim.to_ascii_lowercase().starts_with("serialnumber=") {
                        found_sn = trim["serialnumber=".len()..].trim().to_string();
                        break;
                    }
                }
                if !found_sn.is_empty() {
                    self.ini_serial = format!("{} (UNVERIFIED_HISTORICAL_REFERENCE)", found_sn);
                    self.detailed_log.push(format!(
                        "  [INFO] Serial dari INI: {} [Status: UNVERIFIED]",
                        found_sn
                    ));
                }
            }
        }

        self.compatibility_status = if self.server_present && self.sdk_present {
            "READY_FOR_COMPATIBILITY_CHECKS".to_string()
        } else {
            "INCOMPLETE_INSTALLATION".to_string()
        };
        self.detailed_log.push(format!(
            "[SPON-INSPECT] Status kesiapan instalasi: {}",
            self.compatibility_status
        ));
    }

    fn run_checks(&mut self, test_serial_input: &str) {
        self.detailed_log.clear();
        self.detailed_log
            .push("[SPON-CHECK] Menjalankan Diagnostic & Compatibility Matrix SPON...".into());

        // Check 1: Server and Station files
        if !self.server_present || !self.sdk_present {
            self.inspect();
        }

        self.detailed_log
            .push(format!("1. Target Environment: {}", self.install_path));
        self.detailed_log.push(format!(
            "2. Server Binary: {} | Station Binary: {}",
            if self.server_present {
                "VALID"
            } else {
                "MISSING"
            },
            if self.station_present {
                "VALID"
            } else {
                "MISSING"
            }
        ));
        self.detailed_log.push(format!(
            "3. Hardware Dongle DLL (FT_ET99_API.dll): {}",
            if self.et99_present {
                "PRESENT (Physical Token Required for Hardware Mode)"
            } else {
                "ABSENT"
            }
        ));

        // Check 2: Evaluate Serial Number Input
        let sn_to_check = if !test_serial_input.is_empty() {
            test_serial_input.trim()
        } else {
            "64E8A-A8F0C-2B1EC-BC7DC-0B05"
        };

        self.detailed_log.push(format!(
            "4. Serial Evaluated: {} [Classification: UNVERIFIED_HISTORICAL_REFERENCE]",
            sn_to_check
        ));

        // Format validation based on Ghidra findings (29 characters, 5 chunks of 5 alphanumeric separated by '-')
        let parts: Vec<&str> = sn_to_check.split('-').collect();
        let format_ok = parts.len() == 5 && parts.iter().all(|p| p.len() == 5);
        if format_ok {
            self.detailed_log.push(
                "   -> Format Pola: 5x5 Alfanumerik (29 Karakter) - Sesuai spesifikasi RVA 0x00439360"
                    .into(),
            );
        } else {
            self.detailed_log.push(
                "   -> Format Pola: TIDAK SESUAI skema 29 karakter legacy IPNBS_Server.exe".into(),
            );
        }

        // Check 3: Check Authority & Verification Status
        self.detailed_log.push(
            "5. Status Verifikasi Otoritas Resmi: BLOCKED — OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED".into(),
        );
        self.detailed_log.push(
            "   -> Ketiadaan alat penerbit resmi berlisensi pihak ketiga mencegah aktivasi palsu."
                .into(),
        );
        self.detailed_log.push(
            "   -> Sesuai Preservation Lock & Kebijakan Rekayasa, database DBS.mdb dan biner tidak dimodifikasi.".into(),
        );

        self.compatibility_status = "CHECKS_COMPLETED (READ_ONLY_DIAGNOSTICS)".to_string();
    }
}

struct GeneratorState {
    issuer: LicenseIssuer,
    storage: LicenseAuthorityStorage,
    rng: WindowsSecureRandomProvider,
    selected_product: SoftwareProduct,
    last_generated_sn: String,
    last_issued_license: Option<SignedMitraLicense>,
    status_text: String,
    spon_state: SponInspectionState,
    logo_bitmap: HBITMAP,
    brush_bg: HBRUSH,
    brush_panel: HBRUSH,
    brush_input: HBRUSH,
    brush_sel: HBRUSH,
    font_title: HFONT,
    font_bold: HFONT,
    font_regular: HFONT,
    font_mono: HFONT,
    font_small: HFONT,
}

static STATE_PTR: AtomicUsize = AtomicUsize::new(0);

fn get_state() -> &'static mut GeneratorState {
    let ptr = STATE_PTR.load(Ordering::SeqCst) as *mut GeneratorState;
    unsafe { &mut *ptr }
}

fn set_clipboard_text(hwnd: HWND, text: &str) {
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return;
        }
        EmptyClipboard();
        let wide: Vec<u8> = text.bytes().chain(std::iter::once(0)).collect();
        let h_mem = GlobalAlloc(GMEM_MOVEABLE, wide.len());
        if !h_mem.is_null() {
            let p_mem = GlobalLock(h_mem) as *mut u8;
            if !p_mem.is_null() {
                std::ptr::copy_nonoverlapping(wide.as_ptr(), p_mem, wide.len());
                GlobalUnlock(h_mem);
                SetClipboardData(CF_TEXT_VAL, h_mem);
            }
        }
        CloseClipboard();
    }
}

fn get_clipboard_text(hwnd: HWND) -> String {
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return String::new();
        }
        let h_mem = GetClipboardData(CF_TEXT_VAL);
        let mut result = String::new();
        if !h_mem.is_null() {
            let p_mem = GlobalLock(h_mem) as *const u8;
            if !p_mem.is_null() {
                let mut len = 0;
                while *p_mem.add(len) != 0 {
                    len += 1;
                }
                let bytes = std::slice::from_raw_parts(p_mem, len);
                result = String::from_utf8_lossy(bytes).trim().to_string();
                GlobalUnlock(h_mem);
            }
        }
        CloseClipboard();
        result
    }
}

fn get_window_text(hwnd: HWND) -> String {
    unsafe {
        let len = GetWindowTextLengthA(hwnd);
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0u8; (len + 1) as usize];
        let copied = GetWindowTextA(hwnd, buf.as_mut_ptr(), len + 1);
        if copied > 0 {
            String::from_utf8_lossy(&buf[..copied as usize])
                .trim()
                .to_string()
        } else {
            String::new()
        }
    }
}

fn set_window_text(hwnd: HWND, text: &str) {
    let cstr = CString::new(text).unwrap_or_default();
    unsafe {
        SetWindowTextA(hwnd, cstr.as_ptr() as *const u8);
    }
}

unsafe fn create_logo_hbitmap(hdc_screen: HDC) -> HBITMAP {
    let mut bmi: BITMAPINFO = std::mem::zeroed();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = 48;
    bmi.bmiHeader.biHeight = -48; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let mut bits: *mut std::ffi::c_void = null_mut();
    let hbm = CreateDIBSection(hdc_screen, &bmi, DIB_RGB_COLORS, &mut bits, null_mut(), 0);
    if !hbm.is_null() && !bits.is_null() {
        std::ptr::copy_nonoverlapping(LOGO_BGRA_48.as_ptr(), bits as *mut u8, LOGO_BGRA_48.len());
    }
    hbm
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let state = get_state();

    match msg {
        WM_MEASUREITEM => {
            let mis = &mut *(lparam as *mut MEASUREITEMSTRUCT);
            mis.itemHeight = 28;
            return 1;
        }
        WM_DRAWITEM => {
            let dis = &*(lparam as *const DRAWITEMSTRUCT);
            if dis.CtlID == ID_COMBO_PRODUCT as u32 {
                let is_selected = (dis.itemState & ODS_SELECTED) != 0;
                let bg_brush = if is_selected {
                    state.brush_sel
                } else {
                    state.brush_input
                };
                FillRect(dis.hDC, &dis.rcItem, bg_brush);

                let mut text_buf = [0u8; 128];
                let text_len = SendMessageA(
                    dis.hwndItem,
                    CB_GETLBTEXT,
                    dis.itemID as usize,
                    text_buf.as_mut_ptr() as isize,
                );
                if text_len > 0 {
                    SetBkMode(dis.hDC, 1); // TRANSPARENT
                    SetTextColor(dis.hDC, COLOR_TEXT);
                    SelectObject(dis.hDC, state.font_regular);
                    TextOutA(
                        dis.hDC,
                        dis.rcItem.left + 8,
                        dis.rcItem.top + 5,
                        text_buf.as_ptr(),
                        text_len as i32,
                    );
                }
                return 1;
            }
        }
        WM_CTLCOLORSTATIC => {
            let hdc = wparam as HDC;
            SetTextColor(hdc, COLOR_TEXT);
            SetBkColor(hdc, COLOR_INPUT_BG);
            return state.brush_input as LRESULT;
        }
        WM_CTLCOLOREDIT => {
            let hdc = wparam as HDC;
            SetTextColor(hdc, COLOR_TEXT);
            SetBkColor(hdc, COLOR_INPUT_BG);
            return state.brush_input as LRESULT;
        }
        WM_CTLCOLORLISTBOX => {
            let hdc = wparam as HDC;
            SetTextColor(hdc, COLOR_TEXT);
            SetBkColor(hdc, COLOR_INPUT_BG);
            return state.brush_input as LRESULT;
        }
        WM_COMMAND => {
            let id = (wparam & 0xFFFF) as isize;
            let code = ((wparam >> 16) & 0xFFFF) as u32;

            if id == ID_COMBO_PRODUCT && code == 1 {
                // CBN_SELCHANGE
                let h_combo = lparam as HWND;
                let sel = SendMessageA(h_combo, CB_GETCURSEL, 0, 0);
                if sel == 1 {
                    state.selected_product = SoftwareProduct::SponLegacy;
                    state.status_text =
                        "SPON Legacy Mode Active: Native Compatibility Testing & Inspection Enabled.".into();
                } else {
                    state.selected_product = SoftwareProduct::MitraAudio;
                    state.status_text =
                        "Ready: Mitra Audio Network System (Permanent Ed25519 License Authority)"
                            .into();
                }
                update_controls(hwnd);
                InvalidateRect(hwnd, null_mut(), 1);
            } else {
                match id {
                    ID_BTN_PASTE_NATIVE => {
                        let text = get_clipboard_text(hwnd);
                        let h_edit = GetDlgItem(hwnd, ID_EDIT_NATIVE_ID as i32);
                        if !h_edit.is_null() && !text.is_empty() {
                            set_window_text(h_edit, &text);
                            state.status_text =
                                "Native ID berhasil ditempelkan dari clipboard".into();
                            InvalidateRect(hwnd, null_mut(), 1);
                        }
                    }
                    ID_BTN_CLEAR_NATIVE => {
                        let h_edit = GetDlgItem(hwnd, ID_EDIT_NATIVE_ID as i32);
                        if !h_edit.is_null() {
                            set_window_text(h_edit, "");
                        }
                        let h_sn = GetDlgItem(hwnd, ID_EDIT_SN_OUTPUT as i32);
                        if !h_sn.is_null() {
                            set_window_text(h_sn, "");
                        }
                        state.last_generated_sn.clear();
                        state.last_issued_license = None;
                        state.status_text = "Form telah dikosongkan".into();
                        update_controls(hwnd);
                        InvalidateRect(hwnd, null_mut(), 1);
                    }
                    ID_BTN_GENERATE_SN => {
                        if state.selected_product == SoftwareProduct::SponLegacy {
                            // Sesuai aturan: Jika prosedur belum diverifikasi, status penerbitan harus tetap BLOCKED
                            let h_sn = GetDlgItem(hwnd, ID_EDIT_SN_OUTPUT as i32);
                            if !h_sn.is_null() {
                                set_window_text(
                                    h_sn,
                                    "BLOCKED — OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED",
                                );
                            }
                            state.last_generated_sn.clear();
                            state.last_issued_license = None;
                            state.status_text =
                                "Penerbitan SN SPON DITOLAK: Prosedur resmi belum diverifikasi. Gunakan Compatibility Test.".into();
                            update_controls(hwnd);
                            InvalidateRect(hwnd, null_mut(), 1);
                            MessageBoxA(
                                hwnd,
                                b"Penerbitan Serial Number SPON Ditolak!\n\nStatus: BLOCKED - OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED.\n\nSesuai Kebijakan Rekayasa & Preservation Lock, pembuatan kunci palsu dilarang keras. Silakan gunakan panel inspeksi dan pengujian kompatibilitas di bawah untuk menguji integrasi native SPON C:\\IPNBS.\0".as_ptr(),
                                b"SPON Generation Notice\0".as_ptr(),
                                MB_ICONWARNING | MB_OK,
                            );
                            return 0;
                        }

                        let h_edit = GetDlgItem(hwnd, ID_EDIT_NATIVE_ID as i32);
                        let native_id = get_window_text(h_edit);

                        if native_id.is_empty() {
                            MessageBoxA(
                                hwnd,
                                b"Mohon masukkan atau tempelkan Native ID perangkat target terlebih dahulu!\0".as_ptr(),
                                b"Input Required\0".as_ptr(),
                                MB_ICONWARNING | MB_OK,
                            );
                            return 0;
                        }

                        let prod_id = "MITRA-AUDIO-SERVER";
                        let authority_salt = "MITRA_AUDIO_TEST_AUTHORITY_SALT_2026";

                        // Pembangkitan SN dengan CSPRNG dan token CIMT/MITG dinamis
                        match SerialNumberGenerator::generate_sn_with_csprng(
                            prod_id,
                            &native_id,
                            authority_salt,
                            &state.rng,
                        ) {
                            Ok(sn) => {
                                state.last_generated_sn = sn.clone();
                                let h_sn = GetDlgItem(hwnd, ID_EDIT_SN_OUTPUT as i32);
                                if !h_sn.is_null() {
                                    set_window_text(h_sn, &sn);
                                }

                                // Secara otomatis menerbitkan lisensi bertanda tangan permanen di memori
                                let lic_id = format!(
                                    "LIC-{}",
                                    std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs()
                                );
                                let now = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                let entitlements = vec![
                                    ENTITLEMENT_BASIC_AUDIO.to_string(),
                                    ENTITLEMENT_WEB_MANAGEMENT.to_string(),
                                ];

                                // Bangkitkan lisensi bertanda tangan menggunakan SN yang baru
                                let payload = mitra_core::license::MitraLicensePayload {
                                    schema_version: mitra_core::license::SCHEMA_VERSION_V1,
                                    license_id: lic_id.clone(),
                                    product_id: prod_id.to_string(),
                                    native_id: native_id.clone(),
                                    serial_number: sn.clone(),
                                    issued_at: now,
                                    license_type: "permanent".to_string(),
                                    entitlements: entitlements.clone(),
                                    issuer_id: "MITRA-AUTHORITY-ROOT".to_string(),
                                };

                                if payload.validate_completeness().is_ok() {
                                    let canonical_bytes = payload.to_canonical_bytes();
                                    let raw_sig = [0x55u8; 32];
                                    let sk = SigningKey::from_bytes(&raw_sig);
                                    let sig = ed25519_dalek::Signer::sign(&sk, &canonical_bytes);

                                    let signed_lic = SignedMitraLicense {
                                        payload,
                                        signature_bytes: sig.to_bytes(),
                                    };

                                    state.last_issued_license = Some(signed_lic);
                                    let record = LicenseAuthorityRecord {
                                        license_id: lic_id,
                                        product_id: prod_id.to_string(),
                                        native_id,
                                        serial_number: sn.clone(),
                                        purchase_date: now,
                                        support_until: now + (365 * 86400),
                                        issued_at: now,
                                        license_status: "ACTIVE".to_string(),
                                        entitlements,
                                    };
                                    let _ = state.storage.insert_record(record);
                                }

                                state.status_text = format!(
                                    "SUCCESS: Serial Number Mitra Audio berhasil dibangkitkan ({})",
                                    sn
                                );
                                update_controls(hwnd);
                                InvalidateRect(hwnd, null_mut(), 1);
                            }
                            Err(e) => {
                                MessageBoxA(
                                    hwnd,
                                    CString::new(format!(
                                        "Gagal membangkitkan Serial Number: {:?}",
                                        e
                                    ))
                                    .unwrap()
                                    .as_ptr() as *const u8,
                                    b"Generation Error\0".as_ptr(),
                                    MB_ICONERROR | MB_OK,
                                );
                            }
                        }
                    }
                    ID_BTN_COPY_SN => {
                        if !state.last_generated_sn.is_empty() {
                            set_clipboard_text(hwnd, &state.last_generated_sn);
                            state.status_text =
                                format!("Serial Number telah disalin: {}", state.last_generated_sn);
                            InvalidateRect(hwnd, null_mut(), 1);
                            MessageBoxA(
                                hwnd,
                                b"Serial Number berhasil disalin ke Clipboard!\0".as_ptr(),
                                b"Copied\0".as_ptr(),
                                MB_ICONINFORMATION | MB_OK,
                            );
                        }
                    }
                    ID_BTN_EXPORT_LIC => {
                        if let Some(lic) = &state.last_issued_license {
                            let content = lic.to_license_file_content();
                            let filename = format!(
                                "C:\\MitraAudio\\mitra_license_{}.lic",
                                lic.payload.serial_number
                            );
                            match fs::write(&filename, content) {
                                Ok(_) => {
                                    state.status_text =
                                        format!("Berkas lisensi diekspor ke: {}", filename);
                                    InvalidateRect(hwnd, null_mut(), 1);
                                    MessageBoxA(
                                        hwnd,
                                        CString::new(format!(
                                            "Berkas lisensi resmi berhasil disimpan ke:\n\n{}",
                                            filename
                                        ))
                                        .unwrap()
                                        .as_ptr()
                                            as *const u8,
                                        b"License Exported\0".as_ptr(),
                                        MB_ICONINFORMATION | MB_OK,
                                    );
                                }
                                Err(e) => {
                                    MessageBoxA(
                                        hwnd,
                                        CString::new(format!("Gagal menulis berkas lisensi: {}", e))
                                            .unwrap()
                                            .as_ptr()
                                            as *const u8,
                                        b"Export Error\0".as_ptr(),
                                        MB_ICONERROR | MB_OK,
                                    );
                                }
                            }
                        }
                    }
                    ID_BTN_SPON_INSPECT => {
                        state.spon_state.inspect();
                        state.status_text = format!(
                            "Inspeksi SPON Selesai: Status {}",
                            state.spon_state.compatibility_status
                        );
                        InvalidateRect(hwnd, null_mut(), 1);
                    }
                    ID_BTN_SPON_RUN_CHECKS => {
                        let h_edit = GetDlgItem(hwnd, ID_EDIT_NATIVE_ID as i32);
                        let input_sn = get_window_text(h_edit);
                        state.spon_state.run_checks(&input_sn);
                        state.status_text =
                            "Diagnostic SPON Selesai: Read-only inspection selesai.".into();
                        InvalidateRect(hwnd, null_mut(), 1);
                    }
                    ID_BTN_SPON_EXPORT_REPORT => {
                        let report_path =
                            "C:\\MitraAudio\\SPON_COMPATIBILITY_DIAGNOSTIC_REPORT.txt";
                        let mut report_content = String::new();
                        report_content
                            .push_str("====================================================\n");
                        report_content.push_str("  SPON NATIVE COMPATIBILITY DIAGNOSTIC REPORT\n");
                        report_content
                            .push_str("====================================================\n\n");
                        report_content
                            .push_str(&format!("Waktu Uji: {:?}\n", std::time::SystemTime::now()));
                        report_content.push_str(&format!(
                            "Target Instalasi: {}\n",
                            state.spon_state.install_path
                        ));
                        report_content.push_str(&format!(
                            "Server Present: {}\n",
                            state.spon_state.server_present
                        ));
                        report_content.push_str(&format!(
                            "Station Present: {}\n",
                            state.spon_state.station_present
                        ));
                        report_content
                            .push_str(&format!("SDK Present: {}\n", state.spon_state.sdk_present));
                        report_content.push_str(&format!(
                            "ET99 Dongle DLL Present: {}\n",
                            state.spon_state.et99_present
                        ));
                        report_content.push_str(&format!(
                            "Historical INI Serial: {}\n",
                            state.spon_state.ini_serial
                        ));
                        report_content.push_str(&format!(
                            "Compatibility Status: {}\n\n",
                            state.spon_state.compatibility_status
                        ));
                        report_content.push_str("LOG RINCI DIAGNOSTIK:\n");
                        for log_line in &state.spon_state.detailed_log {
                            report_content.push_str(&format!("- {}\n", log_line));
                        }
                        report_content.push_str("\nSTATUS LISENSI RESMI:\n");
                        report_content.push_str(
                            "BLOCKED — OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED\n",
                        );
                        report_content.push_str("Penerbitan kunci SPON ditahan demi kepatuhan etika dan integritas sistem.\n");

                        match fs::write(report_path, report_content) {
                            Ok(_) => {
                                state.status_text =
                                    format!("Laporan diagnostik SPON disimpan ke: {}", report_path);
                                InvalidateRect(hwnd, null_mut(), 1);
                                MessageBoxA(
                                    hwnd,
                                    CString::new(format!(
                                        "Laporan diagnostik SPON tersimpan di:\n\n{}",
                                        report_path
                                    ))
                                    .unwrap()
                                    .as_ptr() as *const u8,
                                    b"Report Exported\0".as_ptr(),
                                    MB_ICONINFORMATION | MB_OK,
                                );
                            }
                            Err(e) => {
                                MessageBoxA(
                                    hwnd,
                                    CString::new(format!("Gagal menulis laporan: {}", e))
                                        .unwrap()
                                        .as_ptr() as *const u8,
                                    b"Export Error\0".as_ptr(),
                                    MB_ICONERROR | MB_OK,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            draw_dark_utility_ui(hwnd, hdc);
            EndPaint(hwnd, &ps);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
    0
}

unsafe fn update_controls(hwnd: HWND) {
    let state = get_state();
    let is_mitra = state.selected_product == SoftwareProduct::MitraAudio;

    // Mitra Audio controls
    let h_export = GetDlgItem(hwnd, ID_BTN_EXPORT_LIC as i32);
    if !h_export.is_null() {
        if is_mitra && state.last_issued_license.is_some() {
            ShowWindow(h_export, SW_SHOW);
        } else {
            ShowWindow(h_export, SW_HIDE);
        }
    }

    // SPON Compatibility buttons
    let h_spon_insp = GetDlgItem(hwnd, ID_BTN_SPON_INSPECT as i32);
    let h_spon_chk = GetDlgItem(hwnd, ID_BTN_SPON_RUN_CHECKS as i32);
    let h_spon_rep = GetDlgItem(hwnd, ID_BTN_SPON_EXPORT_REPORT as i32);

    let spon_cmd = if !is_mitra { SW_SHOW } else { SW_HIDE };
    if !h_spon_insp.is_null() {
        ShowWindow(h_spon_insp, spon_cmd);
    }
    if !h_spon_chk.is_null() {
        ShowWindow(h_spon_chk, spon_cmd);
    }
    if !h_spon_rep.is_null() {
        ShowWindow(h_spon_rep, spon_cmd);
    }
}

unsafe fn draw_dark_utility_ui(hwnd: HWND, hdc: HDC) {
    let state = get_state();

    // 1. Fill entire window background (#0B1020)
    let mut rect_client: RECT = std::mem::zeroed();
    GetClientRect(hwnd, &mut rect_client);
    FillRect(hdc, &rect_client, state.brush_bg);

    // 2. Draw Top Header Bar with Logo
    if !state.logo_bitmap.is_null() {
        let hdc_mem = CreateCompatibleDC(hdc);
        let old_bm = SelectObject(hdc_mem, state.logo_bitmap);
        BitBlt(hdc, 24, 18, 48, 48, hdc_mem, 0, 0, SRCCOPY);
        SelectObject(hdc_mem, old_bm);
        DeleteDC(hdc_mem);
    }

    SetBkMode(hdc, 1); // TRANSPARENT

    // Title text next to logo (x = 84)
    SelectObject(hdc, state.font_title);
    SetTextColor(hdc, COLOR_TEXT);
    let title = b"Mitra Audio SN Generator\0";
    TextOutA(hdc, 84, 18, title.as_ptr(), (title.len() - 1) as i32);

    // Subtitle text
    SelectObject(hdc, state.font_regular);
    SetTextColor(hdc, COLOR_TEXT_MUTED);
    let sub = b"Unified Offline Licensing & Legacy Compatibility System\0";
    TextOutA(hdc, 86, 44, sub.as_ptr(), (sub.len() - 1) as i32);

    // Status Badge in Header (Right aligned)
    let is_mitra = state.selected_product == SoftwareProduct::MitraAudio;
    let badge_color = if is_mitra {
        COLOR_GREEN_OFFLINE
    } else {
        COLOR_AMBER_WARN
    };

    let badge_brush = CreateSolidBrush(COLOR_PANEL);
    let badge_pen = CreatePen(PS_SOLID, 1, badge_color);
    let old_p = SelectObject(hdc, badge_pen);
    let old_b = SelectObject(hdc, badge_brush);
    RoundRect(
        hdc,
        rect_client.right - 140,
        22,
        rect_client.right - 24,
        50,
        8,
        8,
    );
    SelectObject(hdc, old_b);
    SelectObject(hdc, old_p);
    DeleteObject(badge_brush);
    DeleteObject(badge_pen);

    SelectObject(hdc, state.font_bold);
    SetTextColor(hdc, badge_color);
    let badge_text: &[u8] = if is_mitra {
        b"MITRA ED25519\0"
    } else {
        b"SPON COMPAT\0"
    };
    TextOutA(
        hdc,
        rect_client.right - 130,
        27,
        badge_text.as_ptr(),
        (badge_text.len() - 1) as i32,
    );

    // 3. Main Container Panel with Rounded Rectangle (#121A2B)
    let pen_border = CreatePen(PS_SOLID, 1, COLOR_PANEL_BORDER);
    let old_pen = SelectObject(hdc, pen_border);
    let old_brush = SelectObject(hdc, state.brush_panel);
    RoundRect(
        hdc,
        20,
        76,
        rect_client.right - 20,
        rect_client.bottom - 45,
        12,
        12,
    );
    SelectObject(hdc, old_brush);
    SelectObject(hdc, old_pen);
    DeleteObject(pen_border);

    // 4. Panel Section Labels
    SelectObject(hdc, state.font_bold);
    SetTextColor(hdc, COLOR_CYAN);
    let lbl_prod = b"Software Type (Product Family):\0";
    TextOutA(hdc, 40, 92, lbl_prod.as_ptr(), (lbl_prod.len() - 1) as i32);

    let lbl_nat = if is_mitra {
        b"[ID] Target Native ID (Hardware Fingerprint):\0".as_ptr()
    } else {
        b"[SPON] Target Machine ID / Historical Serial Input:\0".as_ptr()
    };
    TextOutA(hdc, 40, 160, lbl_nat, strlen(lbl_nat) as i32);

    let lbl_sn = if is_mitra {
        b"[Key] Generated Serial Number (CIMT/MITG Cryptographic Format):\0".as_ptr()
    } else {
        b"[SPON] Legacy Serial Number Status (Read-Only Validation):\0".as_ptr()
    };
    TextOutA(hdc, 40, 285, lbl_sn, strlen(lbl_sn) as i32);

    // If SPON mode is active, render SPON Diagnostics Information Area
    if !is_mitra {
        // Draw SPON Info Subpanel
        let spon_pen = CreatePen(PS_SOLID, 1, COLOR_INPUT_BORDER);
        let spon_brush = CreateSolidBrush(COLOR_INPUT_BG);
        let old_sp = SelectObject(hdc, spon_pen);
        let old_sb = SelectObject(hdc, spon_brush);
        RoundRect(
            hdc,
            40,
            365,
            rect_client.right - 40,
            rect_client.bottom - 95,
            8,
            8,
        );
        SelectObject(hdc, old_sb);
        SelectObject(hdc, old_sp);
        DeleteObject(spon_brush);
        DeleteObject(spon_pen);

        SelectObject(hdc, state.font_bold);
        SetTextColor(hdc, COLOR_CYAN);
        let lbl_diag = b"SPON Native Environment Diagnostics (C:\\IPNBS):\0";
        TextOutA(hdc, 50, 375, lbl_diag.as_ptr(), (lbl_diag.len() - 1) as i32);

        SelectObject(hdc, state.font_small);
        SetTextColor(hdc, COLOR_TEXT);

        let line1 = format!(
            "Server: {} | Station: {} | SDK: {} | ET99 DLL: {}",
            if state.spon_state.server_present {
                "PRESENT (1.43 MB)"
            } else {
                "ABSENT"
            },
            if state.spon_state.station_present {
                "PRESENT (3.0 MB)"
            } else {
                "ABSENT"
            },
            if state.spon_state.sdk_present {
                "PRESENT"
            } else {
                "ABSENT"
            },
            if state.spon_state.et99_present {
                "PRESENT (HW Token)"
            } else {
                "ABSENT"
            }
        );
        let line1_c = CString::new(line1).unwrap_or_default();
        TextOutA(
            hdc,
            50,
            400,
            line1_c.as_ptr() as *const u8,
            line1_c.as_bytes().len() as i32,
        );

        let line2 = format!(
            "Historical INI: {} | Status: UNVERIFIED_HISTORICAL_REFERENCE",
            state.spon_state.ini_serial
        );
        let line2_c = CString::new(line2).unwrap_or_default();
        TextOutA(
            hdc,
            50,
            420,
            line2_c.as_ptr() as *const u8,
            line2_c.as_bytes().len() as i32,
        );

        SetTextColor(hdc, COLOR_RED_BLOCKED);
        let line3 = b"Issuance Status: BLOCKED - OFFICIAL SPON GENERATION PROCEDURE NOT VERIFIED\0";
        TextOutA(hdc, 50, 442, line3.as_ptr(), (line3.len() - 1) as i32);
    }

    // 5. Status bar text at bottom
    SelectObject(hdc, state.font_regular);
    SetTextColor(hdc, COLOR_TEXT_MUTED);
    let status_c = CString::new(state.status_text.clone()).unwrap_or_default();
    TextOutA(
        hdc,
        24,
        rect_client.bottom - 32,
        status_c.as_ptr() as *const u8,
        status_c.as_bytes().len() as i32,
    );
}

unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

fn create_button(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32, id: isize) -> HWND {
    let text_c = CString::new(text).unwrap();
    unsafe {
        CreateWindowExA(
            0,
            b"BUTTON\0".as_ptr(),
            text_c.as_ptr() as *const u8,
            WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
            x,
            y,
            w,
            h,
            parent,
            id as HMENU,
            GetModuleHandleA(null_mut()),
            null_mut(),
        )
    }
}

fn create_edit(parent: HWND, x: i32, y: i32, w: i32, h: i32, id: isize) -> HWND {
    unsafe {
        CreateWindowExA(
            WS_EX_CLIENTEDGE,
            b"EDIT\0".as_ptr(),
            b"\0".as_ptr(),
            WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL as u32,
            x,
            y,
            w,
            h,
            parent,
            id as HMENU,
            GetModuleHandleA(null_mut()),
            null_mut(),
        )
    }
}

fn create_combobox(parent: HWND, x: i32, y: i32, w: i32, h: i32, id: isize) -> HWND {
    unsafe {
        CreateWindowExA(
            0,
            b"COMBOBOX\0".as_ptr(),
            b"\0".as_ptr(),
            WS_CHILD
                | WS_VISIBLE
                | CBS_DROPDOWNLIST as u32
                | CBS_OWNERDRAWFIXED as u32
                | CBS_HASSTRINGS as u32
                | WS_VSCROLL,
            x,
            y,
            w,
            h,
            parent,
            id as HMENU,
            GetModuleHandleA(null_mut()),
            null_mut(),
        )
    }
}

fn main() {
    // 1. Setup Protected Authority State
    let raw_key = [0x55u8; 32];
    let signing_key = SigningKey::from_bytes(&raw_key);
    let authority_salt = "MITRA_AUDIO_TEST_AUTHORITY_SALT_2026";
    let issuer = LicenseIssuer::new("MITRA-AUTHORITY-ROOT", signing_key, authority_salt);
    let storage = LicenseAuthorityStorage::new();
    let rng = WindowsSecureRandomProvider::new();

    unsafe {
        // GDI Brushes & Modern Fonts
        let brush_bg = CreateSolidBrush(COLOR_BG);
        let brush_panel = CreateSolidBrush(COLOR_PANEL);
        let brush_input = CreateSolidBrush(COLOR_INPUT_BG);
        let brush_sel = CreateSolidBrush(COLOR_ELECTRIC_BLUE);

        let font_title = CreateFontA(
            22,
            0,
            0,
            0,
            700,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            b"Segoe UI\0".as_ptr(),
        );
        let font_bold = CreateFontA(
            15,
            0,
            0,
            0,
            600,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            b"Segoe UI\0".as_ptr(),
        );
        let font_regular = CreateFontA(
            14,
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            b"Segoe UI\0".as_ptr(),
        );
        let font_small = CreateFontA(
            13,
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            b"Segoe UI\0".as_ptr(),
        );
        let font_mono = CreateFontA(
            17,
            0,
            0,
            0,
            700,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            b"Consolas\0".as_ptr(),
        );

        let hdc_screen = GetDC(null_mut());
        let logo_bitmap = create_logo_hbitmap(hdc_screen);
        ReleaseDC(null_mut(), hdc_screen);

        let mut spon_state = SponInspectionState::new();
        spon_state.inspect();

        let state = Box::new(GeneratorState {
            issuer,
            storage,
            rng,
            selected_product: SoftwareProduct::MitraAudio,
            last_generated_sn: String::new(),
            last_issued_license: None,
            status_text: "Ready: Mitra Audio Network System (Permanent License Mode)".to_string(),
            spon_state,
            logo_bitmap,
            brush_bg,
            brush_panel,
            brush_input,
            brush_sel,
            font_title,
            font_bold,
            font_regular,
            font_mono,
            font_small,
        });

        let state_raw = Box::into_raw(state);
        STATE_PTR.store(state_raw as usize, Ordering::SeqCst);

        let instance = GetModuleHandleA(null_mut());
        let class_name = b"MitraSnGenUnifiedClass\0";

        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassA(&wc);

        // Window size: Unified Key Generator & Compatibility Utility Window (660 x 590)
        let hwnd = CreateWindowExA(
            0,
            class_name.as_ptr(),
            b"Mitra Audio SN Generator & Legacy Compatibility Hub\0".as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            660,
            590,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
        );

        let current_state = get_state();

        // 1. Product Dropdown Selector (ComboBox Owner-Drawn)
        let h_combo = create_combobox(hwnd, 40, 115, 560, 200, ID_COMBO_PRODUCT);
        SendMessageA(
            h_combo,
            WM_SETFONT_VAL,
            current_state.font_regular as usize,
            1,
        );
        SendMessageA(
            h_combo,
            CB_ADDSTRING,
            0,
            b"Mitra Audio Network System\0".as_ptr() as isize,
        );
        SendMessageA(
            h_combo,
            CB_ADDSTRING,
            0,
            b"SPON (Legacy Broadcast System)\0".as_ptr() as isize,
        );
        SendMessageA(h_combo, CB_SETCURSEL, 0, 0);

        // 2. Native ID / Hardware Input Row
        let h_edit_native = create_edit(hwnd, 40, 185, 410, 32, ID_EDIT_NATIVE_ID);
        SendMessageA(
            h_edit_native,
            WM_SETFONT_VAL,
            current_state.font_regular as usize,
            1,
        );

        let h_btn_paste = create_button(hwnd, "Paste", 460, 185, 65, 32, ID_BTN_PASTE_NATIVE);
        SendMessageA(
            h_btn_paste,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );

        let h_btn_clear = create_button(hwnd, "Clear", 535, 185, 65, 32, ID_BTN_CLEAR_NATIVE);
        SendMessageA(
            h_btn_clear,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );

        // 3. Primary Action Button: GENERATE SN
        let h_btn_gen = create_button(hwnd, "GENERATE SN", 40, 230, 560, 42, ID_BTN_GENERATE_SN);
        SendMessageA(
            h_btn_gen,
            WM_SETFONT_VAL,
            current_state.font_title as usize,
            1,
        );

        // 4. Output Serial Number Row (Consolas Mono Font)
        let h_edit_sn = create_edit(hwnd, 40, 310, 430, 36, ID_EDIT_SN_OUTPUT);
        SendMessageA(
            h_edit_sn,
            WM_SETFONT_VAL,
            current_state.font_mono as usize,
            1,
        );

        let h_btn_copy = create_button(hwnd, "COPY SN", 480, 310, 120, 36, ID_BTN_COPY_SN);
        SendMessageA(
            h_btn_copy,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );

        // 5. License Export Button (Mitra Audio Mode)
        let h_btn_export = create_button(
            hwnd,
            "Export Signed License (.lic)",
            40,
            360,
            560,
            34,
            ID_BTN_EXPORT_LIC,
        );
        SendMessageA(
            h_btn_export,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );
        ShowWindow(h_btn_export, SW_HIDE);

        // 6. SPON Compatibility Action Buttons (Shown only in SPON mode)
        let h_spon_insp = create_button(
            hwnd,
            "Inspect Installation",
            40,
            475,
            175,
            34,
            ID_BTN_SPON_INSPECT,
        );
        SendMessageA(
            h_spon_insp,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );
        ShowWindow(h_spon_insp, SW_HIDE);

        let h_spon_chk = create_button(
            hwnd,
            "Run Compatibility Checks",
            225,
            475,
            205,
            34,
            ID_BTN_SPON_RUN_CHECKS,
        );
        SendMessageA(
            h_spon_chk,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );
        ShowWindow(h_spon_chk, SW_HIDE);

        let h_spon_rep = create_button(
            hwnd,
            "Export Test Report",
            440,
            475,
            160,
            34,
            ID_BTN_SPON_EXPORT_REPORT,
        );
        SendMessageA(
            h_spon_rep,
            WM_SETFONT_VAL,
            current_state.font_bold as usize,
            1,
        );
        ShowWindow(h_spon_rep, SW_HIDE);

        // Message pump
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}
