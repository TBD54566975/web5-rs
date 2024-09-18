use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;
use web5::crypto::dsa::Signer;
use web5::crypto::jwk::Jwk as InnerJwk;
use web5::crypto::key_managers::in_memory_key_manager::InMemoryKeyManager as InnerKeyManager;
use web5::crypto::key_managers::{KeyExporter, KeyManager};
use web5::errors::Web5Error;

// Wasm wrapper for the JWK struct
#[derive(Serialize)]
#[wasm_bindgen]
pub struct WasmJwk {
    inner: InnerJwk,
}

#[wasm_bindgen]
impl WasmJwk {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alg: Option<String>,
        kty: String,
        crv: String,
        d: Option<String>,
        x: String,
        y: Option<String>,
    ) -> WasmJwk {
        WasmJwk {
            inner: InnerJwk {
                alg,
                kty,
                crv,
                d,
                x,
                y,
            },
        }
    }

    #[wasm_bindgen]
    pub fn compute_thumbprint(&self) -> Result<String, JsValue> {
        self.inner
            .compute_thumbprint()
            .map_err(|err| JsValue::from_str(&err.to_string()))
    }
}

// Wasm wrapper for the InMemoryKeyManager
#[wasm_bindgen]
pub struct WasmKeyManager {
    inner: InnerKeyManager,
}

#[wasm_bindgen]
impl WasmKeyManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmKeyManager {
        WasmKeyManager {
            inner: InnerKeyManager::new(),
        }
    }

    #[wasm_bindgen]
    pub fn import_private_jwk(&self, private_jwk: WasmJwk) -> Result<WasmJwk, JsValue> {
        match self.inner.import_private_jwk(private_jwk.inner) {
            Ok(public_jwk) => Ok(WasmJwk { inner: public_jwk }),
            Err(err) => Err(JsValue::from_str(&err.to_string())),
        }
    }

    #[wasm_bindgen]
    pub fn get_signer(&self, public_jwk: WasmJwk) -> Result<WasmSigner, JsValue> {
        match self.inner.get_signer(public_jwk.inner) {
            Ok(signer) => Ok(WasmSigner { inner: signer }),
            Err(err) => Err(JsValue::from_str(&err.to_string())),
        }
    }

    #[wasm_bindgen]
    pub fn export_private_jwks(&self) -> Result<JsValue, JsValue> {
        match self.inner.export_private_jwks() {
            Ok(jwks) => {
                let wasm_jwks: Vec<WasmJwk> =
                    jwks.into_iter().map(|jwk| WasmJwk { inner: jwk }).collect();
                let js_array =
                    to_value(&wasm_jwks).map_err(|err| JsValue::from_str(&err.to_string()))?;
                Ok(js_array)
            }
            Err(err) => Err(JsValue::from_str(&err.to_string())),
        }
    }
}

// Wasm wrapper for the Signer trait
#[wasm_bindgen]
pub struct WasmSigner {
    inner: Arc<dyn Signer>,
}

#[wasm_bindgen]
impl WasmSigner {
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JsValue> {
        match self.inner.sign(message) {
            Ok(signature) => Ok(signature),
            Err(err) => Err(JsValue::from_str(&err.to_string())),
        }
    }
}
