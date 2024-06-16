use super::{RcbSigner, RcbVerifier};
use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::{
    dsa::{
        ed25519::{Ed25519Signer as InnerEd25519Signer, Ed25519Verifier as InnerEd25519Verifier},
        Signer as InnerSigner, Verifier as InnerVerifier,
    },
    jwk::Jwk,
};

pub struct RcbEd25519Signer(InnerEd25519Signer);

impl RcbEd25519Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self {
            0: InnerEd25519Signer::new(private_jwk),
        }
    }

    pub fn from_inner(inner: InnerEd25519Signer) -> Self {
        Self { 0: inner }
    }
}

impl RcbSigner for RcbEd25519Signer {
    fn sign(&self, payload: &[u8]) -> RcbResult<Vec<u8>> {
        self.0.sign(payload).map_err(|e| Arc::new(e.into()))
    }

    fn to_inner(&self) -> Arc<dyn InnerSigner> {
        Arc::new(self.0.clone())
    }
}

pub struct RcbEd25519Verifier(InnerEd25519Verifier);

impl RcbEd25519Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self {
            0: InnerEd25519Verifier::new(public_jwk),
        }
    }
}

impl RcbVerifier for RcbEd25519Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> RcbResult<bool> {
        self.0
            .verify(payload, signature)
            .map_err(|e| Arc::new(e.into()))
    }

    fn to_inner(&self) -> Arc<dyn InnerVerifier> {
        Arc::new(self.0.clone())
    }
}