use std::sync::Arc;

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

pub struct Ed25519GeneratorInner {}

impl Ed25519GeneratorInner {
    pub fn generate() -> Jwk {
        unimplemented!()
    }
}

pub struct Ed25519Generator {}

impl Ed25519Generator {
    pub fn generate(&self) -> Jwk {
        Ed25519GeneratorInner::generate()
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

uniffi::include_scaffolding!("web5");
