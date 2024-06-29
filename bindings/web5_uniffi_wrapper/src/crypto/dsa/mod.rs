pub mod ed25519;

use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::dsa::{Signer as InnerSigner, Verifier as InnerVerifier};

pub trait Signer: Send + Sync {
    fn sign(&self, payload: Vec<u8>) -> Result<Vec<u8>>;
}

pub struct ToOuterSigner(pub Arc<dyn InnerSigner>);

impl Signer for ToOuterSigner {
    fn sign(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self.0.sign(&payload)?)
    }
}

pub struct ToInnerSigner(pub Arc<dyn Signer>);

impl InnerSigner for ToInnerSigner {
    fn sign(&self, payload: &[u8]) -> web5::crypto::dsa::Result<Vec<u8>> {
        let signature = self.0.sign(Vec::from(payload)).unwrap(); // 🚧 unwrap, need a .into() I think
        Ok(signature)
    }
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: Vec<u8>, signature: Vec<u8>) -> Result<bool>;
}

pub struct ToInnerVerifier(pub Arc<dyn Verifier>);

impl InnerVerifier for ToInnerVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> web5::crypto::dsa::Result<bool> {
        let verified = self
            .0
            .verify(Vec::from(payload), Vec::from(signature))
            .unwrap(); // 🚧 unwrap, need a .into() I think
        Ok(verified)
    }
}
