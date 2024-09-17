use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

pub fn free_bytes(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

pub unsafe fn opt_cstr_to_string(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        Some(CStr::from_ptr(c_str).to_string_lossy().into_owned())
    }
}

pub fn opt_string_to_cstr(opt: Option<String>) -> *const c_char {
    match opt {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}
