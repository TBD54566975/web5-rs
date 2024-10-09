use crate::errors::{map_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    dids::portable_did::PortableDid,
    json::{FromJson, ToJson},
};
use crate::crypto::jwk::WasmJwk;
use crate::dids::document::WasmDocument;

#[wasm_bindgen]
pub struct WasmPortableDid {
    inner: PortableDid,
}

impl From<WasmPortableDid> for PortableDid {
    fn from(value: WasmPortableDid) -> Self {
        value.inner
    }
}

impl From<PortableDid> for WasmPortableDid {
    fn from(value: PortableDid) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmPortableDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did_uri: String, document: WasmDocument, private_keys: Vec<WasmJwk>) -> Self {
        Self {
            inner: PortableDid {
                did_uri,
                document: document.into(),
                private_jwks: private_keys.into_iter().map(|pj| pj.into()).collect(),
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_json_string(json: &str) -> Result<WasmPortableDid> {
        Ok(Self {
            inner: PortableDid::from_json_string(json).map_err(map_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn did_uri(&self) -> String {
        self.inner.did_uri.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> WasmDocument {
        self.inner.document.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn private_keys(&self) -> Vec<WasmJwk> {
        self.inner
            .private_jwks
            .clone()
            .into_iter()
            .map(|j| j.into())
            .collect()
    }
}
