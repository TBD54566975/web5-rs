use super::CredentialError;
use crate::dids::data_model::document::FindVerificationMethodOptions;
use crate::errors::{Result, Web5Error};
use crate::json::{FromJson, JsonObject, ToJson};
use crate::rfc3339::{
    deserialize_optional_system_time, deserialize_system_time, serialize_optional_system_time,
    serialize_system_time,
};
use crate::{
    crypto::dsa::{ed25519::Ed25519Verifier, Signer, Verifier},
    dids::{
        bearer_did::BearerDid,
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
};
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
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
    time::SystemTime,
};
use uuid::Uuid;

pub const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ObjectIssuer {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: Option<JsonObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Issuer {
    String(String),
    Object(ObjectIssuer),
}

impl FromJson for Issuer {}
impl ToJson for Issuer {}

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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten)]
    pub additional_properties: Option<JsonObject>,
}

impl FromJson for CredentialSubject {}
impl ToJson for CredentialSubject {}

impl<I> From<I> for CredentialSubject
where
    I: Into<String>,
{
    fn from(s: I) -> Self {
        CredentialSubject {
            id: s.into(),
            ..Default::default()
        }
    }
}

impl Display for CredentialSubject {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
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
        options: Option<VerifiableCredentialCreateOptions>,
    ) -> Result<Self> {
        if issuer.to_string().is_empty() {
            return Err(Web5Error::Parameter(String::from(
                "issuer id must not be empty",
            )));
        }

        if let Issuer::Object(ref named_issuer) = issuer {
            if named_issuer.name.is_empty() {
                return Err(Web5Error::Parameter(String::from(
                    "named issuer name must not be empty",
                )));
            }
        }

        if credential_subject.to_string().is_empty() {
            return Err(Web5Error::Parameter(String::from(
                "subject id must not be empty",
            )));
        }

        if Did::parse(&issuer.to_string()).is_err() {
            return Err(Web5Error::Parameter(
                "issuer must be a valid DID URI".to_string(),
            ));
        }

        if Did::parse(&credential_subject.to_string()).is_err() {
            return Err(Web5Error::Parameter(
                "credential subject must be a valid DID URI".to_string(),
            ));
        }

        let options = options.unwrap_or_default();

        let context = {
            let mut contexts = options
                .context
                .unwrap_or_else(|| vec![BASE_CONTEXT.to_string()]);

            if !contexts.contains(&BASE_CONTEXT.to_string()) {
                contexts.insert(0, BASE_CONTEXT.to_string());
            }

            contexts
        };

        let r#type = {
            let mut types = options
                .r#type
                .unwrap_or_else(|| vec![BASE_TYPE.to_string()]);

            if !types.contains(&BASE_TYPE.to_string()) {
                types.insert(0, BASE_TYPE.to_string());
            }

            types
        };

        let id = options
            .id
            .unwrap_or_else(|| format!("urn:uuid:{}", Uuid::new_v4()));

        Ok(Self {
            context,
            id,
            r#type,
            issuer,
            issuance_date: options.issuance_date.unwrap_or_else(SystemTime::now),
            expiration_date: options.expiration_date,
            credential_subject,
        })
    }

    // this function currently only supports Ed25519
    pub fn from_vc_jwt(vc_jwt: &str, verify: bool) -> Result<Self> {
        let header = josekit::jwt::decode_header(vc_jwt)
            .map_err(|_| Web5Error::Parameter("failed to decode vc-jwt jose header".to_string()))?;

        let kid = header
            .claim("kid")
            .and_then(serde_json::Value::as_str)
            .ok_or(CredentialError::MissingKid)?;

        if kid.is_empty() {
            return Err(CredentialError::MissingKid.into());
        }

        let jwt_payload = if verify {
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

            let (jwt_payload, _) = josekit::jwt::decode_with_verifier(vc_jwt, jose_verifier)
                .map_err(|e| {
                    Web5Error::Crypto(format!("vc-jwt failed cryptographic verification {}", e))
                })?;

            jwt_payload
        } else {
            let (jwt_payload, _) = josekit::jwt::decode_with_verifier(
                vc_jwt,
                &JoseVerifierAlwaysTrue {
                    kid: kid.to_string(),
                },
            )
            .map_err(|e| Web5Error::Crypto(format!("vc-jwt failed to decode payload {}", e)))?;

            jwt_payload
        };

        let vc_claim = jwt_payload
            .claim("vc")
            .ok_or(CredentialError::MissingClaim("vc".to_string()))?;
        let vc_payload =
            serde_json::from_value::<JwtPayloadVerifiableCredential>(vc_claim.clone())?;

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

        if let Some(id) = &vc_payload.id {
            if id != jti {
                return Err(CredentialError::ClaimMismatch("id".to_string()).into());
            }
        }

        if let Some(issuer) = &vc_payload.issuer {
            let vc_issuer = issuer.to_string();
            if iss != vc_issuer {
                return Err(CredentialError::ClaimMismatch("issuer".to_string()).into());
            }
        }

        if let Some(credential_subject) = &vc_payload.credential_subject {
            if sub != credential_subject.id {
                return Err(CredentialError::ClaimMismatch("subject".to_string()).into());
            }
        }

        if let Some(vc_payload_expiration_date) = vc_payload.expiration_date {
            match exp {
                None => {
                    return Err(CredentialError::MisconfiguredExpirationDate(
                        "VC has expiration date but no exp in registered claims".to_string(),
                    )
                    .into());
                }
                Some(exp) => {
                    let difference = vc_payload_expiration_date
                        .duration_since(exp)
                        .unwrap_or_else(|_| {
                            exp.duration_since(vc_payload_expiration_date).unwrap()
                        });

                    if difference.as_secs() > 0 {
                        return Err(
                            CredentialError::ClaimMismatch("expiration_date".to_string()).into(),
                        );
                    }
                }
            }
        }

        let vc_issuer = vc_payload.issuer.unwrap_or(Issuer::String(iss.to_string()));

        let vc_credential_subject = vc_payload.credential_subject.unwrap_or(CredentialSubject {
            id: sub.to_string(),
            additional_properties: None,
        });

        let vc = VerifiableCredential {
            context: vc_payload.context,
            id: jti.to_string(),
            r#type: vc_payload.r#type,
            issuer: vc_issuer,
            issuance_date: nbf,
            expiration_date: exp,
            credential_subject: vc_credential_subject,
        };

        if verify {
            validate_vc_data_model(&vc)?;
        }

        Ok(vc)
    }

    fn sign_with_signer(&self, key_id: &str, signer: Arc<dyn Signer>) -> Result<String> {
        let mut payload = JwtPayload::new();
        let vc_claim = JwtPayloadVerifiableCredential {
            context: self.context.clone(),
            id: Some(self.id.clone()),
            r#type: self.r#type.clone(),
            issuer: Some(self.issuer.clone()),
            issuance_date: Some(self.issuance_date),
            expiration_date: self.expiration_date,
            credential_subject: Some(self.credential_subject.clone()),
        };
        payload
            .set_claim("vc", Some(serde_json::to_value(vc_claim)?))
            .map_err(|e| Web5Error::Unknown(format!("failed to set claim {}", e)))?;
        payload.set_issuer(self.issuer.to_string());
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
        let vc_jwt = josekit::jwt::encode_with_signer(&payload, &header, &jose_signer)
            .map_err(|e| Web5Error::Crypto(format!("failed to sign vc-jwt {}", e)))?;

        Ok(vc_jwt)
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        // contains() because the issuer may also include a DID Fragment
        if self.issuer.to_string() != bearer_did.did.uri.clone() {
            return Err(Web5Error::Parameter(format!(
                "bearer_did uri {} does not match issuer {}",
                bearer_did.did.uri.clone(),
                self.issuer
            )));
        }

        let verification_method_id = verification_method_id
            .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());

        let is_assertion_method =
            if let Some(assertion_methods) = &bearer_did.document.assertion_method {
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
        self.sign_with_signer(&verification_method_id, signer)
    }
}

