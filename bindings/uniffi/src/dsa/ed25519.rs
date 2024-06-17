use super::{Signer, Verifier};
use crate::errors::Result;
use std::sync::Arc;
use web5::apid::{
    dsa::{
        ed25519::{
            Ed25519Generator as InnerEd25519Generator, Ed25519Signer as InnerEd25519Signer,
            Ed25519Verifier as InnerEd25519Verifier,
        },
        Signer as InnerSigner, Verifier as InnerVerifier,
    },
    jwk::Jwk,
};

pub fn ed25519_generator_generate() -> Jwk {
    InnerEd25519Generator::generate()
}

pub struct Ed25519Signer(pub InnerEd25519Signer);

impl Ed25519Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self(InnerEd25519Signer::new(private_jwk))
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        self.0.sign(payload).map_err(|e| Arc::new(e.into()))
    }

    fn to_inner(&self) -> Arc<dyn InnerSigner> {
        Arc::new(self.0.clone())
    }
}

pub struct Ed25519Verifier(pub InnerEd25519Verifier);

impl Ed25519Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self(InnerEd25519Verifier::new(public_jwk))
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool> {
        self.0
            .verify(payload, signature)
            .map_err(|e| Arc::new(e.into()))
    }

    fn to_inner(&self) -> Arc<dyn InnerVerifier> {
        Arc::new(self.0.clone())
    }
}
