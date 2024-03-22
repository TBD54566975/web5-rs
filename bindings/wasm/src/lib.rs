use jwk::jwk::JWK as CoreJWK;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JWK {
    inner: CoreJWK,
}

#[wasm_bindgen]
impl JWK {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JWK {
        JWK {
            inner: CoreJWK {
                alg: None,
                kty: None,
                crv: None,
                d: None,
                x: None,
                y: None,
            },
        }
    }

    pub fn compute_thumbprint(&self) -> Result<String, JsValue> {
        self.inner
            .compute_thumbprint()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn set_alg(&mut self, alg: String) {
        self.inner.alg = Some(alg);
    }

    pub fn set_kty(&mut self, kty: String) {
        self.inner.kty = Some(kty);
    }

    pub fn set_crv(&mut self, crv: String) {
        self.inner.crv = Some(crv);
    }

    pub fn set_d(&mut self, d: String) {
        self.inner.d = Some(d);
    }

    pub fn set_x(&mut self, x: String) {
        self.inner.x = Some(x);
    }

    pub fn set_y(&mut self, y: String) {
        self.inner.y = Some(y);
    }
}
