use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    ptr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
use web5::crypto::dsa::Signer;

pub mod ed25519;
pub mod poc;

static SIGNER_ID_COUNTER: AtomicI32 = AtomicI32::new(1);
lazy_static! {
    static ref SIGNER_REGISTRY: Mutex<HashMap<i32, Arc<dyn Signer>>> = Mutex::new(HashMap::new());
}

pub fn add_signer_to_registry(signer: Arc<dyn Signer>) -> i32 {
    let signer_id = SIGNER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    SIGNER_REGISTRY.lock().unwrap().insert(signer_id, signer);
    signer_id
}

#[repr(C)]
pub struct CSigner {
    pub signer_id: i32,
    pub sign: extern "C" fn(
        signer_id: i32,
        payload: *const u8,
        payload_len: usize,
        out_len: *mut usize,
    ) -> *mut u8,
}

#[no_mangle]
pub extern "C" fn alloc_csigner() -> *mut CSigner {
    let signer = CSigner {
        signer_id: 0,
        sign: rust_signer_sign,
    };
    Box::into_raw(Box::new(signer))
}

#[no_mangle]
pub extern "C" fn free_csigner(signer: *mut CSigner) {
    if !signer.is_null() {
        unsafe {
            let _ = Box::from_raw(signer);
        }
    }
}

pub extern "C" fn rust_signer_sign(
    signer_id: i32,
    payload: *const u8,
    payload_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    let payload_slice = unsafe { std::slice::from_raw_parts(payload, payload_len) };

    let registry = SIGNER_REGISTRY.lock().unwrap();
    if let Some(signer) = registry.get(&signer_id) {
        if let Ok(signature) = signer.sign(payload_slice) {
            let signature_len = signature.len();
            let signature_boxed = signature.into_boxed_slice();
            unsafe { *out_len = signature_len };
            return Box::into_raw(signature_boxed) as *mut u8;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn call_sign(
    signer: *const CSigner,
    payload: *const u8,
    payload_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    unsafe { ((*signer).sign)((*signer).signer_id, payload, payload_len, out_len) }
}
