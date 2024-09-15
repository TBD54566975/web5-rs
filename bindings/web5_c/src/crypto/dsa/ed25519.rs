use crate::crypto::jwk::CJwk;
use std::ptr;
use std::sync::Arc;
use web5::crypto::dsa::{ed25519::Ed25519Signer, Signer};
use web5::crypto::jwk::Jwk;

#[repr(C)]
pub struct CEd25519Signer {
    ed25519_signer: Arc<Ed25519Signer>,
}

#[no_mangle]
pub extern "C" fn ed25519_signer_new(cjwk_ptr: *const CJwk) -> *mut CEd25519Signer {
    if cjwk_ptr.is_null() {
        return ptr::null_mut();
    }

    let jwk = unsafe { Jwk::from(&*cjwk_ptr) };
    let signer = Ed25519Signer::new(jwk);

    Box::into_raw(Box::new(CEd25519Signer {
        ed25519_signer: Arc::new(signer),
    }))
}

#[no_mangle]
pub extern "C" fn ed25519_signer_sign(
    signer_ptr: *mut CEd25519Signer,
    payload: *const u8,
    payload_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    if signer_ptr.is_null() || payload.is_null() {
        return ptr::null_mut();
    }

    let ed25519_signer = unsafe { &*(*signer_ptr).ed25519_signer };
    let payload_slice = unsafe { std::slice::from_raw_parts(payload, payload_len) };

    match ed25519_signer.sign(payload_slice) {
        Ok(signature) => {
            let sig_len = signature.len();
            let boxed_signature = signature.into_boxed_slice();
            let sig_ptr = Box::into_raw(boxed_signature) as *mut u8;
            unsafe { *out_len = sig_len };
            sig_ptr
        }
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn ed25519_signer_free(signer_ptr: *mut CEd25519Signer) {
    if !signer_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(signer_ptr);
        }
    }
}
