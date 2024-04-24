use crate::any::Any;
use credentials::vc::{CredentialSubject, VerifiableCredential};
use std::sync::Arc;

pub fn new_verifiable_credential(
    context: Vec<String>,
    id: String,
    r#type: Vec<String>,
    issuer: String,
    issuance_date: i64,
    expiration_date: Option<i64>,
    credential_subject: CredentialSubject,
    evidence: Option<Arc<Any>>,
) -> Arc<VerifiableCredential> {
    Arc::new(VerifiableCredential::new(
        context,
        id,
        r#type,
        issuer,
        issuance_date,
        expiration_date,
        credential_subject,
        evidence.map(|e| (*e).clone().value),
    ))
}
