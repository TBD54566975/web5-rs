use super::{add_signer_to_registry, rust_signer_sign, CSigner};
use crate::crypto::jwk::CJwk;
use std::ptr;
use std::sync::Arc;
use web5::crypto::dsa::{ed25519::Ed25519Signer, Signer};
use web5::crypto::jwk::Jwk;

#[no_mangle]
pub extern "C" fn new_ed25519_signer(cjwk_ptr: *const CJwk) -> *mut CSigner {
    if cjwk_ptr.is_null() {
        return ptr::null_mut();
    }

    let jwk = unsafe { Jwk::from(&*cjwk_ptr) };
    let signer: Arc<dyn Signer> = Arc::new(Ed25519Signer::new(jwk));

    let signer_id = add_signer_to_registry(signer);

    Box::into_raw(Box::new(CSigner {
        signer_id,
        sign: rust_signer_sign,
    }))
}
