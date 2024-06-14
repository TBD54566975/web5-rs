use crate::apid::dsa::{Signer, Verifier};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::Arc,
};

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct NamedIssuer {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Issuer {
    String(String),
    Object(NamedIssuer),
}

impl<I> From<I> for Issuer
where
    I: Into<String>,
{
    fn from(s: I) -> Self {
        Issuer::String(s.into())
    }
}

impl Display for Issuer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Issuer::String(s) => write!(f, "{}", s),
            Issuer::Object(ni) => write!(f, "{}", ni.id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,

    // 🚧 UDL support
    // pub issuer: Issuer,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
    pub credential_subject: CredentialSubject,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten)]
    pub params: Option<HashMap<String, String>>,
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        context: Vec<String>,
        r#type: Vec<String>,
        // 🚧 UDL
        // issuer: Issuer,
        issuer: String,
        issuance_date: String,
        expiration_date: Option<String>,
        credential_subject: CredentialSubject,
    ) -> Self {
        let context_with_base = std::iter::once(BASE_CONTEXT.to_string())
            .chain(context.into_iter().filter(|c| c != BASE_CONTEXT))
            .collect::<Vec<_>>();

        let type_with_base = std::iter::once(BASE_TYPE.to_string())
            .chain(r#type.into_iter().filter(|t| t != BASE_TYPE))
            .collect::<Vec<_>>();

        Self {
            context: context_with_base,
            id,
            r#type: type_with_base,
            issuer,
            issuance_date,
            expiration_date,
            credential_subject,
        }
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> String {
        unimplemented!()
    }

    pub fn verify(jwt: &str) -> Self {
        // 🚧 call VerifiableCredential::verify_with_verifier with Ed25519Verifier
        unimplemented!()
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Self {
        unimplemented!()
    }
}
