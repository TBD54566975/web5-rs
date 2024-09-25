use super::{dsa::WasmSigner, jwk::WasmJwk};
use crate::errors::{map_err, Result};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::{
    jwk::Jwk,
    key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
};

#[wasm_bindgen]
pub struct WasmKeyManager {
    inner: Arc<dyn KeyManager>,
}

impl From<InMemoryKeyManager> for WasmKeyManager {
    fn from(value: InMemoryKeyManager) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

#[wasm_bindgen]
impl WasmKeyManager {
    pub fn import_private_jwk(&self, private_jwk: WasmJwk) -> Result<WasmJwk> {
        Ok(self
            .inner
            .import_private_jwk(private_jwk.into())
            .map_err(map_err)?
            .into())
    }

    pub fn get_signer(&self, public_jwk: WasmJwk) -> Result<WasmSigner> {
        Ok(self
            .inner
            .get_signer(public_jwk.into())
            .map_err(map_err)?
            .into())
    }
}

#[wasm_bindgen]
pub fn new_in_memory_key_manager() -> Result<WasmKeyManager> {
    Ok(InMemoryKeyManager::new().into())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ importPrivateJwk: (privateJwk: WasmJwk) => WasmJwk, getSigner: (publicJwk: WasmJwk) => WasmSigner }"
    )]
    pub type ForeignKeyManager;

    #[wasm_bindgen(method)]
    fn import_private_jwk(this: &ForeignKeyManager, private_jwk: WasmJwk) -> WasmJwk;

    #[wasm_bindgen(method)]
    fn get_signer(this: &ForeignKeyManager, public_jwk: WasmJwk) -> WasmSigner;
}

#[wasm_bindgen]
pub fn poc_key_manager_from_foreign(key_manager: &ForeignKeyManager) -> WasmSigner {
    let private_jwk = Jwk {
        alg: Some("Ed25519".to_string()),
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        d: Some("UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g".to_string()),
        x: "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q".to_string(),
        y: None,
    };

    let public_jwk = key_manager.import_private_jwk(private_jwk.into());
    key_manager.get_signer(public_jwk)
}
