pub mod ed25519;

pub enum Dsa {
    Ed25519,
}

pub trait Signer {
    fn sign(&self, payload: &[u8]) -> Vec<u8>;
}

pub trait Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> bool;
}
