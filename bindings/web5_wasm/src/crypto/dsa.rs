use super::jwk::WasmJwk;
use crate::errors::{map_err, Result};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::dsa::{
    ed25519::{Ed25519Generator, Ed25519Signer},
    secp256k1::{Secp256k1Generator, Secp256k1Signer},
    Signer,
};
#[wasm_bindgen]
pub struct WasmSigner {
    inner: Arc<dyn Signer>,
}

impl From<Ed25519Signer> for WasmSigner {
    fn from(value: Ed25519Signer) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

impl From<Secp256k1Signer> for WasmSigner {
    fn from(value: Secp256k1Signer) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

impl From<Arc<dyn Signer>> for WasmSigner {
    fn from(value: Arc<dyn Signer>) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmSigner {
    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        self.inner.sign(payload).map_err(map_err)
    }
}

#[wasm_bindgen]
pub fn generate_ed25519_key() -> Result<WasmJwk> {
    Ok(Ed25519Generator::generate().into())
}

#[wasm_bindgen]
pub fn generate_secp256k1_key() -> Result<WasmJwk> {
    Ok(Secp256k1Generator::generate().into())
}

#[wasm_bindgen]
pub fn new_ed25519_signer(jwk: WasmJwk) -> Result<WasmSigner> {
    Ok(Ed25519Signer::new(jwk.into()).into())
}

#[wasm_bindgen]
pub fn new_secp256k1_signer(jwk: WasmJwk) -> Result<WasmSigner> {
    Ok(Secp256k1Signer::new(jwk.into()).into())
}
