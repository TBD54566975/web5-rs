use crate::errors::{map_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::jwk::Jwk;

#[wasm_bindgen]
pub struct WasmJwk {
    inner: Jwk,
}

impl From<Jwk> for WasmJwk {
    fn from(value: Jwk) -> Self {
        Self { inner: value }
    }
}

impl From<WasmJwk> for Jwk {
    fn from(value: WasmJwk) -> Self {
        value.inner
    }
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
    pub fn compute_thumbprint(&self) -> Result<String> {
        self.inner.compute_thumbprint().map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn alg(&self) -> Option<String> {
        self.inner.alg.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kty(&self) -> String {
        self.inner.kty.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn crv(&self) -> String {
        self.inner.crv.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn d(&self) -> Option<String> {
        self.inner.d.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> String {
        self.inner.x.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Option<String> {
        self.inner.y.clone()
    }
}
