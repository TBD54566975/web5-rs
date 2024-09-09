use super::{
    credential_schema::CredentialSchema, credential_subject::CredentialSubject, issuer::Issuer,
};
use crate::credentials::verifiable_credential_1_1::CredentialStatus;
use crate::{
    json::JsonObject,
    datetime::{deserialize_optional_unix_timestamp, serialize_optional_unix_timestamp},
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtPayloadVerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<Issuer>,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_optional_unix_timestamp",
        deserialize_with = "deserialize_optional_unix_timestamp"
    )]
    pub issuance_date: Option<SystemTime>,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_unix_timestamp",
        deserialize_with = "deserialize_optional_unix_timestamp"
    )]
    pub expiration_date: Option<SystemTime>,
    #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<CredentialStatus>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "credentialSubject")]
    pub credential_subject: Option<CredentialSubject>,
    #[serde(rename = "credentialSchema", skip_serializing_if = "Option::is_none")]
    pub credential_schema: Option<CredentialSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<JsonObject>>,
}
