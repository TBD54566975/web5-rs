use crypto::Curve;
use keys::key_manager::{local_key_manager::LocalKeyManager, KeyManager};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/**
 * todo spike
 * - BearerDid::from_private_keys()
 * - jwt::sign_jwt()
 * - JwtString::verify()
 * - VerifiableCredential::create()
 * - VerifiableCredential::sign()
 */

#[wasm_bindgen]
pub struct WasmKeyManager {
    key_manager: LocalKeyManager,
}

#[wasm_bindgen]
impl WasmKeyManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmKeyManager {
        let key_manager = LocalKeyManager::new_in_memory();
        WasmKeyManager { key_manager }
    }

    pub fn generate_private_key(&self, curve: String) -> Result<String, JsValue> {
        let curve = match curve.as_str() {
            "Secp256k1" => Curve::Secp256k1,
            "Ed25519" => Curve::Ed25519,
            _ => return Err(JsValue::from_str("Invalid curve type")),
        };
        self.key_manager
            .generate_private_key(curve)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
