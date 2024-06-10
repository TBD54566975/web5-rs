mod stubbed_in;

use stubbed_in::{
    Did as DidData, Dsa, Ed25519Generator as Ed25519GeneratorInner, Ed25519Signer, Ed25519Verifier,
    InMemoryKeyManager, Jwk as JwkData, Signer, Verifier,
};

// 🚧 Custom DSL's STATIC METHOD's not supported with UDL
// 🚧 *Data struct are used for UDL dictionary's
//
// 🚧 try to prefer defaults over `unimplemented!()`

pub struct Ed25519Generator {}

impl Ed25519Generator {
    pub fn generate(&self) -> JwkData {
        Ed25519GeneratorInner::generate()
    }
}

pub struct Did {}

impl Did {
    pub fn parse(&self, uri: &str) -> DidData {
        DidData::new(uri)
    }
}

uniffi::include_scaffolding!("web5");
