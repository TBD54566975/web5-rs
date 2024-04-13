use keys::{
    key::Curve,
    key_manager::{local_key_manager::LocalKeyManager, KeyManager},
};
use std::ffi::{c_char, CString};

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
    Box::into_raw(Box::new(manager))
}

#[no_mangle]
pub extern "C" fn local_key_manager_free(manager: *mut LocalKeyManager) {
    if !manager.is_null() {
        unsafe {
            let _ = Box::from_raw(manager);
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
