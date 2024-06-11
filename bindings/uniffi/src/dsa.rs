use crate::{
    inner::dsa::{
        Ed25519Signer as InnerEd25519Signer, Ed25519Verifier as InnerEd25519Verifier, Signer,
        Verifier,
    },
    keys::Jwk,
};
use std::sync::Arc;

pub struct Ed25519Signer(InnerEd25519Signer);

impl Ed25519Signer {
    pub fn new(private_key: Arc<Jwk>) -> Self {
        Self {
            0: InnerEd25519Signer::new(private_key.to_inner()),
        }
    }

    pub fn from_inner(inner_ed25519_signer: InnerEd25519Signer) -> Self {
        Self {
            0: inner_ed25519_signer,
        }
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, payload: &[u8]) -> Vec<u8> {
        self.0.sign(payload)
    }
}

pub struct Ed25519Verifier(InnerEd25519Verifier);

impl Ed25519Verifier {
    pub fn new(public_key: Arc<Jwk>) -> Self {
        Self {
            0: InnerEd25519Verifier::new(public_key.to_inner()),
        }
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        self.0.verify(message, signature)
    }
}
