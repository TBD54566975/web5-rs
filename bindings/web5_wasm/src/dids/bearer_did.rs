use crate::errors::{map_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::dids::bearer_did::BearerDid;
use crate::crypto::dsa::WasmSigner;
use crate::crypto::key_managers::WasmKeyManager;
use crate::dids::did::WasmDid;
use crate::dids::document::WasmDocument;
use crate::dids::portable_did::WasmPortableDid;

#[wasm_bindgen]
pub struct WasmBearerDid {
    inner: BearerDid,
}

impl From<WasmBearerDid> for BearerDid {
    fn from(value: WasmBearerDid) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmBearerDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did: WasmDid, document: WasmDocument, key_manager: WasmKeyManager) -> Self {
        Self {
            inner: BearerDid {
                did: did.into(),
                document: document.into(),
                key_manager: key_manager.into(),
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_portable_did(portable_did: WasmPortableDid) -> Result<WasmBearerDid> {
        Ok(Self {
            inner: BearerDid::from_portable_did(portable_did.into()).map_err(map_err)?,
        })
    }

    // todo key exporter for to_portable_did

    #[wasm_bindgen]
    pub fn get_signer(&self, verification_method_id: &str) -> Result<WasmSigner> {
        Ok(self
            .inner
            .get_signer(verification_method_id)
            .map_err(map_err)?
            .into())
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> WasmDid {
        self.inner.did.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> WasmDocument {
        self.inner.document.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn key_manager(&self) -> WasmKeyManager {
        self.inner.key_manager.clone().into()
    }
}
