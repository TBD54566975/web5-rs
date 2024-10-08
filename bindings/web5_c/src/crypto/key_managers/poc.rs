use super::{call_get_signer, call_import_private_jwk, CKeyManager};
use crate::{
    c::free_bytes,
    crypto::{
        dsa::{call_sign, free_csigner},
        jwk::{free_cjwk, CJwk},
    },
};
use web5::crypto::jwk::Jwk;

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

    free_cjwk(public_jwk);
    free_csigner(signer);
    free_bytes(signature);
}
