use crate::crypto::{
    dsa::{add_signer_to_registry, rust_signer_sign, CSigner},
    jwk::CJwk,
};
use web5::crypto::{
    jwk::Jwk,
    key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
};

#[repr(C)]
pub struct CInMemoryKeyManager {
    manager: InMemoryKeyManager,
}

#[no_mangle]
pub extern "C" fn in_memory_key_manager_new() -> *mut CInMemoryKeyManager {
    Box::into_raw(Box::new(CInMemoryKeyManager {
        manager: InMemoryKeyManager::new(),
    }))
}

#[no_mangle]
pub extern "C" fn in_memory_key_manager_import_private_jwk(
    c_manager: *mut CInMemoryKeyManager,
    private_jwk: *const CJwk,
) -> *mut CJwk {
    if c_manager.is_null() || private_jwk.is_null() {
        return std::ptr::null_mut();
    }
    let c_manager = unsafe { &*c_manager };
    let private_jwk = unsafe { Jwk::from(&*private_jwk) };
    match c_manager.manager.import_private_jwk(private_jwk) {
        Ok(public_jwk) => Box::into_raw(Box::new(CJwk::from(public_jwk))),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn in_memory_key_manager_get_signer(
    c_manager: *mut CInMemoryKeyManager,
    public_jwk: *const CJwk,
) -> *mut CSigner {
    if c_manager.is_null() || public_jwk.is_null() {
        return std::ptr::null_mut();
    }
    let c_manager = unsafe { &*c_manager };
    let public_jwk = unsafe { Jwk::from(&*public_jwk) };
    match c_manager.manager.get_signer(public_jwk) {
        Ok(signer) => {
            let signer_id = add_signer_to_registry(signer);

            Box::into_raw(Box::new(CSigner {
                signer_id,
                sign: rust_signer_sign,
            }))
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn in_memory_key_manager_free(manager: *mut CInMemoryKeyManager) {
    if !manager.is_null() {
        unsafe {
            let _ = Box::from_raw(manager);
        }
    }
}
