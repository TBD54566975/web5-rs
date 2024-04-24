use crate::jwk::Jwk;
use crypto::{ed25519::Ed25199, secp256k1::Secp256k1, CurveOperations};
use jwk::Jwk as InternalJwk;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = generateEd25519Key)]
pub fn generate_ed25519_key() -> Result<Jwk, JsValue> {
    let internal_jwk = Ed25199::generate().map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let jwk = Jwk::from(internal_jwk);
    Ok(jwk)
}

#[wasm_bindgen(js_name = generateSecp256k1Key)]
pub fn generate_secp256k1_key() -> Result<Jwk, JsValue> {
    let internal_jwk = Secp256k1::generate().map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let jwk = Jwk::from(internal_jwk);
    Ok(jwk)
}

#[wasm_bindgen(js_name = signSecp256k1)]
pub fn sign_secp256k1(private_key: &Jwk, payload: &[u8]) -> Result<Vec<u8>, JsValue> {
    let internal_jwk = InternalJwk::from(private_key);
    Secp256k1::sign(&internal_jwk, payload).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[wasm_bindgen(js_name = signEd25519)]
pub fn sign_ed25519(private_key: &Jwk, payload: &[u8]) -> Result<Vec<u8>, JsValue> {
    let internal_jwk = InternalJwk::from(private_key);
    Ed25199::sign(&internal_jwk, payload).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[wasm_bindgen(js_name = verifySecp256k1)]
pub fn verify_secp256k1(public_key: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), JsValue> {
    let internal_jwk = InternalJwk::from(public_key);
    Secp256k1::verify(&internal_jwk, payload, signature)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[wasm_bindgen(js_name = verifyEd25519)]
pub fn verify_ed25519(public_key: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), JsValue> {
    let internal_jwk = InternalJwk::from(public_key);
    Ed25199::verify(&internal_jwk, payload, signature)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
}
