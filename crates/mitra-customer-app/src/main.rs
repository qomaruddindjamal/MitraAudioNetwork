//! Mitra Audio Network System Customer-facing Application GUI PoC
//!
//! Includes:
//! 1. Display Hardware Native ID with One-click Copy
//! 2. Serial Number input field
//! 3. License file (.lic) import support
//! 4. Permanent Registration Engine with Ed25519 Cryptographic Verification
//! 5. Feature Entitlements Display (e.g. BASIC_AUDIO, WEB_MANAGEMENT)
//! 6. Registration Persistence to local file (`mitra_customer_registration.dat`)
//!
//! Note: This application contains ONLY the public verification key.
//! It does NOT contain the SN generator, private signing key, or vendor issuance records.

#![windows_subsystem = "windows"]
#![allow(clippy::manual_c_str_literals)]

use mitra_adapter_windows::hw_windows::WindowsHardwareIdProvider;
use mitra_core::license::{LicenseVerifier, SignedMitraLicense, ENTITLEMENT_WEB_MANAGEMENT};
use std::ffi::CString;
use std::fs;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::DataExchange::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const CF_TEXT_VAL: u32 = 1;
const SS_LEFT_VAL: u32 = 0;

// Hardcoded Vendor Public Verifying Key (32 bytes) - Derived from vendor authority key pair
const VENDOR_PUBLIC_KEY_BYTES: [u8; 32] = [
    0xac, 0x1b, 0x86, 0x48, 0xca, 0x6e, 0x8a, 0x1c, 0x35, 0x38, 0x2f, 0x3d, 0xbf, 0x3b, 0x47, 0x77,
    0xb6, 0x98, 0x4f, 0xec, 0x2f, 0x45, 0x01, 0x18, 0x7e, 0x86, 0x84, 0x06, 0x36, 0x8b, 0xc1, 0x93,
];

const REGISTRATION_FILE_PATH: &str = "C:\\MitraAudio\\mitra_customer_registration.dat";

const ID_BTN_COPY_NATIVE: isize = 101;
const ID_BTN_REGISTER: isize = 102;
const ID_BTN_LOAD_LIC: isize = 103;
const ID_BTN_PASTE_SN: isize = 104;

const ID_EDIT_NATIVE_ID: isize = 201;
const ID_EDIT_SN: isize = 202;
const ID_EDIT_LIC_PATH: isize = 203;
const ID_EDIT_REG_STATUS: isize = 204;
const ID_EDIT_FEATURE_STATUS: isize = 205;

struct CustomerAppState {
    native_id: String,
    registration_status: String,
    registered_license: Option<SignedMitraLicense>,
    features_active: Vec<String>,
}

static CUST_STATE_PTR: AtomicUsize = AtomicUsize::new(0);

fn get_cust_state() -> &'static mut CustomerAppState {
    let ptr = CUST_STATE_PTR.load(Ordering::SeqCst) as *mut CustomerAppState;
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