fn validate_vc_data_model(vc: &VerifiableCredential) -> Result<()> {
    // Required fields ["@context", "id", "type", "issuer", "issuanceDate", "credentialSubject"]
    if vc.id.is_empty() {
        return Err(CredentialError::DataModelValidationError("missing id".to_string()).into());
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(
            CredentialError::DataModelValidationError("missing context".to_string()).into(),
        );
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(CredentialError::DataModelValidationError("missing type".to_string()).into());
    }

    if vc.issuer.to_string().is_empty() {
        return Err(CredentialError::DataModelValidationError("missing issuer".to_string()).into());
    }

    if vc.credential_subject.id.is_empty() {
        return Err(CredentialError::DataModelValidationError(
            "missing credential subject".to_string(),
        )
        .into());
    }

    let now = SystemTime::now();
    if vc.issuance_date > now {
        return Err(CredentialError::DataModelValidationError(
            "issuance date in future".to_string(),
        )
        .into());
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = &vc.expiration_date {
        if expiration_date < &now {
            return Err(CredentialError::DataModelValidationError(
                "credential expired".to_string(),
            )
            .into());
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JwtPayloadVerifiableCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    r#type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<Issuer>,
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_optional_system_time",
        deserialize_with = "deserialize_optional_system_time"
    )]
    issuance_date: Option<SystemTime>,
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_system_time",
        deserialize_with = "deserialize_optional_system_time"
    )]
    expiration_date: Option<SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "credentialSubject")]
    credential_subject: Option<CredentialSubject>,
}

#[derive(Clone)]
struct JoseSigner {
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
        self.verifier
            .verify(message, signature)
            .map_err(|e| JosekitError::InvalidSignature(e.into()))
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

#[derive(Clone)]
struct JoseVerifierAlwaysTrue {
    pub kid: String,
}
impl JosekitJwsVerifier for JoseVerifierAlwaysTrue {
    fn algorithm(&self) -> &dyn JosekitJwsAlgorithm {
        &JosekitEddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.kid.as_str())
    }

    fn verify(&self, _message: &[u8], _signature: &[u8]) -> core::result::Result<(), JosekitError> {
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn JosekitJwsVerifier> {
        Box::new(self.clone())
    }
}
impl core::fmt::Debug for JoseVerifierAlwaysTrue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier").field("kid", &"").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::JsonValue;
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::HashMap;

    const ISSUER_DID_URI: &str = "did:web:tbd.website";
    const SUBJECT_DID_URI: &str = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";

    fn issuer() -> Issuer {
        Issuer::from(ISSUER_DID_URI)
    }
    fn credential_subject() -> CredentialSubject {
        CredentialSubject::from(SUBJECT_DID_URI)
    }

    mod create {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("verifiable_credential_1_1_create");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_default_context_added_if_not_supplied() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            assert_eq!(vc.context, vec![BASE_CONTEXT]);
        }

