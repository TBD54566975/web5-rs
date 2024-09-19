use crate::errors::map_err;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web5::crypto::jwk::Jwk;

#[wasm_bindgen]
pub struct WasmJwk {
    inner: Jwk,
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
            inner: Jwk {
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
        self.inner.compute_thumbprint().map_err(map_err)
    }
}
