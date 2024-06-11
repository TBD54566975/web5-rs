use crate::inner::{
    dsa::{Signer, Verifier},
    vc::VerifiableCredential as InnerVerifiableCredential,
};
use std::sync::{Arc, RwLock};

pub struct VerifiableCredential(Arc<RwLock<InnerVerifiableCredential>>);

impl VerifiableCredential {
    pub fn new(
        context: Vec<String>,
        id: String,
        r#type: Vec<String>,
        issuer: String,
        issuance_date: String,
        expiration_date: Option<String>,
        credential_subject: String,
    ) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerVerifiableCredential {
                context,
                id,
                r#type,
                issuer,
                issuance_date,
                expiration_date,
                credential_subject,
            })),
        }
    }

    pub fn from_inner(inner: InnerVerifiableCredential) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> String {
        self.0.read().unwrap().sign(signer)
    }

    pub fn verify(vcjwt: String) -> Self {
        let inner = InnerVerifiableCredential::verify(vcjwt);
        Self::from_inner(inner)
    }

    pub fn verify_with_verifier(vcjwt: String, verifier: Arc<dyn Verifier>) -> Self {
        let inner = InnerVerifiableCredential::verify_with_verifier(vcjwt, verifier);
        Self::from_inner(inner)
    }

    pub fn get_context(&self) -> Vec<String> {
        self.0.read().unwrap().context.clone()
    }

    pub fn set_context(&self, context: Vec<String>) {
        let mut inner = self.0.write().unwrap();
        inner.context = context;
    }

    pub fn get_id(&self) -> String {
        self.0.read().unwrap().id.clone()
    }

    pub fn set_id(&self, id: String) {
        let mut inner = self.0.write().unwrap();
        inner.id = id;
    }

    pub fn get_type(&self) -> Vec<String> {
        self.0.read().unwrap().r#type.clone()
    }

    pub fn set_type(&self, r#type: Vec<String>) {
        let mut inner = self.0.write().unwrap();
        inner.r#type = r#type;
    }

    pub fn get_issuer(&self) -> String {
        self.0.read().unwrap().issuer.clone()
    }

    pub fn set_issuer(&self, issuer: String) {
        let mut inner = self.0.write().unwrap();
        inner.issuer = issuer;
    }

    pub fn get_issuance_date(&self) -> String {
        self.0.read().unwrap().issuance_date.clone()
    }

    pub fn set_issuance_date(&self, issuance_date: String) {
        let mut inner = self.0.write().unwrap();
        inner.issuance_date = issuance_date;
    }

    pub fn get_expiration_date(&self) -> Option<String> {
        self.0.read().unwrap().expiration_date.clone()
    }

    pub fn set_expiration_date(&self, expiration_date: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.expiration_date = expiration_date;
    }

    pub fn get_credential_subject(&self) -> String {
        self.0.read().unwrap().credential_subject.clone()
    }

    pub fn set_credential_subject(&self, credential_subject: String) {
        let mut inner = self.0.write().unwrap();
        inner.credential_subject = credential_subject;
    }
}
