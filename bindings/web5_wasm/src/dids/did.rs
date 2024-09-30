use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web5::dids::did::Did;

#[wasm_bindgen]
pub struct WasmDid {
    inner: Did,
}

impl From<WasmDid> for Did {
    fn from(value: WasmDid) -> Self {
        value.inner
    }
}

impl From<Did> for WasmDid {
    fn from(value: Did) -> Self {
        WasmDid { inner: value }
    }
}

#[wasm_bindgen]
impl WasmDid {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        uri: String,
        url: String,
        method: String,
        id: String,
        params: JsValue,
        path: Option<String>,
        query: Option<String>,
        fragment: Option<String>,
    ) -> Self {
        let params = if params.is_undefined() {
            None
        } else {
            serde_wasm_bindgen::from_value(params).unwrap_or(Some(HashMap::new()))
        };

        Self {
            inner: Did {
                uri,
                url,
                method,
                id,
                params,
                path,
                query,
                fragment,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.inner.uri.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.inner.url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.inner.method.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn params(&self) -> JsValue {
        match &self.inner.params {
            Some(map) => serde_wasm_bindgen::to_value(map).unwrap_or(JsValue::undefined()),
            None => JsValue::undefined(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<String> {
        self.inner.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn query(&self) -> Option<String> {
        self.inner.query.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn fragment(&self) -> Option<String> {
        self.inner.fragment.clone()
    }
}
