use super::data_model_validation::validate_vc_data_model;
use super::decode::decode;
use super::CredentialSubject;
use super::Issuer;

use crate::dids::bearer_did::BearerDid;
use crate::errors::Result;
use crate::json::{FromJson, ToJson};
use crate::rfc3339::{
    deserialize_optional_system_time, deserialize_system_time, serialize_optional_system_time,
    serialize_system_time,
};

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: Issuer,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_system_time",
        deserialize_with = "deserialize_system_time"
    )]
    pub issuance_date: SystemTime,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_system_time",
        deserialize_with = "deserialize_optional_system_time"
    )]
    pub expiration_date: Option<SystemTime>,
}

impl FromJson for VerifiableCredential {}
impl ToJson for VerifiableCredential {}

#[derive(Default)]
pub struct VerifiableCredentialCreateOptions {
    pub id: Option<String>,
    pub context: Option<Vec<String>>,
    pub r#type: Option<Vec<String>>,
    pub issuance_date: Option<SystemTime>,
    pub expiration_date: Option<SystemTime>,
}

impl VerifiableCredential {
    pub fn create(
        issuer: Issuer,
        credential_subject: CredentialSubject,
        options: VerifiableCredentialCreateOptions,
    ) -> Result<Self> {
        super::create::create_vc(issuer, credential_subject, options)
    }

    // this function currently only supports Ed25519
    pub fn from_vc_jwt(vc_jwt: &str, verify: bool) -> Result<Self> {
        let vc = decode(vc_jwt, verify)?;

        if verify {
            validate_vc_data_model(&vc)?;
        }

        Ok(vc)
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        super::sign::sign_with_did(self, bearer_did, verification_method_id)
    }
}
