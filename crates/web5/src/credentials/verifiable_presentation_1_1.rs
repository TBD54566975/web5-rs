use crate::credentials::verifiable_credential_1_1::VerifiableCredential;
use crate::credentials::VerificationError;
use crate::datetime::{
    deserialize_optional_rfc3339, deserialize_rfc3339, serialize_optional_rfc3339,
    serialize_rfc3339,
};
use crate::dids::bearer_did::BearerDid;
use crate::dids::did::Did;
use crate::errors::{Result, Web5Error};
use crate::jose::{Jwt, JwtClaims};
use crate::json::{json_value_type_name, FromJsonValue, JsonValue, ToJsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

pub const BASE_PRESENTATION_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const BASE_PRESENTATION_TYPE: &str = "VerifiablePresentation";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub holder: String,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_rfc3339",
        deserialize_with = "deserialize_rfc3339"
    )]
    pub issuance_date: SystemTime,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_rfc3339",
        deserialize_with = "deserialize_optional_rfc3339",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<SystemTime>,
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<String>,
}

#[derive(Default, Clone)]
pub struct VerifiablePresentationCreateOptions {
    pub id: Option<String>,
    pub context: Option<Vec<String>>,
    pub r#type: Option<Vec<String>>,
    pub issuance_date: Option<SystemTime>,
    pub expiration_date: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtPayloadVerifiablePresentation {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_optional_rfc3339",
        deserialize_with = "deserialize_optional_rfc3339",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub issuance_date: Option<SystemTime>,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_rfc3339",
        deserialize_with = "deserialize_optional_rfc3339",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<SystemTime>,
    #[serde(rename = "verifiableCredential", skip_serializing_if = "Vec::is_empty")]
    pub verifiable_credential: Vec<String>,
}

impl FromJsonValue for JwtPayloadVerifiablePresentation {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::Object(ref obj) = *value {
            let json_value = serde_json::to_value(obj)?;
            let value = serde_json::from_value::<Self>(json_value)?;
            Ok(value)
        } else {
            Err(Web5Error::Json(format!(
                "expected object, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl ToJsonValue for JwtPayloadVerifiablePresentation {
    fn to_json_value(&self) -> Result<JsonValue> {
        let json_string = serde_json::to_string(self)?;
        let map = serde_json::from_str::<HashMap<String, JsonValue>>(&json_string)?;
        Ok(map.to_json_value()?)
    }
}

impl VerifiablePresentation {
    pub fn create(
        holder: String,
        vc_jwts: Vec<String>,
        options: Option<VerifiablePresentationCreateOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        if Did::parse(&holder).is_err() {
            return Err(Web5Error::Parameter(
                "holder must be a valid DID URI".into(),
            ));
        }

        // Verify vcjwts
        for vc_jwt in vc_jwts.clone() {
            VerifiableCredential::from_vc_jwt(&vc_jwt, true)?;
        }

        let context = build_vp_context(options.context);
        let r#type = build_vp_type(options.r#type);
        let id = options
            .id
            .unwrap_or_else(|| format!("urn:uuid:{}", Uuid::new_v4()));

        let verifiable_presentation = VerifiablePresentation {
            context,
            id,
            r#type,
            holder,
            issuance_date: options.issuance_date.unwrap_or_else(SystemTime::now),
            expiration_date: options.expiration_date,
            verifiable_credential: vc_jwts,
        };

        Ok(verifiable_presentation)
    }

    pub fn from_vp_jwt(vp_jwt: &str, verify: bool) -> Result<Self> {
        let verifiable_presentation = decode_vp_jwt(vp_jwt, verify)?;

        if verify {
            validate_vp_data_model(&verifiable_presentation)?;
        }

        Ok(verifiable_presentation)
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        sign_presentation_with_did(self, bearer_did, verification_method_id)
    }
}

pub fn sign_presentation_with_did(
    vp: &VerifiablePresentation,
    bearer_did: &BearerDid,
    verification_method_id: Option<String>,
) -> Result<String> {
    if !vp.holder.starts_with(&bearer_did.did.uri) {
        return Err(Web5Error::Parameter(format!(
            "Bearer DID URI {} does not match holder {}",
            bearer_did.did.uri, vp.holder
        )));
    }

    let vp_claims = JwtPayloadVerifiablePresentation {
        context: vp.context.clone(),
        id: Some(vp.id.clone()),
        r#type: vp.r#type.clone(),
        holder: Some(vp.holder.clone()),
        verifiable_credential: vp.verifiable_credential.clone(),
        issuance_date: Some(vp.issuance_date),
        expiration_date: vp.expiration_date,
    };

    let mut additional_properties: HashMap<String, JsonValue> = HashMap::new();
    additional_properties.insert("vp".to_string(), vp_claims.to_json_value()?);

    let claims = JwtClaims {
        iss: Some(vp.holder.clone()),
        jti: Some(vp.id.clone()),
        sub: None,
        nbf: Some(vp.issuance_date),
        iat: Some(SystemTime::now()),
        exp: vp.expiration_date,
        additional_properties: Some(additional_properties),
    };

    let jwt = Jwt::from_claims(&claims, bearer_did, verification_method_id)?;
    Ok(jwt.compact_jws)
}

fn build_vp_context(context: Option<Vec<String>>) -> Vec<String> {
    let mut contexts = context.unwrap_or_else(|| vec![BASE_PRESENTATION_CONTEXT.to_string()]);
    if !contexts.contains(&BASE_PRESENTATION_CONTEXT.to_string()) {
        contexts.insert(0, BASE_PRESENTATION_CONTEXT.to_string());
    }
    contexts
}

fn build_vp_type(r#type: Option<Vec<String>>) -> Vec<String> {
    let mut types = r#type.unwrap_or_else(|| vec![BASE_PRESENTATION_TYPE.to_string()]);
    if !types.contains(&BASE_PRESENTATION_TYPE.to_string()) {
        types.insert(0, BASE_PRESENTATION_TYPE.to_string());
    }
    types
}

pub fn decode_vp_jwt(vp_jwt: &str, verify_signature: bool) -> Result<VerifiablePresentation> {
    let jwt = Jwt::from_compact_jws(vp_jwt, verify_signature)?;

    let jti = jwt
        .claims
        .jti
        .ok_or(VerificationError::MissingClaim("jti".to_string()))?;
    let iss = jwt
        .claims
        .iss
        .ok_or(VerificationError::MissingClaim("issuer".to_string()))?;
    let nbf = jwt
        .claims
        .nbf
        .ok_or(VerificationError::MissingClaim("not_before".to_string()))?;
    let exp = jwt.claims.exp;

    let vp_payload = JwtPayloadVerifiablePresentation::from_json_value(
        jwt.claims
            .additional_properties
            .ok_or(VerificationError::MissingClaim("vp".to_string()))?
            .get("vp")
            .ok_or(VerificationError::MissingClaim("vp".to_string()))?,
    )?;

    if let Some(id) = vp_payload.id {
        if id != jti {
            return Err(VerificationError::ClaimMismatch("id".to_string()).into());
        }
    }

    let vp_issuer = vp_payload.holder.clone();
    if let Some(holder) = vp_payload.holder {
        if iss != holder {
            return Err(VerificationError::ClaimMismatch("holder".to_string()).into());
        }
    }

    let verifiable_presentation = VerifiablePresentation {
        context: vp_payload.context,
        id: jti.to_string(),
        r#type: vp_payload.r#type,
        holder: vp_issuer.unwrap_or_default(),
        issuance_date: nbf,
        expiration_date: exp,
        verifiable_credential: vp_payload.verifiable_credential,
    };

    Ok(verifiable_presentation)
}

pub fn validate_vp_data_model(
    vp: &VerifiablePresentation,
) -> std::result::Result<(), VerificationError> {
    // Required fields ["@context", "id", "type", "holder", "verifiableCredential"]
    if vp.id.is_empty() {
        return Err(VerificationError::DataModelValidationError(
            "missing id".to_string(),
        ));
    }

    if vp.context.is_empty() || vp.context[0] != BASE_PRESENTATION_CONTEXT {
        return Err(VerificationError::DataModelValidationError(
            "missing or invalid context".to_string(),
        ));
    }

    if vp.r#type.is_empty() || vp.r#type[0] != BASE_PRESENTATION_TYPE {
        return Err(VerificationError::DataModelValidationError(
            "missing or invalid type".to_string(),
        ));
    }

    if vp.holder.is_empty() {
        return Err(VerificationError::DataModelValidationError(
            "missing holder".to_string(),
        ));
    }

    let now = SystemTime::now();
    if vp.issuance_date > now {
        return Err(VerificationError::DataModelValidationError(
            "issuance date in future".to_string(),
        ));
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = &vp.expiration_date {
        if expiration_date < &now {
            return Err(VerificationError::DataModelValidationError(
                "presentation expired".to_string(),
            ));
        }
    }

    // Verify vc_jwts
    for vc_jwt in vp.verifiable_credential.clone() {
        VerifiableCredential::from_vc_jwt(&vc_jwt, true).map_err(|e| {
            VerificationError::DataModelValidationError(format!("invalid vc_jwt: {}", e))
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::{CredentialSubject, Issuer};
    use crate::dids::methods::did_jwk::DidJwk;
    fn setup_vc_issuer_and_holder() -> (BearerDid, String, BearerDid, String) {
        let vc_issuer_did = DidJwk::create(None).expect("Failed to create VC issuer DID");
        let vc_issuer_uri = vc_issuer_did.did.uri.clone();

        let holder = DidJwk::create(None).expect("Failed to create holder DID");
        let holder_uri = holder.did.uri.clone();

        (vc_issuer_did, vc_issuer_uri, holder, holder_uri)
    }

    fn create_verifiable_credential(vc_issuer_uri: &str) -> VerifiableCredential {
        let credential_subject = CredentialSubject {
            id: vc_issuer_uri.to_string(),
            ..Default::default()
        };

        VerifiableCredential::create(
            Issuer::String(vc_issuer_uri.to_string()),
            credential_subject,
            None,
        )
        .expect("Failed to create Verifiable Credential")
    }

    fn sign_verifiable_credential(vc: &VerifiableCredential, vc_issuer_did: &BearerDid) -> String {
        vc.sign(vc_issuer_did, None)
            .expect("Failed to sign Verifiable Credential")
    }

    #[test]
    fn test_create_verifiable_presentation() {
        let (vc_issuer_did, vc_issuer_uri, _holder, holder_uri) = setup_vc_issuer_and_holder();

        let vc = create_verifiable_credential(&vc_issuer_uri);

        let vc_jwt = sign_verifiable_credential(&vc, &vc_issuer_did);

        let vp = VerifiablePresentation::create(holder_uri.clone(), vec![vc_jwt.clone()], None)
            .expect("Failed to create Verifiable Presentation");

        assert_eq!(vp.holder, holder_uri);
        assert_eq!(vp.context[0], BASE_PRESENTATION_CONTEXT);
        assert_eq!(vp.r#type[0], BASE_PRESENTATION_TYPE);
        assert_eq!(vp.verifiable_credential.len(), 1);
        assert_eq!(vp.verifiable_credential[0], vc_jwt);

        assert!(vp.issuance_date <= SystemTime::now());
        assert!(vp.expiration_date.is_none() || vp.expiration_date.unwrap() > SystemTime::now());

        validate_vp_data_model(&vp).expect("Verifiable Presentation data model validation failed");
    }

    #[test]
    fn test_verifiable_presentation_expiration() {
        let (vc_issuer_did, vc_issuer_uri, _holder, holder_uri) = setup_vc_issuer_and_holder();

        let vc = create_verifiable_credential(&vc_issuer_uri);

        let vc_jwt = sign_verifiable_credential(&vc, &vc_issuer_did);

        let expired_expiration_date = SystemTime::now() - std::time::Duration::from_secs(3600); // 1 hour ago
        let vp = VerifiablePresentation::create(
            holder_uri.clone(),
            vec![vc_jwt.clone()],
            Some(VerifiablePresentationCreateOptions {
                expiration_date: Some(expired_expiration_date),
                ..Default::default()
            }),
        )
        .expect("Failed to create Verifiable Presentation");

        let validation_result = validate_vp_data_model(&vp);

        match validation_result {
            Err(VerificationError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "presentation expired".to_string());
            }
            _ => panic!(
                "Verifiable Presentation should be considered expired, but it passed validation"
            ),
        }
    }

    #[test]
    fn test_verifiable_presentation_sign() {
        let (vc_issuer_did, vc_issuer_uri, holder, holder_uri) = setup_vc_issuer_and_holder();

        let vc = create_verifiable_credential(&vc_issuer_uri);

        let vc_jwt = sign_verifiable_credential(&vc, &vc_issuer_did);

        let vp = VerifiablePresentation::create(holder_uri.clone(), vec![vc_jwt.clone()], None)
            .expect("Failed to create Verifiable Presentation");

        // Sign the Verifiable Presentation
        let vp_jwt = vp
            .sign(&holder, None)
            .expect("Failed to sign Verifiable Presentation");

        // Decode the signed Verifiable Presentation JWT
        let decoded_vp = VerifiablePresentation::from_vp_jwt(&vp_jwt, true)
            .expect("Failed to decode signed Verifiable Presentation JWT");

        // Verify that the decoded Verifiable Presentation matches the original
        assert_eq!(decoded_vp.holder, vp.holder);
        assert_eq!(decoded_vp.context, vp.context);
        assert_eq!(decoded_vp.r#type, vp.r#type);
        assert_eq!(decoded_vp.verifiable_credential, vp.verifiable_credential);

        // Validate the signed Verifiable Presentation data model
        validate_vp_data_model(&decoded_vp)
            .expect("Signed Verifiable Presentation data model validation failed");
    }
}
