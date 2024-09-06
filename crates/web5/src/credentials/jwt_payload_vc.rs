use super::{
    credential_schema::CredentialSchema, credential_subject::CredentialSubject, issuer::Issuer,
};
use crate::credentials::verifiable_credential_1_1::CredentialStatus;
use crate::errors::Result;
use crate::json::{FromJsonValue, JsonValue, ToJsonValue};
use crate::{
    json::JsonObject,
    rfc3339::{deserialize_optional_system_time, serialize_optional_system_time},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        serialize_with = "serialize_optional_system_time",
        deserialize_with = "deserialize_optional_system_time"
    )]
    pub issuance_date: Option<SystemTime>,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_system_time",
        deserialize_with = "deserialize_optional_system_time"
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

impl FromJsonValue for JwtPayloadVerifiableCredential {
    fn from_json_value(value: &JsonValue) -> Result<Option<Self>> {
        if let JsonValue::Object(ref obj) = *value {
            let json_value = serde_json::to_value(obj)?;
            let value = serde_json::from_value::<Self>(json_value)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl ToJsonValue for JwtPayloadVerifiableCredential {
    fn to_json_value(&self) -> Result<JsonValue> {
        let json_string = serde_json::to_string(self)?;
        let map = serde_json::from_str::<HashMap<String, JsonValue>>(&json_string)?;
        Ok(map.to_json_value()?)
    }
}
