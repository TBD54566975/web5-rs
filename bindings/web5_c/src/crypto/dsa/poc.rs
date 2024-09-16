use std::sync::{atomic::Ordering, Arc};
use web5::crypto::{
    dsa::{ed25519::Ed25519Signer, Signer},
    jwk::Jwk,
};

use super::{rust_signer_sign, CSigner, SIGNER_ID_COUNTER, SIGNER_REGISTRY};

#[no_mangle]
pub extern "C" fn poc_signer_from_foreign(signer: *const CSigner) {
    if signer.is_null() {
        return;
    }

    let signer = unsafe { &*signer };
    let payload = b"Test message";

    let mut out_len: usize = 0;
    (signer.sign)(
        signer.signer_id,
        payload.as_ptr(),
        payload.len(),
        &mut out_len,
    );
}

// todo temporary
#[no_mangle]
pub extern "C" fn poc_signer_from_rust() -> *mut CSigner {
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
