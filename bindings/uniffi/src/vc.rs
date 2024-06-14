use std::sync::Arc;
use web5::apid::{
    credentials::verifiable_credential_11::VerifiableCredential as InnerVerifiableCredential,
    dsa::{Signer, Verifier},
};

pub struct VerifiableCredential(InnerVerifiableCredential);

impl VerifiableCredential {
    pub fn new(verifiable_credential: InnerVerifiableCredential) -> Self {
        Self {
            0: verifiable_credential,
        }
    }

    pub fn verify(vcjwt: &str) -> Self {
        Self {
            0: InnerVerifiableCredential::verify(vcjwt),
        }
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Self {
        Self {
            0: InnerVerifiableCredential::verify_with_verifier(vcjwt, verifier),
        }
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> String {
        self.0.sign(signer)
    }

    pub fn get_data(&self) -> InnerVerifiableCredential {
        self.0.clone()
    }
}
