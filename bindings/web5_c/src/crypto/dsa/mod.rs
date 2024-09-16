use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    ptr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
use web5::crypto::{
    dsa::{ed25519::Ed25519Signer, Signer},
    jwk::Jwk,
};
pub mod ed25519;

static SIGNER_ID_COUNTER: AtomicI32 = AtomicI32::new(1);
lazy_static! {
    static ref SIGNER_REGISTRY: Mutex<HashMap<i32, Arc<dyn Signer>>> = Mutex::new(HashMap::new());
}

#[repr(C)]
pub struct CSigner {
    pub signer_id: i32,
    pub sign: extern "C" fn(signer_id: i32, payload: *const u8, payload_len: usize) -> *mut u8,
}

extern "C" fn rust_signer_sign(signer_id: i32, payload: *const u8, payload_len: usize) -> *mut u8 {
    let payload_slice = unsafe { std::slice::from_raw_parts(payload, payload_len) };

    let registry = SIGNER_REGISTRY.lock().unwrap();
    if let Some(signer) = registry.get(&signer_id) {
        if let Ok(signature) = signer.sign(payload_slice) {
            let signature_boxed = signature.into_boxed_slice();
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
) -> *mut u8 {
    unsafe { ((*signer).sign)((*signer).signer_id, payload, payload_len) }
}

// todo temporary
#[no_mangle]
pub extern "C" fn proof_of_concept(signer: *const CSigner) {
    if signer.is_null() {
        return;
    }

    let signer = unsafe { &*signer };
    let payload = b"Test message";

    (signer.sign)(signer.signer_id, payload.as_ptr(), payload.len());
}

// todo temporary
#[no_mangle]
pub extern "C" fn proof_of_concept_2() -> *mut CSigner {
    let private_jwk = Jwk {
        alg: Some("Ed25519".to_string()),
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        d: Some("UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g".to_string()),
        x: "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q".to_string(),
        y: None,
    };
    let signer: Arc<dyn Signer> = Arc::new(Ed25519Signer::new(private_jwk));

    let signer_id = SIGNER_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    SIGNER_REGISTRY.lock().unwrap().insert(signer_id, signer);

    Box::into_raw(Box::new(CSigner {
        signer_id,
        sign: rust_signer_sign,
    }))
}
