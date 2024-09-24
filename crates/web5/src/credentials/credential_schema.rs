use super::verifiable_credential_1_1::VerifiableCredential;
use crate::errors::{Result, Web5Error};
use jsonschema::{Draft, JSONSchema};
use serde::{Deserialize, Serialize};

pub const CREDENTIAL_SCHEMA_TYPE: &str = "JsonSchema";

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CredentialSchema {
    pub id: String,
    pub r#type: String,
}

pub(crate) fn validate_credential_schema(
    verifiable_credential: &VerifiableCredential,
) -> Result<()> {
    let credential_schema = match &verifiable_credential.credential_schema {
        None => return Ok(()),
        Some(c) => c,
    };

    if credential_schema.r#type != CREDENTIAL_SCHEMA_TYPE {
        return Err(Web5Error::Parameter(format!(
            "type must be {}",
            CREDENTIAL_SCHEMA_TYPE
        )));
    }

    let url = &credential_schema.id;

    let response = http_std::fetch(&url, None)?;

    if !(200..300).contains(&response.status_code) {
        return Err(Web5Error::JsonSchema(format!(
            "failed to resolve status code {}",
            response.status_code
        )));
    }

    let json_schema = serde_json::from_slice::<serde_json::Value>(&response.body)?;

    let compiled_schema = JSONSchema::options().compile(&json_schema).map_err(|err| {
        Web5Error::JsonSchema(format!("unable to compile json schema {} {}", url, err))
    })?;

    let draft = compiled_schema.draft();
    if draft == Draft::Draft4 || draft == Draft::Draft6 {
        return Err(Web5Error::JsonSchema(format!(
            "draft unsupported {:?}",
            draft
        )));
    }

    let instance = serde_json::to_value(verifiable_credential)?;
    let result = compiled_schema.validate(&instance);
    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("{} at {}", e, e.instance_path))
            .collect();
        return Err(Web5Error::JsonSchema(format!(
            "validation errors {}",
            error_messages.join(", ")
        )));
    }

    Ok(())
}
