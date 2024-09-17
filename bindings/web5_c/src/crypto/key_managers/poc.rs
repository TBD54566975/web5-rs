use crate::crypto::{dsa::call_sign, jwk::CJwk};

use super::{
    add_key_manager_to_registry, call_get_signer, call_import_private_jwk,
    rust_key_manager_get_signer, rust_key_manager_import_private_jwk, CKeyManager,
};
use std::sync::Arc;
use web5::crypto::{
    jwk::Jwk,
    key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
};

#[no_mangle]
pub extern "C" fn poc_key_manager_from_foreign(manager: *const CKeyManager) {
    if manager.is_null() {
        return;
    }

    let manager = unsafe { &*manager };
    let private_jwk = Jwk {
        alg: Some("Ed25519".to_string()),
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        d: Some("UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g".to_string()),
        x: "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q".to_string(),
        y: None,
    };

    let public_jwk = call_import_private_jwk(manager, &CJwk::from(private_jwk));

    let signer = call_get_signer(manager, public_jwk);

    let payload = b"Test message";
    let mut out_len: usize = 0;
    let signature = call_sign(signer, payload.as_ptr(), payload.len(), &mut out_len);

    if !signature.is_null() {
        let signature_slice = unsafe { std::slice::from_raw_parts(signature, out_len) };
        println!("Signature: {:?}", signature_slice);
        unsafe {
            let _ = Box::from_raw(signature);
        }
    }
}

#[no_mangle]
pub extern "C" fn poc_key_manager_from_rust() -> *mut CKeyManager {
    let manager: Arc<dyn KeyManager> = Arc::new(InMemoryKeyManager::new());

    let manager_id = add_key_manager_to_registry(manager);

    Box::into_raw(Box::new(CKeyManager {
        manager_id,
        import_private_jwk: rust_key_manager_import_private_jwk,
        get_signer: rust_key_manager_get_signer,
    }))
}
