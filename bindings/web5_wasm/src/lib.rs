use wasm_bindgen::prelude::*;
use web5::crypto::jwk::Jwk as InnerJwk;

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
