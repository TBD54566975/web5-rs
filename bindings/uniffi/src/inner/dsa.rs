use crate::inner::keys::Jwk;

pub enum Dsa {
    Ed25519,
}

pub trait Signer: Send + Sync {
    fn sign(&self, _payload: &[u8]) -> Vec<u8>;
}

pub trait Verifier: Send + Sync {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool;
}

// pub struct Ed25519Generator {}

// impl Ed25519Generator {
//     pub fn generate() -> Jwk {
//         unimplemented!()
//     }
// }

pub struct Ed25519Signer {}

impl Ed25519Signer {
    pub fn new(_private_key: Jwk) -> Self {
        println!("Invoked Ed25519Signer::new()");
        Self {}
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, _payload: &[u8]) -> Vec<u8> {
        println!("Invoked Ed25519Signer.sign()");
        Vec::new()
    }
}

pub struct Ed25519Verifier {}

impl Ed25519Verifier {
    pub fn new(_public_key: Jwk) -> Self {
        println!("Invoked Ed25519Verifier::new()");
        Self {}
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        println!("Invoked Ed25519Verifier.verify()");
        true
    }
}
