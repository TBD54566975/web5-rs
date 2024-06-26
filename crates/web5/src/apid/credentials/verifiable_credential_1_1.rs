use super::{CredentialError, Result};
use crate::apid::{
    dids::{
        bearer_did::BearerDid,
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    dsa::{ed25519::Ed25519Verifier, DsaError, Signer, Verifier},
};
use chrono::{DateTime, TimeZone, Utc};
use core::fmt;
use josekit::{
    jws::{
        alg::eddsa::EddsaJwsAlgorithm as JosekitEddsaJwsAlgorithm,
        JwsAlgorithm as JosekitJwsAlgorithm, JwsHeader, JwsSigner as JosekitJwsSigner,
        JwsVerifier as JosekitJwsVerifier,
    },
    jwt::JwtPayload,
    JoseError as JosekitError,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
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

fn serialize_system_time<S>(
    time: &SystemTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: chrono::DateTime<Utc> = (*time).into();
    let s = datetime.to_rfc3339();
    serializer.serialize_str(&s)
}

fn deserialize_system_time<'de, D>(deserializer: D) -> std::result::Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = chrono::DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(datetime.with_timezone(&Utc).into())
}

fn serialize_option_system_time<S>(
    time: &Option<SystemTime>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        Some(time) => serialize_system_time(time, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_option_system_time<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<SystemTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let datetime = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
            Ok(Some(datetime.with_timezone(&Utc).into()))
        }
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: Issuer,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_system_time",
        deserialize_with = "deserialize_system_time"
    )]
    pub issuance_date: SystemTime,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_option_system_time",
        deserialize_with = "deserialize_option_system_time"
    )]
    pub expiration_date: Option<SystemTime>,
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
        issuer: Issuer,
        issuance_date: SystemTime,
        expiration_date: Option<SystemTime>,
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

    pub fn sign(&self, bearer_did: &BearerDid) -> Result<String> {
        // default to first VM
        let key_id = bearer_did.document.verification_method[0].id.clone();
        let signer = bearer_did.get_signer(key_id.clone())?;

        self.sign_with_signer(&key_id, signer)
    }

    pub fn sign_with_signer(&self, key_id: &str, signer: Arc<dyn Signer>) -> Result<String> {
        let mut payload = JwtPayload::new();
        let vc_value = serde_json::to_value(self)?;
        payload.set_claim("vc", Some(vc_value))?;
        payload.set_issuer(&self.issuer.to_string());
        payload.set_jwt_id(&self.id);
        payload.set_subject(&self.credential_subject.id);
        payload.set_not_before(&self.issuance_date);
        payload.set_issued_at(&SystemTime::now());
        if let Some(exp) = &self.expiration_date {
            payload.set_expires_at(exp)
        }

        let jose_signer = JoseSigner {
            kid: key_id.to_string(),
            signer,
        };

        let mut header = JwsHeader::new();
        header.set_token_type("JWT");
        let vc_jwt = josekit::jwt::encode_with_signer(&payload, &header, &jose_signer)?;

        Ok(vc_jwt)
    }

    pub fn verify(vc_jwt: &str) -> Result<Self> {
        // this function currently only supports Ed25519
        let header = josekit::jwt::decode_header(vc_jwt)?;

        let kid = header
            .claim("kid")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| JosekitError::InvalidJwtFormat(CredentialError::MissingKid.into()))?
            .to_string();

        let did = Did::new(&kid)?;

        let resolution_result = ResolutionResult::new(&did.uri);
        if let Some(err) = resolution_result.resolution_metadata.error.clone() {
            return Err(CredentialError::Resolution(err));
        }

        let public_key_jwk = resolution_result
            .document
            .ok_or_else(|| {
                JosekitError::InvalidJwtFormat(ResolutionMetadataError::InternalError.into())
            })?
            .find_public_key_jwk(kid.to_string())?;

        let verifier = Ed25519Verifier::new(public_key_jwk);

        Self::verify_with_verifier(vc_jwt, Arc::new(verifier))
    }

    pub fn verify_with_verifier(vc_jwt: &str, verifier: Arc<dyn Verifier>) -> Result<Self> {
        let header = josekit::jwt::decode_header(vc_jwt)?;

        let kid = header
            .claim("kid")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| JosekitError::InvalidJwtFormat(CredentialError::MissingKid.into()))?
            .to_string();

        let jose_verifier = &JoseVerifier { kid, verifier };

        let (jwt_payload, _) = josekit::jwt::decode_with_verifier(vc_jwt, jose_verifier)?;

        let vc_claim = jwt_payload
            .claim("vc")
            .ok_or(CredentialError::MissingClaim("vc".to_string()))?;
        let vc = serde_json::from_value::<Self>(vc_claim.clone())?;

        // registered claims checks
        let jti = jwt_payload
            .jwt_id()
            .ok_or(CredentialError::MissingClaim("jti".to_string()))?;
        let iss = jwt_payload
            .issuer()
            .ok_or(CredentialError::MissingClaim("issuer".to_string()))?;
        let sub = jwt_payload
            .subject()
            .ok_or(CredentialError::MissingClaim("subject".to_string()))?;
        let nbf = jwt_payload
            .not_before()
            .ok_or(CredentialError::MissingClaim("not_before".to_string()))?;
        let exp = jwt_payload.expires_at();

        if jti != vc.id {
            return Err(CredentialError::ClaimMismatch("id".to_string()));
        }

        if iss != vc.issuer.to_string() {
            return Err(CredentialError::ClaimMismatch("issuer".to_string()));
        }

        if sub != vc.credential_subject.id {
            return Err(CredentialError::ClaimMismatch("subject".to_string()));
        }

        // disregard nano-seconds which may be slightly different as a result of different time libraries
        if nbf.duration_since(UNIX_EPOCH)?.as_secs()
            != vc.issuance_date.duration_since(UNIX_EPOCH)?.as_secs()
        {
            return Err(CredentialError::ClaimMismatch("issuance_date".to_string()));
        }

        if let Some(exp) = exp {
            if SystemTime::now() > exp {
                return Err(CredentialError::CredentialExpired);
            }
        }

        // TODO prioritize JWT claims -- hit up Neal

        validate_vc_data_model(&vc)?;

        Ok(vc)
    }
}

