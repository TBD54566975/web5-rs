use wasm_bindgen::JsValue;
use web5::errors::Web5Error;

pub fn map_err(err: Web5Error) -> JsValue {
    JsValue::from_str(&err.to_string())
}
