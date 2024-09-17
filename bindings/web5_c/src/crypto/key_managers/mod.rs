use super::{
    dsa::{add_signer_to_registry, rust_signer_sign, CSigner},
    jwk::CJwk,
};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    ptr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
use web5::crypto::{jwk::Jwk, key_managers::KeyManager};

pub mod in_memory_key_manager;
pub mod poc;

static MANAGER_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

lazy_static! {
    static ref KEY_MANAGER_REGISTRY: Mutex<HashMap<i32, Arc<dyn KeyManager>>> =
        Mutex::new(HashMap::new());
}

pub fn add_key_manager_to_registry(manager: Arc<dyn KeyManager>) -> i32 {
    let manager_id = MANAGER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    KEY_MANAGER_REGISTRY
        .lock()
        .unwrap()
        .insert(manager_id, manager);
    manager_id
}

#[repr(C)]
pub struct CKeyManager {
    pub manager_id: i32,
    pub import_private_jwk: extern "C" fn(manager_id: i32, private_jwk: *const CJwk) -> *mut CJwk,
    pub get_signer: extern "C" fn(manager_id: i32, public_jwk: *const CJwk) -> *mut CSigner,
}

pub extern "C" fn rust_key_manager_import_private_jwk(
    manager_id: i32,
    private_jwk: *const CJwk,
) -> *mut CJwk {
    let private_jwk = unsafe { Jwk::from(&*private_jwk) };

    let registry = KEY_MANAGER_REGISTRY.lock().unwrap();
    if let Some(manager) = registry.get(&manager_id) {
        if let Ok(public_jwk) = manager.import_private_jwk(private_jwk) {
            return Box::into_raw(Box::new(CJwk::from(public_jwk)));
        }
    }
    ptr::null_mut()
}

pub extern "C" fn rust_key_manager_get_signer(
    manager_id: i32,
    public_jwk: *const CJwk,
) -> *mut CSigner {
    let public_jwk = unsafe { Jwk::from(&*public_jwk) };

    let registry = KEY_MANAGER_REGISTRY.lock().unwrap();
    if let Some(manager) = registry.get(&manager_id) {
        if let Ok(signer) = manager.get_signer(public_jwk) {
            let signer_id = add_signer_to_registry(signer);
            return Box::into_raw(Box::new(CSigner {
                signer_id,
                sign: rust_signer_sign,
            }));
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn call_import_private_jwk(
    manager: *const CKeyManager,
    private_jwk: *const CJwk,
) -> *mut CJwk {
    unsafe { ((*manager).import_private_jwk)((*manager).manager_id, private_jwk) }
}

#[no_mangle]
pub extern "C" fn call_get_signer(
    manager: *const CKeyManager,
    public_jwk: *const CJwk,
) -> *mut CSigner {
    unsafe { ((*manager).get_signer)((*manager).manager_id, public_jwk) }
}