fn load_persisted_registration(
    actual_native_id: &str,
) -> (String, Option<SignedMitraLicense>, Vec<String>) {
    if let Ok(content) = fs::read_to_string(REGISTRATION_FILE_PATH) {
        if let Ok(lic) = SignedMitraLicense::from_license_file_content(&content) {
            let verifier =
                LicenseVerifier::from_public_key_bytes(&VENDOR_PUBLIC_KEY_BYTES).unwrap();
            let revoked = [];
            match verifier.verify_mitra_license(
                &lic,
                "MITRA-AUDIO-SERVER",
                actual_native_id,
                &revoked,
            ) {
                Ok(()) => {
                    let mut feats = lic.payload.entitlements.clone();
                    feats.sort();
                    let status = format!(
                        "STATUS: TERLISENSI PERMANEN (AKTIF)\r\n\
                         Product ID   : {}\r\n\
                         Native ID    : {}\r\n\
                         Serial Number: {}\r\n\
                         License Type : Permanent (Seumur Hidup)\r\n\
                         Entitlements : {:?}\r\n\
                         Tanda Tangan : VALID (Ed25519 Offline Verified)",
                        lic.payload.product_id,
                        lic.payload.native_id,
                        lic.payload.serial_number,
                        feats
                    );
                    return (status, Some(lic), feats);
                }
                Err(e) => {
                    return (
                        format!(
                            "STATUS: LISENSI TERSIMPAN TIDAK COCOK ATAU DITOLAK ({:?})",
                            e
                        ),
                        None,
                        Vec::new(),
                    );
                }
            }
        }
    }
    ("STATUS: BELUM TERREGISTRASI (VERSI DEMO / TIDAK BERLISENSI)\r\nSilakan masukkan Serial Number dan berkas lisensi resmi dari vendor.".to_string(), None, Vec::new())
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let id = (wparam & 0xFFFF) as isize;
            let state = get_cust_state();

            match id {
                ID_BTN_COPY_NATIVE => {
                    set_clipboard_text(hwnd, &state.native_id);
                    MessageBoxA(
                        hwnd,
                        b"Native ID berhasil disalin ke Clipboard!\nKirimkan kode ini ke vendor untuk menerbitkan lisensi resmi.\0".as_ptr(),
                        b"Salin Berhasil\0".as_ptr(),
                        MB_ICONINFORMATION | MB_OK,
                    );
                }
                ID_BTN_PASTE_SN => {
                    let clip = get_clipboard_text(hwnd);
                    let h_sn = GetDlgItem(hwnd, ID_EDIT_SN as i32);
                    if !h_sn.is_null() && !clip.is_empty() {
                        set_window_text(h_sn, &clip);
                    }
                }
                ID_BTN_LOAD_LIC => {
                    let h_path = GetDlgItem(hwnd, ID_EDIT_LIC_PATH as i32);
                    let path = get_window_text(h_path);
                    let load_path = if path.is_empty() {
                        "C:\\MitraAudio\\mitra_license.lic".to_string()
                    } else {
                        path
                    };

                    match fs::read_to_string(&load_path) {
                        Ok(content) => {
                            match SignedMitraLicense::from_license_file_content(&content) {
                                Ok(lic) => {
                                    let h_sn = GetDlgItem(hwnd, ID_EDIT_SN as i32);
                                    set_window_text(h_sn, &lic.payload.serial_number);
                                    state.registered_license = Some(lic);
                                    MessageBoxA(
                                        hwnd,
                                        b"Berkas lisensi .lic berhasil dimuat. Klik 'Register' untuk menyelesaikan aktivasi.\0".as_ptr(),
                                        b"Berkas Lisensi Dimuat\0".as_ptr(),
                                        MB_ICONINFORMATION | MB_OK,
                                    );
                                }
                                Err(e) => {
                                    MessageBoxA(
                                        hwnd,
                                        CString::new(format!(
                                            "Berkas lisensi rusak atau format tidak dikenali: {:?}",
                                            e
                                        ))
                                        .unwrap()
                                        .as_ptr()
                                            as *const u8,
                                        b"Format Berkas Salah\0".as_ptr(),
                                        MB_ICONERROR | MB_OK,
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            MessageBoxA(
                                hwnd,
                                CString::new(format!(
                                    "Gagal membuka berkas lisensi {}: {}",
                                    load_path, e
                                ))
                                .unwrap()
                                .as_ptr() as *const u8,
                                b"Gagal Membaca Berkas\0".as_ptr(),
                                MB_ICONERROR | MB_OK,
                            );
                        }
                    }
                }
                ID_BTN_REGISTER => {
                    let h_sn = GetDlgItem(hwnd, ID_EDIT_SN as i32);
                    let sn_input = get_window_text(h_sn);

                    if sn_input.is_empty() {
                        MessageBoxA(
                            hwnd,
                            b"Mohon masukkan Serial Number (SN) terlebih dahulu!\0".as_ptr(),
                            b"Peringatan\0".as_ptr(),
                            MB_ICONWARNING | MB_OK,
                        );
                        return 0;
                    }

                    // A valid SN alone MUST NOT bypass signed license verification!
                    if state.registered_license.is_none() {
                        // Coba cari file lisensi otomatis berdasarkan SN
                        let auto_lic_path =
                            format!("C:\\MitraAudio\\mitra_license_{}.lic", sn_input);
                        if let Ok(c) = fs::read_to_string(&auto_lic_path) {
                            if let Ok(lic) = SignedMitraLicense::from_license_file_content(&c) {
                                state.registered_license = Some(lic);
                            }
                        }
                    }

                    if let Some(lic) = &state.registered_license {
                        if lic.payload.serial_number != sn_input {
                            MessageBoxA(
                                hwnd,
                                b"Serial Number yang dimasukkan tidak cocok dengan berkas paket lisensi yang dimuat!\0".as_ptr(),
                                b"Serial Number Tidak Cocok\0".as_ptr(),
                                MB_ICONERROR | MB_OK,
                            );
                            return 0;
                        }

                        let verifier =
                            LicenseVerifier::from_public_key_bytes(&VENDOR_PUBLIC_KEY_BYTES)
                                .unwrap();
                        let revoked = [];
                        match verifier.verify_mitra_license(
                            lic,
                            "MITRA-AUDIO-SERVER",
                            &state.native_id,
                            &revoked,
                        ) {
                            Ok(()) => {
                                // Simpan registrasi permanen ke disk
                                let file_content = lic.to_license_file_content();
                                let _ = fs::write(REGISTRATION_FILE_PATH, file_content);

                                state.features_active = lic.payload.entitlements.clone();
                                state.registration_status = format!(
                                    "STATUS: TERLISENSI PERMANEN (AKTIF)\r\n\
                                     Product ID   : {}\r\n\
                                     Native ID    : {}\r\n\
                                     Serial Number: {}\r\n\
                                     License Type : Permanent (Seumur Hidup)\r\n\
                                     Entitlements : {:?}\r\n\
                                     Tanda Tangan : VALID (Ed25519 Offline Verified)",
                                    lic.payload.product_id,
                                    lic.payload.native_id,
                                    lic.payload.serial_number,
                                    state.features_active
                                );

                                let h_status = GetDlgItem(hwnd, ID_EDIT_REG_STATUS as i32);
                                let h_feat = GetDlgItem(hwnd, ID_EDIT_FEATURE_STATUS as i32);
                                set_window_text(h_status, &state.registration_status);

                                let feat_str = format!(
                                    "Fitur Dasar Audio (BASIC_AUDIO): AKTIF\r\n\
                                     Manajemen Web (WEB_MANAGEMENT) : {}\r\n\
                                     Hak Siaran Multi-Zona         : DIAKTIFKAN\r\n\
                                     Interkom & Panggilan Dua Arah : DIAKTIFKAN",
                                    if state
                                        .features_active
                                        .contains(&ENTITLEMENT_WEB_MANAGEMENT.to_string())
                                    {
                                        "AKTIF (Enterprise Authorized)"
                                    } else {
                                        "TIDAK AKTIF"
                                    }
                                );
                                set_window_text(h_feat, &feat_str);

                                MessageBoxA(
                                    hwnd,
                                    b"Selamat! Registrasi Lisensi Permanen Berhasil Diverifikasi dan Diaktifkan!\0".as_ptr(),
                                    b"Aktivasi Sukses\0".as_ptr(),
                                    MB_ICONINFORMATION | MB_OK,
                                );
                            }
                            Err(e) => {
                                MessageBoxA(
                                    hwnd,
                                    CString::new(format!("Verifikasi Lisensi GAGAL!\n\nAlasan Penolakan: {:?}\n\nLisensi ini tidak dapat diaktifkan pada perangkat ini.", e)).unwrap().as_ptr() as *const u8,
                                    b"Registrasi Ditolak\0".as_ptr(),
                                    MB_ICONERROR | MB_OK,
                                );
                            }
                        }
                    } else {
                        MessageBoxA(
                            hwnd,
                            b"Berkas tanda tangan digital (.lic) belum dimuat!\nSerial Number saja tanpa tanda tangan kriptografi sah tidak dapat mengaktifkan lisensi.\0".as_ptr(),
                            b"Verifikasi Kriptografi Diperlukan\0".as_ptr(),
                            MB_ICONERROR | MB_OK,
                        );
                    }
                }
                _ => {}
            }
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Header Background (#131B2E - Surface Navy)
            let brush_header = CreateSolidBrush(0x2E1B13);
            let rect_header = RECT {
                left: 0,
                top: 0,
                right: 800,
                bottom: 60,
            };
            FillRect(hdc, &rect_header, brush_header);
            DeleteObject(brush_header);

            SetBkMode(hdc, 1);
            SetTextColor(hdc, 0xFFFFFF);
            let brand = b"MITRA AUDIO NETWORK SYSTEM\0";
            TextOutA(hdc, 20, 15, brand.as_ptr(), (brand.len() - 1) as i32);

            SetTextColor(hdc, 0xF8BD38);
            let sub = b"Customer Registration & Hardware License Activation Console (PoC)\0";
            TextOutA(hdc, 20, 35, sub.as_ptr(), (sub.len() - 1) as i32);

            EndPaint(hwnd, &ps);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
    0
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

fn create_edit(parent: HWND, x: i32, y: i32, w: i32, h: i32, id: isize, multiline: bool) -> HWND {
    let mut style = WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL as u32;
    if multiline {
        style = WS_CHILD
            | WS_VISIBLE
            | WS_BORDER
            | ES_MULTILINE as u32
            | ES_AUTOVSCROLL as u32
            | WS_VSCROLL;
    }
    unsafe {
        CreateWindowExA(
            WS_EX_CLIENTEDGE,
            b"EDIT\0".as_ptr(),
            b"\0".as_ptr(),
            style,
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

fn create_label(parent: HWND, text: &str, x: i32, y: i32, w: i32, h: i32) -> HWND {
    let text_c = CString::new(text).unwrap();
    unsafe {
        CreateWindowExA(
            0,
            b"STATIC\0".as_ptr(),
            text_c.as_ptr() as *const u8,
            WS_CHILD | WS_VISIBLE | SS_LEFT_VAL,
            x,
            y,
            w,
            h,
            parent,
            null_mut(),
            GetModuleHandleA(null_mut()),
            null_mut(),
        )
    }
}

fn main() {
    let hw_provider = WindowsHardwareIdProvider::new();
    let native_id = hw_provider
        .compute_native_id_hash()
        .unwrap_or_else(|_| "WIN-HWID-V1:UNKNOWN-NODE".to_string());

    let (status, loaded_lic, feats) = load_persisted_registration(&native_id);

    let state = Box::new(CustomerAppState {
        native_id: native_id.clone(),
        registration_status: status,
        registered_license: loaded_lic,
        features_active: feats,
    });

    let state_raw = Box::into_raw(state);
    CUST_STATE_PTR.store(state_raw as usize, Ordering::SeqCst);

    unsafe {
        let instance = GetModuleHandleA(null_mut());
        let class_name = b"MitraCustomerAppClass\0";

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

        let hwnd = CreateWindowExA(
            0,
            class_name.as_ptr(),
            b"Mitra Audio Network System - License Registration Console\0".as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            780,
            640,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
        );

        // Hardware Native ID Display
        create_label(
            hwnd,
            "Perangkat Native ID (Hardware Fingerprint Komputer Ini):",
            20,
            75,
            450,
            20,
        );
        let h_nat = create_edit(hwnd, 20, 100, 560, 26, ID_EDIT_NATIVE_ID, false);
        set_window_text(h_nat, &native_id);
        create_button(
            hwnd,
            "Copy Native ID",
            590,
            100,
            150,
            26,
            ID_BTN_COPY_NATIVE,
        );

        // License File (.lic) Path
        create_label(
            hwnd,
            "Berkas Lisensi Digital (.lic) dari Vendor:",
            20,
            135,
            400,
            20,
        );
        create_edit(hwnd, 20, 160, 560, 26, ID_EDIT_LIC_PATH, false);
        create_button(hwnd, "Load .lic File", 590, 160, 150, 26, ID_BTN_LOAD_LIC);

        // Serial Number Input
        create_label(hwnd, "Serial Number (SN):", 20, 195, 200, 20);
        create_edit(hwnd, 20, 220, 420, 30, ID_EDIT_SN, false);
        create_button(hwnd, "Paste SN", 450, 220, 130, 30, ID_BTN_PASTE_SN);
        create_button(hwnd, "REGISTER", 590, 220, 150, 30, ID_BTN_REGISTER);

        // Registration Status Box
        create_label(
            hwnd,
            "Status Registrasi & Integritas Kriptografi:",
            20,
            265,
            400,
            20,
        );
        let h_stat = create_edit(hwnd, 20, 290, 720, 150, ID_EDIT_REG_STATUS, true);
        let st = get_cust_state();
        set_window_text(h_stat, &st.registration_status);

        // Feature Entitlements Box
        create_label(
            hwnd,
            "Fitur Aplikasi yang Diaktifkan Berdasarkan Lisensi:",
            20,
            455,
            450,
            20,
        );
        let h_feat = create_edit(hwnd, 20, 480, 720, 100, ID_EDIT_FEATURE_STATUS, true);
        let feat_str = format!(
            "Fitur Dasar Audio (BASIC_AUDIO): AKTIF\r\n\
             Manajemen Web (WEB_MANAGEMENT) : {}\r\n\
             Hak Siaran Multi-Zona         : DIAKTIFKAN\r\n\
             Interkom & Panggilan Dua Arah : DIAKTIFKAN",
            if st
                .features_active
                .contains(&ENTITLEMENT_WEB_MANAGEMENT.to_string())
            {
                "AKTIF (Enterprise Authorized)"
            } else {
                "TIDAK AKTIF (Perlu Lisensi)"
            }
        );
        set_window_text(h_feat, &feat_str);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}
