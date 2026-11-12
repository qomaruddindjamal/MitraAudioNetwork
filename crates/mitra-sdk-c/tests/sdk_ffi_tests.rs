use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicI32, Ordering};
use IPNBSSDK::*;

static CALLBACK_INVOCATION_COUNTER: AtomicI32 = AtomicI32::new(0);

extern "C" fn test_callback_handler(
    event_type: std::ffi::c_int,
    _data: *mut std::ffi::c_void,
    _len: std::ffi::c_int,
) {
    CALLBACK_INVOCATION_COUNTER.fetch_add(event_type, Ordering::SeqCst);
}

#[test]
fn test_sdk_ffi_session_lifecycle() {
    unsafe {
        let ip_cstr = CString::new("127.0.0.1").unwrap();
        IPNBSSDK_SetServerIP(ip_cstr.as_ptr());
        IPNBSSDK_SetServerPort(2046);
        IPNBSSDK_SetStatePort(2048);
        IPNBSSDK_SetIsUnicode(0);
        IPNBSSDK_SetParentWnd(std::ptr::null_mut());

        IPNBSSDK_SetStatusCallBack(Some(test_callback_handler));

        // Connect
        let connect_res = IPNBSSDK_TCPConnect(ip_cstr.as_ptr(), std::ptr::null_mut());
        assert_eq!(connect_res, 1);
        assert_eq!(CALLBACK_INVOCATION_COUNTER.load(Ordering::SeqCst), 1); // event 1

        // Close
        let close_res = IPNBSSDK_TCPClose();
        assert_eq!(close_res, 1);
    }
}

#[test]
fn test_sdk_ffi_control_commands() {
    unsafe {
        let terms = [101, 102, 103];
        let broadcast_res =
            IPNBSSDK_CtrlBroadcast(1, terms.as_ptr(), terms.len() as std::ffi::c_int, 1);
        assert_eq!(broadcast_res, 1);

        let single_res = IPNBSSDK_CtrlBroadcastSingle(101, 1);
        assert_eq!(single_res, 1);

        let call_res = IPNBSSDK_CtrlCall(101, 102);
        assert_eq!(call_res, 1);

        let answer_res = IPNBSSDK_CtrlAnswer(102);
        assert_eq!(answer_res, 1);

        let hang_res = IPNBSSDK_CtrlHang(101);
        assert_eq!(hang_res, 1);

        let alarm_res = IPNBSSDK_CtrlFireAlarm(1, 1);
        assert_eq!(alarm_res, 1);
    }
}

#[test]
fn test_sdk_ffi_queries() {
    unsafe {
        let ip_query = CString::new("192.168.1.100").unwrap();
        let term_id = IPNBSSDK_CtrlQueryID(ip_query.as_ptr());
        assert_eq!(term_id, 1001);

        let mut out_ip_buf = [0u8; 32];
        let query_ip_res =
            IPNBSSDK_CtrlQueryIP(1001, out_ip_buf.as_mut_ptr() as *mut std::ffi::c_char);
        assert_eq!(query_ip_res, 1);
        let out_ip_str = CStr::from_ptr(out_ip_buf.as_ptr() as *const std::ffi::c_char)
            .to_str()
            .unwrap();
        assert_eq!(out_ip_str, "192.168.1.100");

        let port = IPNBSSDK_CtrlQueryPort(1001);
        assert_eq!(port, 2046);

        let state = IPNBSSDK_CtrlQueryState(1001);
        assert_eq!(state, 1);

        let term_count = IPNBSSDK_CtrlQueryTermCount();
        assert!(term_count >= 1);
    }
}
