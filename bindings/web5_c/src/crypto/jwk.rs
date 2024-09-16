use crate::c::{opt_cstr_to_string, opt_string_to_cstr};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use web5::crypto::jwk::Jwk;

#[repr(C)]
pub struct CJwk {
    pub alg: *const c_char,
    pub kty: *const c_char,
    pub crv: *const c_char,
    pub d: *const c_char,
    pub x: *const c_char,
    pub y: *const c_char,
}

impl From<&CJwk> for Jwk {
    fn from(jwk_c: &CJwk) -> Self {
        Jwk {
            alg: unsafe { opt_cstr_to_string(jwk_c.alg) },
            kty: unsafe { CStr::from_ptr(jwk_c.kty).to_string_lossy().into_owned() },
            crv: unsafe { CStr::from_ptr(jwk_c.crv).to_string_lossy().into_owned() },
            d: unsafe { opt_cstr_to_string(jwk_c.d) },
            x: unsafe { CStr::from_ptr(jwk_c.x).to_string_lossy().into_owned() },
            y: unsafe { opt_cstr_to_string(jwk_c.y) },
        }
    }
}

impl From<Jwk> for CJwk {
    fn from(jwk: Jwk) -> Self {
        CJwk {
            alg: opt_string_to_cstr(jwk.alg),
            kty: CString::new(jwk.kty).unwrap().into_raw(),
            crv: CString::new(jwk.crv).unwrap().into_raw(),
            d: opt_string_to_cstr(jwk.d),
            x: CString::new(jwk.x).unwrap().into_raw(),
            y: opt_string_to_cstr(jwk.y),
        }
    }
}

#[no_mangle]
pub extern "C" fn jwk_compute_thumbprint(jwk_ptr: *const CJwk) -> *mut c_char {
    if jwk_ptr.is_null() {
        return ptr::null_mut();
    }

    let jwk = unsafe { Jwk::from(&*jwk_ptr) };

    match jwk.compute_thumbprint() {
        Ok(thumbprint) => CString::new(thumbprint).unwrap().into_raw(),
        Err(_) => ptr::null_mut(),
    }
}