        #[test]
        fn test_default_context_not_duplicated_if_supplied() {
            TEST_SUITE.include(test_name!());

            let options = Some(VerifiableCredentialCreateOptions {
                context: Some(vec![BASE_CONTEXT.to_string()]),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.context, vec![BASE_CONTEXT]);
        }

        #[test]
        fn test_developer_provided_context_appended_to_default() {
            TEST_SUITE.include(test_name!());

            let custom_context = "https://example.com/custom-context";
            let options = Some(VerifiableCredentialCreateOptions {
                context: Some(vec![custom_context.to_string()]),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.context, vec![BASE_CONTEXT, custom_context]);
        }

        #[test]
        fn test_default_type_added_if_not_supplied() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            assert_eq!(vc.r#type, vec![BASE_TYPE]);
        }

        #[test]
        fn test_default_type_not_duplicated_if_supplied() {
            TEST_SUITE.include(test_name!());

            let options = Some(VerifiableCredentialCreateOptions {
                r#type: Some(vec![BASE_TYPE.to_string()]),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.r#type, vec![BASE_TYPE]);
        }

        #[test]
        fn test_developer_provided_type_appended_to_default() {
            TEST_SUITE.include(test_name!());

            let custom_type = "CustomType";
            let options = Some(VerifiableCredentialCreateOptions {
                r#type: Some(vec![custom_type.to_string()]),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.r#type, vec![BASE_TYPE, custom_type]);
        }

        #[test]
        fn test_id_generated_if_not_supplied() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            let uuid_regex = Regex::new(r"^urn:uuid:[0-9a-fA-F-]{36}$").unwrap();
            assert!(uuid_regex.is_match(&vc.id));
        }

        #[test]
        fn test_id_must_be_set_if_supplied() {
            TEST_SUITE.include(test_name!());

            let custom_id = "custom-id";
            let options = Some(VerifiableCredentialCreateOptions {
                id: Some(custom_id.to_string()),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.id, custom_id);
        }

        #[test]
        fn test_issuer_string_must_not_be_empty() {
            TEST_SUITE.include(test_name!());

            let empty_issuer = Issuer::from("");
            let result = VerifiableCredential::create(empty_issuer, credential_subject(), None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "issuer id must not be empty");
                }
                _ => panic!("Expected Web5Error::Parameter with specific error message"),
            };
        }

        #[test]
        fn test_issuer_string_must_be_set() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            assert_eq!(vc.issuer, issuer());
        }

        #[test]
        fn test_issuer_string_must_be_valid_did() {
            TEST_SUITE.include(test_name!());

            let result = VerifiableCredential::create(
                Issuer::String("did:invalid-123".to_string()),
                credential_subject(),
                None,
            );

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "issuer must be a valid DID URI")
                }
                _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
            };
        }

