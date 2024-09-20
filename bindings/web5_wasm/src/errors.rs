use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web5::errors::Web5Error;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct WasmWeb5Error {
    variant: String,
    message: String,
}

#[wasm_bindgen]
impl WasmWeb5Error {
    #[wasm_bindgen(getter)]
    pub fn variant(&self) -> String {
        self.variant.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

pub fn map_err(err: Web5Error) -> JsValue {
    let msg = format!("{:?}", err);
    let variant = msg.split('(').next().unwrap_or("Unknown").to_string();

    let js_error = WasmWeb5Error {
        variant,
        message: err.to_string(),
    };

    to_value(&js_error).unwrap_or_else(|_| JsValue::from_str("failed to serialize error"))
}
