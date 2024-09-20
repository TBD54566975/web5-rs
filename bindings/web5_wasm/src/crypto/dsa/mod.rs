use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "{ sign: (payload: Uint8Array) => Uint8Array }")]
    pub type Signer;

    #[wasm_bindgen(method)]
    fn sign(this: &Signer, payload: &[u8]) -> Vec<u8>;
}

#[wasm_bindgen]
pub fn pass_signer(signer: &Signer) -> () {
    println!("pass_signer called in rust");
    let payload = b"hello from rust";
    let result = signer.sign(payload);
    println!(
        "result from rust {}",
        String::from_utf8(result.clone()).unwrap()
    );
}
