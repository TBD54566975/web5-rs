use jwk::Jwk as InternalJwk;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Jwk(InternalJwk);

impl From<InternalJwk> for Jwk {
    fn from(value: InternalJwk) -> Self {
        Self::new(value.alg, value.kty, value.crv, value.d, value.x, value.y)
    }
}

impl From<&Jwk> for InternalJwk {
    fn from(value: &Jwk) -> Self {
        Self {
            alg: value.alg().clone(),
            kty: value.kty().clone(),
            crv: value.crv().clone(),
            d: value.d().clone(),
            x: value.x().clone(),
            y: value.y().clone(),
        }
    }
}

#[wasm_bindgen]
impl Jwk {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alg: String,
        kty: String,
        crv: String,
        d: Option<String>,
        x: String,
        y: Option<String>,
    ) -> Jwk {
        let jwk = InternalJwk {
            alg,
            kty,
            crv,
            d,
            x,
            y,
        };
        Jwk(jwk)
    }

    #[wasm_bindgen(getter)]
    pub fn alg(&self) -> String {
        self.0.alg.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kty(&self) -> String {
        self.0.kty.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn crv(&self) -> String {
        self.0.crv.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn d(&self) -> Option<String> {
        self.0.d.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> String {
        self.0.x.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Option<String> {
        self.0.y.clone()
    }

    #[wasm_bindgen]
    pub fn compute_thumbprint(&self) -> Result<String, JsValue> {
        self.0
            .compute_thumbprint()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    #[wasm_bindgen]
    pub fn to_public(&self) -> Jwk {
        // todo unwrap no bueno
        let public_jwk = self.0.to_public().unwrap();
        Jwk(public_jwk)
    }
}
