use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use josekit::jws::JwsHeader;
use josekit::jwt::JwtPayload;
use uuid::Uuid;
use crate::credentials::josekit::{JoseSigner, JoseVerifier, JoseVerifierAlwaysTrue};
use crate::credentials::verifiable_credential_1_1::{VerifiableCredential};
use crate::credentials::VerificationError;
use crate::crypto::dsa::ed25519::Ed25519Verifier;
use crate::crypto::dsa::Signer;
use crate::dids::bearer_did::BearerDid;
use crate::dids::data_model::document::FindVerificationMethodOptions;
use crate::dids::did::Did;
use crate::dids::resolution::resolution_metadata::ResolutionMetadataError;
use crate::dids::resolution::resolution_result::ResolutionResult;
use crate::errors::{Result, Web5Error};
use crate::rfc3339::{
    deserialize_optional_system_time, serialize_optional_system_time,
    deserialize_system_time, serialize_system_time,
};

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
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<String>
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
    #[serde(rename = "verifiableCredential", skip_serializing_if = "Vec::is_empty")]
    pub verifiable_credential: Vec<String>,
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

        let verifiable_presentation = VerifiablePresentation{
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

pub fn sign_presentation_with_signer(
    vp: &VerifiablePresentation,
    key_id: &str,
    signer: Arc<dyn Signer>,
) -> Result<String> {
    let mut payload = JwtPayload::new();
    let vp_claim = JwtPayloadVerifiablePresentation {
        context: vp.context.clone(),
        id: Some(vp.id.clone()),
        r#type: vp.r#type.clone(),
        holder: Some(vp.holder.clone()),
        verifiable_credential: vp.verifiable_credential.clone(),
        issuance_date: Some(vp.issuance_date),
        expiration_date: vp.expiration_date,
    };

    payload
        .set_claim("vp", Some(serde_json::to_value(vp_claim)?))
        .map_err(|e| Web5Error::Unknown(format!("failed to set claim {}", e)))?;
    payload.set_issuer(vp.holder.to_string());
    payload.set_jwt_id(&vp.id);
    payload.set_not_before(&vp.issuance_date);
    payload.set_issued_at(&SystemTime::now());
    if let Some(exp) = &vp.expiration_date {
        payload.set_expires_at(exp);
    }

    let jose_signer = JoseSigner {
        kid: key_id.to_string(),
        signer,
    };

    let mut header = JwsHeader::new();
    header.set_token_type("JWT");
    let vp_jwt = josekit::jwt::encode_with_signer(&payload, &header, &jose_signer)
        .map_err(|e| Web5Error::Crypto(format!("failed to sign vp-jwt {}", e)))?;

    Ok(vp_jwt)
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

    let verification_method_id = verification_method_id
        .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());

    let is_assertion_method = if let Some(assertion_methods) = &bearer_did.document.assertion_method
    {
        assertion_methods.contains(&verification_method_id)
    } else {
        false
    };

    if !is_assertion_method {
        return Err(Web5Error::Parameter(format!(
            "verification_method_id {} is not an assertion_method",
            verification_method_id
        )));
    }

    let signer = bearer_did.get_signer(&verification_method_id)?;
    sign_presentation_with_signer(vp, &verification_method_id, signer)
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
    let header = josekit::jwt::decode_header(vp_jwt)
        .map_err(|_| Web5Error::Parameter("failed to decode vp-jwt jose header".to_string()))?;

    let kid = header
        .claim("kid")
        .and_then(serde_json::Value::as_str)
        .ok_or(VerificationError::MissingKid)?;

    if kid.is_empty() {
        return Err(VerificationError::MissingKid.into());
    }

    let jwt_payload = if verify_signature {
        let did = Did::parse(kid)?;

        let resolution_result = ResolutionResult::resolve(&did.uri);
        if let Some(err) = resolution_result.resolution_metadata.error.clone() {
            return Err(err.into());
        }

        let public_key_jwk = resolution_result
            .document
            .ok_or(ResolutionMetadataError::InternalError)?
            .find_verification_method(FindVerificationMethodOptions {
                verification_method_id: Some(kid.to_string()),
            })?
            .public_key_jwk;

        let jose_verifier = &JoseVerifier {
            kid: kid.to_string(),
            verifier: Arc::new(Ed25519Verifier::new(public_key_jwk)),
        };

        let (jwt_payload, _) =
            josekit::jwt::decode_with_verifier(vp_jwt, jose_verifier).map_err(|e| {
                Web5Error::Crypto(format!("vp-jwt failed cryptographic verification {}", e))
            })?;

        jwt_payload
    } else {
        let (jwt_payload, _) = josekit::jwt::decode_with_verifier(
            vp_jwt,
            &JoseVerifierAlwaysTrue {
                kid: kid.to_string(),
            },
        )
            .map_err(|e| Web5Error::Crypto(format!("vp-jwt failed to decode payload {}", e)))?;

        jwt_payload
    };

    let vp_claim = jwt_payload
        .claim("vp")
        .ok_or(VerificationError::MissingClaim("vp".to_string()))?;
    let vp_payload = serde_json::from_value::<JwtPayloadVerifiablePresentation>(vp_claim.clone())?;

    // Registered claims checks
    let jti = jwt_payload
        .jwt_id()
        .ok_or(VerificationError::MissingClaim("jti".to_string()))?;
    let iss = jwt_payload
        .issuer()
        .ok_or(VerificationError::MissingClaim("issuer".to_string()))?;
    let nbf = jwt_payload
        .not_before()
        .ok_or(VerificationError::MissingClaim("not_before".to_string()))?;
    let exp = jwt_payload.expires_at();

    if let Some(id) = &vp_payload.id {
        if id != jti {
            return Err(VerificationError::ClaimMismatch("id".to_string()).into());
        }
    }

    let vp_issuer = vp_payload.holder.clone();
    if let Some(holder) = &vp_payload.holder {
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
        VerifiableCredential::from_vc_jwt(&vc_jwt, true)
            .map_err(|e| VerificationError::DataModelValidationError(format!("invalid vc_jwt: {}", e)))?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::credentials::{CredentialSubject, Issuer};
    use crate::dids::methods::did_jwk::DidJwk;
    use super::*;
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

        VerifiableCredential::create(Issuer::String(vc_issuer_uri.to_string()), credential_subject, None)
            .expect("Failed to create Verifiable Credential")
    }

    fn sign_verifiable_credential(vc: &VerifiableCredential, vc_issuer_did: &BearerDid) -> String {
        vc.sign(vc_issuer_did, None).expect("Failed to sign Verifiable Credential")
    }

    #[test]
    fn test_create_verifiable_presentation() {
        let (vc_issuer_did, vc_issuer_uri, _holder, holder_uri) = setup_vc_issuer_and_holder();

        let vc = create_verifiable_credential(&vc_issuer_uri);

        let vc_jwt = sign_verifiable_credential(&vc, &vc_issuer_did);

        let vp = VerifiablePresentation::create(
            holder_uri.clone(),
            vec![vc_jwt.clone()],
            None,
        ).expect("Failed to create Verifiable Presentation");

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
        ).expect("Failed to create Verifiable Presentation");

        let validation_result = validate_vp_data_model(&vp);

        match validation_result {
            Err(VerificationError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "presentation expired".to_string());
            }
            _ => panic!("Verifiable Presentation should be considered expired, but it passed validation"),
        }
    }

    #[test]
    fn test_verifiable_presentation_sign() {
        let (vc_issuer_did, vc_issuer_uri, holder, holder_uri) = setup_vc_issuer_and_holder();

        let vc = create_verifiable_credential(&vc_issuer_uri);

        let vc_jwt = sign_verifiable_credential(&vc, &vc_issuer_did);

        let vp = VerifiablePresentation::create(
            holder_uri.clone(),
            vec![vc_jwt.clone()],
            None,
        ).expect("Failed to create Verifiable Presentation");

        // Sign the Verifiable Presentation
        let vp_jwt = vp.sign(&holder, None).expect("Failed to sign Verifiable Presentation");

        // Decode the signed Verifiable Presentation JWT
        let decoded_vp = VerifiablePresentation::from_vp_jwt(&vp_jwt, true)
            .expect("Failed to decode signed Verifiable Presentation JWT");

        // Verify that the decoded Verifiable Presentation matches the original
        assert_eq!(decoded_vp.holder, vp.holder);
        assert_eq!(decoded_vp.context, vp.context);
        assert_eq!(decoded_vp.r#type, vp.r#type);
        assert_eq!(decoded_vp.verifiable_credential, vp.verifiable_credential);

        // Validate the signed Verifiable Presentation data model
        validate_vp_data_model(&decoded_vp).expect("Signed Verifiable Presentation data model validation failed");
    }
}
