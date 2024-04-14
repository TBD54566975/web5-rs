use crate::jwk::Jwk;
use crypto::Curve;
use jwk::Jwk as InternalJwk;
use keys::{
    key::PrivateKey,
    key_manager::{local_key_manager::LocalKeyManager as InternalLocalKeyManager, KeyManager},
};
use std::sync::Arc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct LocalJwkManager(InternalLocalKeyManager);

#[wasm_bindgen]
impl LocalJwkManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(InternalLocalKeyManager::new_in_memory())
    }

    #[wasm_bindgen]
    pub fn generate_private_key(&self, curve: String) -> Result<String, JsValue> {
        let curve = match curve.as_str() {
            "Secp256k1" => Curve::Secp256k1,
            "Ed25519" => Curve::Ed25519,
            _ => return Err(JsValue::from_str("Invalid curve type")),
        };

        let key_alias = self
            .0
            .generate_private_key(curve)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(key_alias)
    }

    #[wasm_bindgen]
    pub fn get_public_key(&self, key_alias: &str) -> Result<Jwk, JsValue> {
        let public_jwk = self
            .0
            .get_public_key(key_alias)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?
            .jwk()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(Jwk::from(public_jwk))
    }

    #[wasm_bindgen]
    pub fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, JsValue> {
        let signed_payload = self
            .0
            .sign(key_alias, payload)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(signed_payload)
    }

    // TODO function type as a return type
    // fn get_signer(&self, key_alias: &str) -> Result<Signer, KeyManagerError> {
    //     let signer = self.key_store.get_signer(key_alias)?;
    //     Ok(signer)
    // }

    #[wasm_bindgen]
    pub fn export_private_keys(&self) -> Result<Vec<Jwk>, JsValue> {
        let private_keys = self
            .0
            .export_private_keys()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        let jwks: Result<Vec<Jwk>, JsValue> = private_keys
            .iter()
            .map(|pk| {
                pk.jwk()
                    .map(Jwk::from)
                    .map_err(|e| JsValue::from_str(&format!("Error converting to JWK: {}", e)))
            })
            .collect();

        jwks
    }

    #[wasm_bindgen]
    pub fn import_private_keys(&self, private_keys: Vec<Jwk>) -> Result<(), JsValue> {
        let internal_jwks: Vec<Arc<dyn PrivateKey>> = private_keys
            .into_iter()
            .map(|jwk| Arc::new(InternalJwk::from(&jwk)) as Arc<dyn PrivateKey>)
            .collect();

        self.0
            .import_private_keys(internal_jwks)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

        Ok(())
    }
}
