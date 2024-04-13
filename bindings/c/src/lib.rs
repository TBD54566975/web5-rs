use dids::{
    bearer::BearerDid,
    method::{
        jwk::{DidJwk, DidJwkCreateOptions},
        Method,
    },
};
use keys::{
    key::Curve,
    key_manager::{local_key_manager::LocalKeyManager, KeyManager},
};
use std::{
    ffi::{c_char, CString},
    sync::Arc,
};

#[repr(C)]
pub struct FFICurve {
    value: i32,
}

impl FFICurve {
    fn from_curve(curve: Curve) -> Self {
        FFICurve {
            value: match curve {
                Curve::Secp256k1 => 0,
                Curve::Ed25519 => 1,
            },
        }
    }

    fn to_curve(self) -> Option<Curve> {
        match self.value {
            0 => Some(Curve::Secp256k1),
            1 => Some(Curve::Ed25519),
            _ => None,
        }
    }
}

#[no_mangle]
pub extern "C" fn local_key_manager_new() -> *mut LocalKeyManager {
    let manager = LocalKeyManager::new_in_memory();
    let arc = Arc::new(manager);
    Arc::into_raw(arc.clone()) as *mut _
}

#[no_mangle]
pub extern "C" fn local_key_manager_free(manager_ptr: *mut LocalKeyManager) {
    if !manager_ptr.is_null() {
        unsafe {
            // Convert the raw pointer back to an Arc to decrease the ref count
            let _manager = Arc::from_raw(manager_ptr);
            // Arc gets dropped here, and the memory will be freed if ref count reaches 0
        }
    }
}

#[no_mangle]
pub extern "C" fn local_key_manager_generate_private_key(
    manager: *mut LocalKeyManager,
    ffi_curve: FFICurve,
    out_key_alias: *mut *const c_char,
) -> bool {
    let curve = match ffi_curve.to_curve() {
        Some(c) => c,
        None => return false,
    };

    let manager = unsafe { &mut *manager };
    match manager.generate_private_key(curve) {
        Ok(key_alias) => {
            unsafe {
                *out_key_alias = CString::new(key_alias).unwrap().into_raw();
            }
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn create_did_jwk(
    key_manager_ptr: *mut LocalKeyManager,
    curve_type: i32,
) -> *mut BearerDid {
    if key_manager_ptr.is_null() {
        return std::ptr::null_mut();
    }

    // Safely convert the raw pointer back to an Arc without decrementing the ref count
    let key_manager = unsafe { Arc::from_raw(key_manager_ptr) };
    // Clone the Arc to keep using it within this scope
    let key_manager_clone = key_manager.clone();
    // Forget the original Arc to prevent its drop when going out of scope
    std::mem::forget(key_manager);

    let curve = match curve_type {
        0 => Curve::Secp256k1,
        1 => Curve::Ed25519,
        _ => return std::ptr::null_mut(),
    };
    let options = DidJwkCreateOptions { curve };

    match DidJwk::create(key_manager_clone, options) {
        Ok(bearer_did) => {
            // Instead of converting BearerDid into a Box, wrap it with Arc and convert to raw pointer
            Arc::into_raw(Arc::new(bearer_did)) as *mut _
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_bearer_did(bearer_did_ptr: *mut BearerDid) {
    if !bearer_did_ptr.is_null() {
        unsafe {
            // Convert the raw pointer back to an Arc, which will deallocate the BearerDid
            // once the last reference goes out of scope
            let _bearer_did = Arc::from_raw(bearer_did_ptr);
            // Arc is dropped here, and the BearerDid will be deallocated if this was the last reference
        }
    }
}
