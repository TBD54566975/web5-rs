use std::sync::Arc;

use super::dsa::{Signer, Verifier};

#[derive(Default, Clone)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub issuer: String, // 🚧
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    pub credential_subject: String, // 🚧
}

impl VerifiableCredential {
    pub fn sign(&self, _signer: Arc<dyn Signer>) -> String {
        println!("VerifiableCredential.sign()");
        String::default()
    }

    pub fn verify(_vcjwt: &str) -> Self {
        println!("VerifiableCredential::verify()");
        Self {
            ..Default::default()
        }
    }

    pub fn verify_with_verifier(_vcjwt: &str, _verifier: Arc<dyn Verifier>) -> Self {
        println!("VerifiableCredential::verify_with_verifier()");
        Self {
            ..Default::default()
        }
    }
}
