use std::{collections::HashMap, sync::Arc};

// 🚧 Custom DSL's STATIC METHOD's not supported with UDL
// 🚧 *Data struct are used for UDL dictionary's
//
// 🚧 try to prefer defaults over `unimplemented!()`

pub struct Jwk {
    pub alg: String,
    pub kty: String,
    pub crv: String,
    pub d: Option<String>,
    pub x: String,
    pub y: Option<String>,
}

pub struct InMemoryKeyManager {}

impl InMemoryKeyManager {
    pub fn generate_key_material(&self) -> Jwk {
        unimplemented!()
    }

    pub fn get_signer(&self, _public_key: Jwk) -> Arc<Ed25519Signer> {
        unimplemented!()
    }

    pub fn import_key(&self, _private_key: Jwk) -> Jwk {
        unimplemented!()
    }
}

pub enum Dsa {
    Ed25519,
}

pub trait Signer: Send + Sync {
    fn sign(&self, _payload: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}

pub trait Verifier: Send + Sync {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unimplemented!()
    }
}

pub struct Ed25519Generator {}

impl Ed25519Generator {
    pub fn generate() -> Jwk {
        unimplemented!()
    }
}

pub struct Ed25519Signer {}

impl Ed25519Signer {
    pub fn new(_private_key: Jwk) -> Self {
        unimplemented!()
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, _payload: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct Ed25519Verifier {}

impl Ed25519Verifier {
    pub fn new(_public_key: Jwk) -> Self {
        unimplemented!()
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unimplemented!()
    }
}

pub struct Did {
    pub uri: String,
    pub url: String,
    pub method: String,
    pub id: String,
    pub params: Option<HashMap<String, String>>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Did {
    pub fn new(_uri: &str) -> Self {
        unimplemented!()
    }
}
