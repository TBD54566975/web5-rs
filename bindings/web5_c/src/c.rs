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

use tokio::runtime::Runtime;
use web5::errors::Result;
use web5::errors::Web5Error;

pub fn get_rt() -> Result<Runtime> {
    let rt = Runtime::new()
        .map_err(|e| Web5Error::Unknown(format!("unable to instantiate tokio runtime {}", e)))?;
    Ok(rt)
}

use serde_json;
use std::ptr;
use web5::dids::methods::did_dht;

#[no_mangle]
pub extern "C" fn did_dht_resolve(uri: *const c_char, gateway_url: *const c_char) -> *mut c_char {
    let uri = match unsafe { CStr::from_ptr(uri).to_str() } {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let gateway_url = if gateway_url.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(gateway_url).to_str() } {
            Ok(s) => Some(s.to_string()),
            Err(_) => return ptr::null_mut(),
        }
    };

    let rt = match get_rt() {
        Ok(rt) => rt,
        Err(_) => return ptr::null_mut(),
    };

    let resolution_result = rt.block_on(did_dht::DidDht::resolve(uri, gateway_url));

    match serde_json::to_string(&resolution_result) {
        Ok(json_string) => match CString::new(json_string) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

pub fn free_bytes(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

pub fn opt_cstr_to_string(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() })
    }
}

pub fn opt_string_to_cstr(opt: Option<String>) -> *const c_char {
    match opt {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}
