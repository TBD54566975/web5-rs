pub mod crypto;

pub mod errors;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub type JsFunction;

//     #[wasm_bindgen(method)]
//     fn call(this: &JsFunction);
// }

// #[wasm_bindgen]
// pub fn call_js_function(f: &JsFunction) {
//     f.call();
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "{hello1: Function, hello2: Function}")]
    pub type JsObject1;

    #[wasm_bindgen(method)]
    fn hello1(this: &JsObject1);

    #[wasm_bindgen(method)]
    fn hello2(this: &JsObject1);
}

#[wasm_bindgen]
pub fn call_js_functions(obj: &JsObject1) {
    obj.hello1();
    obj.hello2();
}