fn validate_vc_data_model(vc: &VerifiableCredential) -> Result<()> {
    // Required fields ["@context", "id", "type", "issuer", "issuanceDate", "credentialSubject"]
    if vc.id.is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing id".to_string(),
        ));
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(CredentialError::VcDataModelValidationError(
            "missing context".to_string(),
        ));
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(CredentialError::VcDataModelValidationError(
            "missing type".to_string(),
        ));
    }

    if vc.issuer.to_string().is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing issuer".to_string(),
        ));
    }

    if vc.credential_subject.id.is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing credential subject".to_string(),
        ));
    }

    let now = SystemTime::now();
    if vc.issuance_date > now {
        return Err(CredentialError::VcDataModelValidationError(
            "issuance date in future".to_string(),
        ));
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = &vc.expiration_date {
        if expiration_date < &now {
            return Err(CredentialError::VcDataModelValidationError(
                "credential expired".to_string(),
            ));
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}

#[derive(Clone)]
pub struct JoseSigner {
    pub kid: String,
    pub signer: Arc<dyn Signer>,
}

impl JosekitJwsSigner for JoseSigner {
    fn algorithm(&self) -> &dyn JosekitJwsAlgorithm {
        &JosekitEddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(&self.kid)
    }

    fn signature_len(&self) -> usize {
        64
    }

    fn sign(&self, message: &[u8]) -> core::result::Result<Vec<u8>, JosekitError> {
        self.signer
            .sign(message)
            // 🚧 improve error message semantics
            .map_err(|err| JosekitError::InvalidSignature(err.into()))
    }

    fn box_clone(&self) -> Box<dyn JosekitJwsSigner> {
        Box::new(self.clone())
    }
}

impl core::fmt::Debug for JoseSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signer").field("kid", &self.kid).finish()
    }
}

#[derive(Clone)]
struct JoseVerifier {
    pub kid: String,
    pub verifier: Arc<dyn Verifier>,
}

impl JosekitJwsVerifier for JoseVerifier {
    fn algorithm(&self) -> &dyn JosekitJwsAlgorithm {
        &JosekitEddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.kid.as_str())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> core::result::Result<(), JosekitError> {
        let result = self
            .verifier
            .verify(message, signature)
            .map_err(|e| JosekitError::InvalidSignature(e.into()))?;

        match result {
            true => Ok(()),
            false => Err(JosekitError::InvalidSignature(
                // 🚧 improve error message semantics
                DsaError::VerificationFailure("ed25519 verification failed".to_string()).into(),
            )),
        }
    }

    fn box_clone(&self) -> Box<dyn JosekitJwsVerifier> {
        Box::new(self.clone())
    }
}

impl core::fmt::Debug for JoseVerifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier").field("kid", &self.kid).finish()
    }
}

pub fn timestamp_to_rfc3339(timestamp: i64) -> Result<String> {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| CredentialError::InvalidTimestamp("Invalid timestamp".to_string()))?;
    Ok(datetime.to_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apid::{
        crypto::key_managers::in_memory_key_manager::InMemoryKeyManager,
        dids::methods::did_jwk::DidJwk, dsa::ed25519::Ed25519Generator,
    };
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn can_create_sign_and_verify() {
        let key_manager = InMemoryKeyManager::new();
        let public_jwk = key_manager
            .import_private_jwk(Ed25519Generator::generate())
            .unwrap();
        let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();
        let bearer_did = BearerDid::new(&did_jwk.did.uri, Arc::new(key_manager)).unwrap();

        let now = SystemTime::now();
        let vc = VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_CONTEXT.to_string()],
            vec![BASE_TYPE.to_string()],
            Issuer::String(bearer_did.did.uri.clone()),
            now,
            Some(now + Duration::from_secs(20 * 365 * 24 * 60 * 60)), // now + 20 years
            CredentialSubject {
                id: bearer_did.did.uri.clone(),
                ..Default::default()
            },
        );

        let vc_jwt = vc.sign(&bearer_did).unwrap();
        assert_ne!(String::default(), vc_jwt);

        VerifiableCredential::verify(&vc_jwt).unwrap();
    }
}