        #[test]
        fn test_issuer_object_id_must_not_be_empty() {
            TEST_SUITE.include(test_name!());

            let issuer = Issuer::Object(ObjectIssuer {
                id: "".to_string(),
                name: "Example Name".to_string(),
                additional_properties: None,
            });

            let result = VerifiableCredential::create(issuer, credential_subject(), None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "issuer id must not be empty");
                }
                _ => panic!("Expected Web5Error::Parameter with specific error message"),
            };
        }

        #[test]
        fn test_issuer_object_id_must_be_valid_did() {
            TEST_SUITE.include(test_name!());

            let result = VerifiableCredential::create(
                issuer(),
                CredentialSubject {
                    id: "did:something-invalid".to_string(),
                    ..Default::default()
                },
                None,
            );

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "credential subject must be a valid DID URI")
                }
                _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
            };
        }

        #[test]
        fn test_issuer_object_name_must_not_be_empty() {
            TEST_SUITE.include(test_name!());

            let issuer = Issuer::Object(ObjectIssuer {
                id: ISSUER_DID_URI.to_string(),
                name: "".to_string(),
                additional_properties: None,
            });

            let result = VerifiableCredential::create(issuer, credential_subject(), None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "named issuer name must not be empty");
                }
                _ => panic!("Expected Web5Error::Parameter with specific error message"),
            };
        }

        #[test]
        fn test_issuer_object_must_be_set() {
            TEST_SUITE.include(test_name!());

            let issuer = Issuer::Object(ObjectIssuer {
                id: ISSUER_DID_URI.to_string(),
                name: "Example Name".to_string(),
                additional_properties: None,
            });

            let vc =
                VerifiableCredential::create(issuer.clone(), credential_subject(), None).unwrap();

            assert_eq!(vc.issuer, issuer);
        }

        #[test]
        fn test_issuer_object_supports_additional_properties() {
            TEST_SUITE.include(test_name!());

            let additional_properties = JsonObject {
                properties: HashMap::from([(
                    "extra_key".to_string(),
                    JsonValue::String("extra_value".to_string()),
                )]),
            };

            let issuer = Issuer::Object(ObjectIssuer {
                id: ISSUER_DID_URI.to_string(),
                name: "Example Name".to_string(),
                additional_properties: Some(additional_properties.clone()),
            });

            let vc =
                VerifiableCredential::create(issuer.clone(), credential_subject(), None).unwrap();

            match vc.issuer {
                Issuer::Object(ref obj) => {
                    assert_eq!(obj.additional_properties, Some(additional_properties));
                }
                _ => panic!("Issuer is not an ObjectIssuer"),
            };
        }

        #[test]
        fn test_credential_subject_id_must_not_be_empty() {
            TEST_SUITE.include(test_name!());

            let credential_subject = CredentialSubject::from("");

            let result = VerifiableCredential::create(issuer(), credential_subject, None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "subject id must not be empty");
                }
                _ => panic!("Expected Web5Error::Parameter with specific error message"),
            };
        }

        #[test]
        fn test_credential_subject_must_be_set() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            assert_eq!(vc.credential_subject, credential_subject());
        }

        #[test]
        fn test_credential_subject_supports_additional_properties() {
            TEST_SUITE.include(test_name!());

            let additional_properties = JsonObject {
                properties: HashMap::from([(
                    "extra_key".to_string(),
                    JsonValue::String("extra_value".to_string()),
                )]),
            };

            let credential_subject = CredentialSubject {
                id: SUBJECT_DID_URI.to_string(),
                additional_properties: Some(additional_properties.clone()),
            };

            let vc =
                VerifiableCredential::create(issuer(), credential_subject.clone(), None).unwrap();

            assert_eq!(
                vc.credential_subject.additional_properties,
                Some(additional_properties)
            );
        }

        #[test]
        fn test_issuance_date_must_be_set() {
            TEST_SUITE.include(test_name!());

            let issuance_date = SystemTime::now();

            let options = Some(VerifiableCredentialCreateOptions {
                issuance_date: Some(issuance_date),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.issuance_date, issuance_date);
        }

        #[test]
        fn test_issuance_date_must_be_now_if_not_supplied() {
            TEST_SUITE.include(test_name!());

            let vc = VerifiableCredential::create(issuer(), credential_subject(), None).unwrap();

            let now = SystemTime::now();
            let hundred_millis_ago = now - std::time::Duration::from_millis(100);

            assert!(vc.issuance_date >= hundred_millis_ago && vc.issuance_date <= now);
        }

        #[test]
        fn test_expiration_date_must_be_set_if_supplied() {
            TEST_SUITE.include(test_name!());

            let expiration_date = SystemTime::now();
            let options = Some(VerifiableCredentialCreateOptions {
                expiration_date: Some(expiration_date),
                ..Default::default()
            });

            let vc = VerifiableCredential::create(issuer(), credential_subject(), options).unwrap();

            assert_eq!(vc.expiration_date, Some(expiration_date));
        }
    }

    mod from_vc_jwt {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("verifiable_credential_1_1_from_vc_jwt");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_missing_kid_jose_header() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_without_kid = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmEzYzY3NGI5LTliNGUtNGE2OS1hYzUwLWM3N2JhYzY0OTg2MCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKclZ6YzNjbnA2VTNGUExYSkxUekpIY1hwd04zRk9TRGhEYWtORmJFMHpabmhMUmtWcVdFTmtaRWxWSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yN1QyMDo0NzoyNS4xMTk2MjQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJWemMzY25wNlUzRlBMWEpMVHpKSGNYcHdOM0ZPU0RoRGFrTkZiRTB6Wm5oTFJrVnFXRU5rWkVsVkluMCIsImp0aSI6InVybjp1dWlkOmEzYzY3NGI5LTliNGUtNGE2OS1hYzUwLWM3N2JhYzY0OTg2MCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDc5MTY0NSwiaWF0IjoxNzI0NzkxNjQ1fQ.ocOyYhqFwz4Jvkdwpa69oFDXCOr2n-_IXSHg5elFebOM0T_lx3Cs6DgQJ7YLLk--mAOvPqrH05bh92BSaLB_DQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_without_kid, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingKid)
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_empty_kid_jose_header() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_kid = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IiJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKNE5XODFYM3BHTTE5a1QzaDBlbFZzTUVjNGREQnZkVlUyZFRsVFFVdEpiVkZXZFRaa1lsSlpiMUJqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yN1QyMDozODo0Ni45MjcxMzYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjROVzgxWDNwR00xOWtUM2gwZWxWc01FYzRkREJ2ZFZVMmRUbFRRVXRKYlZGV2RUWmtZbEpaYjFCakluMCIsImp0aSI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDc5MTEyNiwiaWF0IjoxNzI0NzkxMTI2fQ.0LzNrPzFY4CsEWRqYdo8pogGDonZqjRqfx9k30NEoWASw8pas6YC-mlDSAQ-4qQaE-otQ6p7zoMeopfw9M1CCQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_kid, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingKid)
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_kid_invalid_did_uri() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_invalid_did_uri = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImludmFsaWQgZGlkIHVyaSJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhYzFmN2M5LTIzOWQtNGE4OC05NDBiLTEwOTk3NmViNWYyNCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiaW52YWxpZCBkaWQgdXJpIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMjozMDowOC41OTAxOTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImludmFsaWQgZGlkIHVyaSIsImp0aSI6InVybjp1dWlkOmFhYzFmN2M5LTIzOWQtNGE4OC05NDBiLTEwOTk3NmViNWYyNCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg0ODIwOCwiaWF0IjoxNzI0ODQ4MjA4fQ.YdmnfP0wIK5HDu8Lft52UFdZCzfdFO0rclAOF-mWt6Y1vqAgoyuOn7AnX1Lx782-iWaekKApCGqCTaXepzj4CQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_did_uri, true);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(err_msg, "identifier regex match failure invalid did uri")
                }
                _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
            };
        }

        #[test]
        fn test_kid_fail_to_resolve_did() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_invalid_did_resolution = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6c29tZXRoaW5nLWludmFsaWQifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjhiNGM1NmI5LTM1ODgtNGM0Mi1iOTg3LWEwZTAxNDFmNzA2YSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDpzb21ldGhpbmctaW52YWxpZCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTI6MzQ6NDguMzMzMjg5KzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6ZGh0OnNvbWV0aGluZy1pbnZhbGlkIiwianRpIjoidXJuOnV1aWQ6OGI0YzU2YjktMzU4OC00YzQyLWI5ODctYTBlMDE0MWY3MDZhIiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODQ4NDg4LCJpYXQiOjE3MjQ4NDg0ODh9.hXbWLVU8ef38O5SY-HshVhXPM1RadFEAGRj0ds5Yjw1_lweWxe1-CNJxLmo0D4BiRCo4T4hCWP_bkwRoteImBA"#;

            let result =
                VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_did_resolution, true);

            match result {
                Err(Web5Error::Resolution(err)) => {
                    assert_eq!(err, ResolutionMetadataError::InvalidPublicKey)
                }
                _ => panic!("Expected Web5Error::Resolution, but got: {:?}", result),
            };
        }

        #[test]
        fn test_kid_missing_verification_method() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_vm = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmZNblZ0WDJFM2EwMU1OM1p0TXpGdlRWbGhjVE5WWWpGTWRWbzBhazFUTjNaT2NsTndlbWxvVWpkWkluMCMwLWludmFsaWQifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmM5ZjUzNTY0LTdkMjYtNGE1NS1iN2E4LTk2MTU4ZTBhNWVhNSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKZk1uVnRYMkUzYTAxTU4zWnRNekZ2VFZsaGNUTlZZakZNZFZvMGFrMVROM1pPY2xOd2VtbG9VamRaSW4wIzAtaW52YWxpZCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTI6NDA6NDIuMjk2Njc4KzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpmTW5WdFgyRTNhMDFNTjNadE16RnZUVmxoY1ROVllqRk1kVm8wYWsxVE4zWk9jbE53ZW1sb1VqZFpJbjAjMC1pbnZhbGlkIiwianRpIjoidXJuOnV1aWQ6YzlmNTM1NjQtN2QyNi00YTU1LWI3YTgtOTYxNThlMGE1ZWE1Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODQ4ODQyLCJpYXQiOjE3MjQ4NDg4NDJ9.g-KcBy9jJ87PvIZkBUDPkBVF-dlnSTsLUVxOxB4az5q64aIDFJNTffVETD3Cq0fjXKX3tZq3QpfzmNoiTo4xBQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_vm, true);

            match result {
                Err(Web5Error::NotFound(err_msg)) => {
                    assert_eq!(err_msg, "verification method not found")
                }
                _ => panic!("Expected Web5Error::NotFound, but got: {:?}", result),
            };
        }

        #[test]
        fn test_fails_cryptographic_verification() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_invalid_signature = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw-invalid-signature"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_signature, true);

            match result {
                Err(Web5Error::Crypto(err_msg)) => {
                    assert!(err_msg.contains("vc-jwt failed cryptographic verification"))
                }
                _ => panic!("Expected Web5Error::Crypto, but got: {:?}", result),
            };
        }

        #[test]
        fn test_passes_cryptographic_verification() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_valid = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw"#;

            let vc = VerifiableCredential::from_vc_jwt(vc_jwt_valid, true)
                .expect("vc_jwt should be valid");
            assert_eq!(
                "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJHVzZFTDlITTltdHlycGlYdFFUMGpxZk52aWNnQTlBVDg0MHY1Y08yb1RrIn0#0", 
                vc.issuer.to_string()
            );
        }

        #[test]
        fn test_can_skip_cryptographic_verification() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_invalid_signature = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw-invalid-signature"#;

            let vc = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_signature, false)
                .expect("vc_jwt should be valid");
            assert_eq!("urn:uuid:6fa4605c-aedb-4d64-977b-1fccfa55e35f", vc.id)
        }

        #[test]
        fn test_can_skip_data_model_validation() {
            TEST_SUITE.include(test_name!());

            // expired would throw an error, but since verify=false it doesn't
            let vc_jwt_with_expired = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"#;

            let vc = VerifiableCredential::from_vc_jwt(vc_jwt_with_expired, false)
                .expect("vc_jwt should be valid");
            assert_eq!("urn:uuid:ad70f7f6-a157-4f1f-af29-7cbbd44f89f0", vc.id)
        }

        #[test]
        fn test_issuer_string() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_issuer_as_string = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnlkMmhYU1VOWWNsSjNiMFphUm1SMU0wbHNOaTFCTkdVdGRqazNRbE14UmtaUmFWRTRhV05tV2t0ckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjc0NTY5ZmIzLWMyZTktNGZiMy1hOThkLWY3NGFjNzVjYTg5NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKeWQyaFhTVU5ZY2xKM2IwWmFSbVIxTTBsc05pMUJOR1V0ZGprM1FsTXhSa1pSYVZFNGFXTm1Xa3RySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxNjozNjoyOS4zNDc4ODArMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnlkMmhYU1VOWWNsSjNiMFphUm1SMU0wbHNOaTFCTkdVdGRqazNRbE14UmtaUmFWRTRhV05tV2t0ckluMCIsImp0aSI6InVybjp1dWlkOjc0NTY5ZmIzLWMyZTktNGZiMy1hOThkLWY3NGFjNzVjYTg5NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg2Mjk4OSwiaWF0IjoxNzI0ODYyOTg5fQ.0DSZ2XbPtjtrtxNKo3tImoByb1-jlQxZQN11lsngaFSe4lhy4mYmaxGAby4wIl-c_cLEkgBULfF3Qa_dlNSTCw"#;

            let vc = VerifiableCredential::from_vc_jwt(&vc_jwt_with_issuer_as_string, false)
                .expect("should be valid vc jwt");
            match vc.issuer {
                Issuer::String(issuer) => {
                    assert_eq!(
                        "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJyd2hXSUNYclJ3b0ZaRmR1M0lsNi1BNGUtdjk3QlMxRkZRaVE4aWNmWktrIn0",
                        issuer
                    )
                }
                Issuer::Object(_) => panic!("issuer should be string"),
            }
        }

        #[test]
        fn test_issuer_object() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_issuer_object = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTFVazF5YVVNMVZsaHVielpTVkRoTVdWVnJibnBKWm5OamFUUXlZbXhCYVdsTFdrcENaR2huVm5WQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjcwNWM0MTZiLTU1ODYtNDUzMS1hMmRmLWI3YzdhNTMxMGY5NiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMVVrMXlhVU0xVmxodWJ6WlNWRGhNV1ZWcmJucEpabk5qYVRReVlteEJhV2xMV2twQ1pHaG5WblZCSW4wIiwibmFtZSI6InNvbWUgbmFtZSJ9LCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDE2OjQwOjExLjUwNDIyMCswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMVVrMXlhVU0xVmxodWJ6WlNWRGhNV1ZWcmJucEpabk5qYVRReVlteEJhV2xMV2twQ1pHaG5WblZCSW4wIiwianRpIjoidXJuOnV1aWQ6NzA1YzQxNmItNTU4Ni00NTMxLWEyZGYtYjdjN2E1MzEwZjk2Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODYzMjExLCJpYXQiOjE3MjQ4NjMyMTF9.Mv-wlUcnj0w-OWuoMBCciaQXrAogXL3qqgZnthTRI9f55S5PidYiSapWFxFqc4SzxTVSpe64H2vF7kfGU-QpBw"#;

            let vc = VerifiableCredential::from_vc_jwt(&vc_jwt_with_issuer_object, false)
                .expect("should be valid vc jwt");
            match vc.issuer {
                Issuer::String(_) => panic!("issuer should be object"),
                Issuer::Object(issuer) => {
                    assert_eq!(
                        "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiI1Uk1yaUM1VlhubzZSVDhMWVVrbnpJZnNjaTQyYmxBaWlLWkpCZGhnVnVBIn0",
                        issuer.id
                    );
                    assert_eq!("some name", issuer.name)
                }
            }
        }

        #[test]
        fn test_missing_vc_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_vc_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlNSbkZSVlVWS1RFOVhlbXh3T1ZaRk1rdEtSalp6UjBwT00yVnpaWHBsY0hSSE0ySTFlbTh4YjAwNEluMCMwIn0.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpTUm5GUlZVVktURTlYZW14d09WWkZNa3RLUmpaelIwcE9NMlZ6WlhwbGNIUkhNMkkxZW04eGIwMDRJbjAiLCJqdGkiOiJ1cm46dXVpZDozNmU0ZjllNi0yYzdjLTQ0NGMtOTI4OS0zNDhmY2IxNDZlYjYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTA1MjIsImlhdCI6MTcyNDg1MDUyMn0.SqwZC0q9RuHp9hAtFmE6sBYeJ1uHuuq1hyijF0NmW9nksSBqtDpfNroNlitK_Tl-CLWtwbTpK3b3JduTfzGEAw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_vc_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingClaim("vc".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_missing_jti_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_jti_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjEwODM2MzgwLWI2MmMtNGVmZC04YmU0LTZhNzJiMDZjYWI4NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbmMyOVNkazFQWVcweWIyUlBOVjg1ZWpsTGVXbHNXVXMzVjNvdFlrWjBZbnB2VWtabWJVOVRNVEk0SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMDo1NS4yMDYwOTIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDY1NSwiaWF0IjoxNzI0ODUwNjU1fQ.1XDmdvB1GDsCHw9Qwp0HA5r8W-JnZB4lz9Yqo0C2V_EEe-uk88bQSl8P9HV8ViNyBC_YaYatLiPTD4jBZY77DA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_jti_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingClaim("jti".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_missing_issuer_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_iss_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjamREVWtVek1HbzNjVVU0Y2taVVJYQXdSbFJzYnpKVVVXVmlZa1ZHTVVvelJHaHRTVWhaVTNFd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNqZERVa1V6TUdvM2NVVTRja1pVUlhBd1JsUnNiekpVVVdWaVlrVkdNVW96UkdodFNVaFpVM0V3SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMTo1Mi4zMjg4MTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImp0aSI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDcxMiwiaWF0IjoxNzI0ODUwNzEyfQ.hwR6edt6ItlN0HHkDcxzhE3N5hLk-5-VYDLrqkalUoTKB41vsfaPvGnt_UQK3EAuekQgrTQ0SuCq-6ut0EdlBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_iss_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingClaim("issuer".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_missing_subject_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_sub_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKV2JEWmtOMUUxZFc5M1IxWTVNbGxGVVZKTE4ydE5aR1EzWVhwUmIwbGxTSFpzUVdoU1IxUmZOVEkwSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMjo0NS40NTg4MjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCIsImp0aSI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsIm5iZiI6MTcyNDg1MDc2NSwiaWF0IjoxNzI0ODUwNzY1fQ.61IFQhdASbbcYKUzMfhO7WPmikBd8AoE468FTlqRysxXck7kNa3bAAow3jK2uhYrIWLyRu3kuBp7JyYhLavjBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_sub_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingClaim("subject".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_missing_nbf_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_nbf_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd05HVnNkbEZ2UlZsMFlsQkhPRGxaVW1oalNHZElPVzAyUzNKNmJFWTJRakZSV2tkbE5HZEZSMkpqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMzoyNi4zMzQzNjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCIsImp0aSI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsImlhdCI6MTcyNDg1MDgwNn0.ZXfuZmvddH1nvmub8WDpQ2UEOhuiLaN6WL2q3XDhn0eouM_bNVa7vmCUCUZc3sfJ1YCtnAGCJOlJxSGnD3tOCw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_nbf_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::MissingClaim("not_before".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_claim_mismatch_id() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_id = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnZTWHBSUjJkTmNGTlNPSEpRWTNkd1IxZEJTRnBaV0hwUFdYRlRiMFkyTWtoM09HTlJRamRJUzIxM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InNvbWV0aGluZyBpbnZhbGlkIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUp2U1hwUlIyZE5jRk5TT0hKUVkzZHdSMWRCU0ZwWldIcFBXWEZUYjBZMk1raDNPR05SUWpkSVMyMTNJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE2OjAwLjcyMjgxOSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKdlNYcFJSMmROY0ZOU09ISlFZM2R3UjFkQlNGcFpXSHBQV1hGVGIwWTJNa2gzT0dOUlFqZElTMjEzSW4wIiwianRpIjoidXJuOnV1aWQ6ZGFkM2Y2MjktMzFiMS00NDcxLWFhYTMtMWE4MGZjN2I1YmU2Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUwOTYwLCJpYXQiOjE3MjQ4NTA5NjB9.P8-Z3KsMxIk7-Dz9a5odVhbGJZtWsWp4mDVYLlVxuZTNJl-Km-j2S1KusTjRTDkg1DqQoiVvp2Is0kr5WoAFBA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_id, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::ClaimMismatch("id".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_claim_mismatch_issuer() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_issuer = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXpWRVZsYWs0emIzSXpUbXR4WkZWVllYQjZaMVZ5TFcxblZFTkNkWEZRWVZkT1JWcE9lRXcwWkhRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiNzQzNWY0LWU0YjctNGQyZC1iN2M2LTVkOTE5ODRlNDlhOCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoic29tZXRoaW5nIGludmFsaWQiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE3OjQ1LjI4ODk2NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJelZFVmxhazR6YjNJelRtdHhaRlZWWVhCNloxVnlMVzFuVkVOQ2RYRlFZVmRPUlZwT2VFdzBaSFF3SW4wIiwianRpIjoidXJuOnV1aWQ6MmI3NDM1ZjQtZTRiNy00ZDJkLWI3YzYtNWQ5MTk4NGU0OWE4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMDY1LCJpYXQiOjE3MjQ4NTEwNjV9.x0UY38J4lEwmrXR4qrzhnk58btjZfMf8DVhdgBoj9M0JOgJqCDFCzwcS5weVCpNAv3gN72Qo32RH9Tx0eYyoDA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_issuer, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::ClaimMismatch("issuer".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_claim_mismatch_subject() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_subject = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdVRmh0UkVNMlNIWnVia1E0Vmw5QkxWbDVSelZ1TWtSa2IxQkdTVFkxY2tkb2MwVTVZWFZsWW5CckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjAwNDJiYTQ4LWU0ZGYtNGVhMS04ZmJjLWJjYmI4ODY3ZjFhMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxOToxMC4xNjM0ODkrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJzb21ldGhpbmcgaW52YWxpZCJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwianRpIjoidXJuOnV1aWQ6MDA0MmJhNDgtZTRkZi00ZWExLThmYmMtYmNiYjg4NjdmMWEwIiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMTUwLCJpYXQiOjE3MjQ4NTExNTB9.bAm9kKJX2-Rcw679VS7cUPbqg9awuq5Lwu9wiZoGcE0TCSc59rQTIP4nvxlP22o3V-VVs_DbfpJU-qB4duDSCA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_subject, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, CredentialError::ClaimMismatch("subject".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_claim_misconfigured_exp() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_misconfigured_exp = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKcmRYUjZXbVYzTURVMExWVTBRVEEzYWxjMlluSTFSVXg1TVRCWk5IbE9VMUJpWTI5M1dqQndOMlZqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyMzo0My45NDg4MzQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MjM6NDMuOTQ4NzYwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCIsImp0aSI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTQyMywiaWF0IjoxNzI0ODUxNDIzfQ.AWYyvLRISXwLH5gAXb5CcwBXNwaRKwacGqstXjnk-xIHx9gmm5xj8zGONvcKE2Xx0t9j3pNHicrhkp5wcOkABQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_misconfigured_exp, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::MisconfiguredExpirationDate(
                            "VC has expiration date but no exp in registered claims".to_string()
                        )
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_claim_mismatch_exp() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_exp = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKTWVYQmZSMlV6VURka2NtVmFhMlJXVWxOclluZE5WV2RxWTFRNGEwd3pVRVUyTUdzelkwaDNVMnRySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyNzozMy40Mjg1NjMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6Mjc6MzMuNDI4NDgyKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCIsImp0aSI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTY1MywiaWF0IjoxNzI0ODUxNjUzLCJleHAiOjE4ODUxMjM2NTN9.lAaTG8RhL2D92iNI6psZrv1uhtHYAO0m0AacGIQrW0XIThg-Livef36_CN9t4Lz2Ta5US2Be2VP6D3lCA-z1DQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_exp, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::ClaimMismatch("expiration_date".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_empty_id() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_id = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6IiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbGNrSlhOamhNWldaVVlsaERTRnByTlV4bk5VWXlTelJqUmtWaU1WaE1iVVZrVVU1bFNtUkpXamROSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozMDo1My44NDQ2ODMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MzA6NTMuODQ0NjMwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCIsImp0aSI6IiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTg1MywiaWF0IjoxNzI0ODUxODUzLCJleHAiOjE4ODIwOTk4NTN9.X_jkleLbhdAo0vm7KtN0qr6nR6hvWrXxk08UslfZAhZCkDN2kqLvWhoHps3GNznmGAuhJxwhZ0SN60OV7pp1DQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_id, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing id".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_empty_context() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjFSbXRGTVdkblNGcENaME4yY1doa1VqRkJZelZqUkd0Q2IybDFRMnhOYm5CTVVFNDRYM1ZOVjBRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6W10sImlkIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4IiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoxUm10Rk1XZG5TRnBDWjBOMmNXaGtVakZCWXpWalJHdENiMmwxUTJ4TmJuQk1VRTQ0WDNWTlYwUXdJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjMyOjM1LjI2MzM1NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKMVJtdEZNV2RuU0ZwQ1owTjJjV2hrVWpGQll6VmpSR3RDYjJsMVEyeE5ibkJNVUU0NFgzVk5WMFF3SW4wIiwianRpIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxOTU1LCJpYXQiOjE3MjQ4NTE5NTV9.2GaazffucPj-LfdnO9OtMwij0PQK9crDC7rMMcwV9nt50Q3ACc1UtYCruMWsfYMc_CKfl5g7m6-zwDW8SpDzAw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_context, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing context".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_context_base_mismatch() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_without_base_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJhIGNvbnRleHQiXSwiaWQiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzQ6MjcuODk4MDkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpxUlVkZlNVeE5WQzFVVDBoMk4yMXhSelJSVlZkTE1rNXVPRXBpVFVOTlpXSkNaV1Y0ZFRsSGJYVlpJbjAiLCJqdGkiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIwNjcsImlhdCI6MTcyNDg1MjA2N30.cgkGQF5CXqHw_C1KNaKLFeIzPzmBuWRzRk-7KgvEYc1jJzwoXoOWB6cn-8I3MjWAgd_NeM1Yt656lJ60gy0RAQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_without_base_context, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing context".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_empty_type() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTJSV2Q2T0RZeGExRjNVMTh3U25SWVVqQlJha3BtWldOemQyVkJiRWN3UzBadGMwZGxUa0ZoZVdwQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmNmNjQ1MTNiLTIwODQtNDliNC1iNWMzLTgxZTk1ODNjOTcyOCIsInR5cGUiOltdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUkyUldkNk9EWXhhMUYzVTE4d1NuUllVakJSYWtwbVpXTnpkMlZCYkVjd1MwWnRjMGRsVGtGaGVXcEJJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjM1OjM2LjkxMzQyNyswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMlJXZDZPRFl4YTFGM1UxOHdTblJZVWpCUmFrcG1aV056ZDJWQmJFY3dTMFp0YzBkbFRrRmhlV3BCSW4wIiwianRpIjoidXJuOnV1aWQ6Y2Y2NDUxM2ItMjA4NC00OWI0LWI1YzMtODFlOTU4M2M5NzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUyMTM2LCJpYXQiOjE3MjQ4NTIxMzZ9.EY1q2nZHnPk-hnzdScvf6QYA0ko_sshHWOnPxU9tkU-RhxdklRoO9JQgmoHZC1FdDgEfgs4nDFNUKyX-FlJPBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_type, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing type".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_type_base_mismatch() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_without_base_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmQ1NTdkODY3LWRlNTgtNDE3Ny1iZjE4LTM1ZjQ3NDA5NDlmMSIsInR5cGUiOlsiYSB0eXBlIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzY6MzUuNDgwMTkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpSUzI1clQzb3dkRE5KUVhkalVrRmpkMk4xZFVsSlJGbE9jR1pYZFhSb2NtUlRORFZLYnpkRVNXbHJJbjAiLCJqdGkiOiJ1cm46dXVpZDpkNTU3ZDg2Ny1kZTU4LTQxNzctYmYxOC0zNWY0NzQwOTQ5ZjEiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIxOTUsImlhdCI6MTcyNDg1MjE5NX0.S3vchUrNfdgXTQFeu7HcI5F0ZdkQdYkd4IqAXF8_uhcOe_sX9joDWspBSxwP3BY6ESCPIpJoms_dPIp01RWABA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_without_base_type, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing type".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_empty_issuer() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_issuer = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnRPSEZRTVhoR1p6RndZMlpqZUY5UWVrUnJOMjFPYVhoak9YQTFWamN4WlVZelYwRTViSGwzTWpsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozOToxNS42MjMzOTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6IiIsImp0aSI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjM1NSwiaWF0IjoxNzI0ODUyMzU1fQ.mRYZKF3qNz_Vyg8xpemuBOipGLOliYy9xJ6b9ZqcMNjZbb8GtEyiaBv8rgF2jqmHreRT71wHaT3P6mV9GsQOCA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_issuer, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("missing issuer".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_empty_subject() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_subject = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKek5qRXpTWGhsYVZaT1YxbzVjMFozU2t0TU5IQjZaR1pGVGxZdFZURnFiQzB5U0hwWldXaEJRbFYzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0MToxMC4xNzM2NzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiIifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCIsImp0aSI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInN1YiI6IiIsIm5iZiI6MTcyNDg1MjQ3MCwiaWF0IjoxNzI0ODUyNDcwfQ.Ek9NMfHyb8BzJ7GnV0JRQPVl-UQyMOCMZ2_ABMx9Cvh8d8T81wMjrYUPp6v57-veqKntYFO_WZciL2FC_VZWAw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_subject, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError(
                            "missing credential subject".to_string()
                        )
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_issuance_in_future() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_issuance_date_in_future = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKaGFrMU9OazVmTWs5VFZtWXlMV0Z3VWw5bVVtZHBUbU4xU1UxemFVYzNNWFozYVhWcFVIZ3hhMU5OSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyOS0wOC0yMlQxMzo0Mjo1Ni43OTA2OTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCIsImp0aSI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTg4MjEwMDU3NiwiaWF0IjoxNzI0ODUyNTc2fQ.QM4LHyJ8wW1_A0PcuhpsorI3FOA9NLX9-u7a6MkAMXrQoxwNFIfHZeHuwLGVBshmco2emUievVAfKWUQFpOvBQ"#;

            let result =
                VerifiableCredential::from_vc_jwt(vc_jwt_with_issuance_date_in_future, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError(
                            "issuance date in future".to_string()
                        )
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_validate_dm_credential_expired() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_expired = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_expired, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        CredentialError::DataModelValidationError("credential expired".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }
    }

    mod sign {
        use crate::dids::methods::did_jwk::DidJwk;

        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("verifiable_credential_1_1_sign");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_can_sign_then_verify() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                None,
            )
            .unwrap();

            let vc_jwt = vc
                .sign(&bearer_did, None)
                .expect("should be able to sign vc jwt");

            let vc_from_vc_jwt = VerifiableCredential::from_vc_jwt(&vc_jwt, true)
                .expect("should be able to verify the signed vc jwt");
            assert_eq!(vc.id, vc_from_vc_jwt.id)
        }

        #[test]
        fn test_bearer_did_mismatch_issuer() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                None,
            )
            .unwrap();

            let different_bearer_did = DidJwk::create(None).unwrap();
            let result = vc.sign(&different_bearer_did, None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(
                        err_msg,
                        format!(
                            "bearer_did uri {} does not match issuer {}",
                            different_bearer_did.did.uri, bearer_did.did.uri
                        )
                    )
                }
                _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
            };
        }

        #[test]
        fn test_defaults_to_first_vm() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                None,
            )
            .unwrap();

            let vc_jwt = vc
                .sign(&bearer_did, None)
                .expect("should sign with default vm");

            let header = josekit::jwt::decode_header(vc_jwt).unwrap();

            let kid = header
                .claim("kid")
                .and_then(serde_json::Value::as_str)
                .unwrap();

            assert_eq!(bearer_did.document.verification_method[0].id, kid)
        }

        #[test]
        fn test_vm_must_be_assertion_method() {
            TEST_SUITE.include(test_name!());

            let mut bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                None,
            )
            .unwrap();

            // remove the assertionMethod
            if let Some(assertion_method) = bearer_did.document.assertion_method.as_mut() {
                assertion_method.remove(0);
            }

            let vm_id = bearer_did.document.verification_method[0].id.clone();

            let result = vc.sign(&bearer_did, Some(vm_id.clone()));

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(
                        err_msg,
                        format!(
                            "verification_method_id {} is not an assertion_method",
                            vm_id
                        )
                    )
                }
                _ => panic!("expected Web5Error::Parameter but got {:?}", result),
            }
        }
    }
}
