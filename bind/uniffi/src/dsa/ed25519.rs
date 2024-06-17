use super::{RcbSigner, RcbVerifier};
use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::{
    dsa::{
        ed25519::{Ed25519Generator, Ed25519Signer, Ed25519Verifier},
        Signer, Verifier,
    },
    jwk::Jwk,
};

pub fn rcb_ed25519_generator_generate() -> Jwk {
    Ed25519Generator::generate()
}

pub struct RcbEd25519Signer(pub Ed25519Signer);

impl RcbEd25519Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self(Ed25519Signer::new(private_jwk))
    }
}

impl RcbSigner for RcbEd25519Signer {
    fn sign(&self, payload: &[u8]) -> RcbResult<Vec<u8>> {
        self.0.sign(payload).map_err(|e| Arc::new(e.into()))
    }

    fn to_signer(&self) -> Arc<dyn Signer> {
        Arc::new(self.0.clone())
    }
}

pub struct RcbEd25519Verifier(pub Ed25519Verifier);

impl RcbEd25519Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self(Ed25519Verifier::new(public_jwk))
    }
}

impl RcbVerifier for RcbEd25519Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> RcbResult<bool> {
        self.0
            .verify(payload, signature)
            .map_err(|e| Arc::new(e.into()))
    }

    fn to_verifier(&self) -> Arc<dyn Verifier> {
        Arc::new(self.0.clone())
    }
}
