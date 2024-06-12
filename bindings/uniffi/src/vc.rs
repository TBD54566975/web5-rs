use std::sync::Arc;

use crate::inner::{
    dsa::{Signer, Verifier},
    vc::VerifiableCredential as InnerVerifiableCredential,
};

pub struct VerifiableCredential(InnerVerifiableCredential);

impl VerifiableCredential {
    pub fn new(verifiable_credential: InnerVerifiableCredential) -> Self {
        Self(verifiable_credential)
    }

    pub fn verify(vcjwt: &str) -> Self {
        Self(InnerVerifiableCredential::verify(vcjwt))
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Self {
        Self(InnerVerifiableCredential::verify_with_verifier(
            vcjwt, verifier,
        ))
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> String {
        self.0.sign(signer)
    }

    pub fn get_data(&self) -> InnerVerifiableCredential {
        self.0.clone()
    }
}
