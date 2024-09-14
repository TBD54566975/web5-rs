pub mod credentials;
pub mod crypto;
pub mod dids;

mod datetime;
pub mod errors;
mod jose;
pub mod json;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod test_vectors;

use crate::crypto::jwk::Jwk;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn jwk_compute_thumbprint_from_json(jwk_json: *const c_char) -> *mut c_char {
    // Convert C string to Rust string
    let jwk_json = unsafe {
        assert!(!jwk_json.is_null());
        CStr::from_ptr(jwk_json).to_str().unwrap_or("")
    };

    // Deserialize JSON string into Jwk struct
    let jwk: Jwk = match serde_json::from_str(jwk_json) {
        Ok(jwk) => jwk,
        Err(_) => {
            let error_message = CString::new("Invalid JSON").unwrap();
            return error_message.into_raw();
        }
    };

    // Compute the thumbprint
    match jwk.compute_thumbprint() {
        Ok(thumbprint) => {
            let c_str_thumbprint = CString::new(thumbprint).unwrap();
            c_str_thumbprint.into_raw() // Return pointer to C string
        }
        Err(err) => {
            let error_message = CString::new(err.to_string()).unwrap();
            error_message.into_raw() // Return error message
        }
    }
}

#[no_mangle]
pub extern "C" fn jwk_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s); // Free the memory allocated by `CString`
        }
    }
}

#[no_mangle]
pub extern "C" fn bridge_in_rust(func: extern "C" fn() -> i32) -> i32 {
    func() // Call the Go function passed in
}
