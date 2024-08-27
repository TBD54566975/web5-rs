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
    time::{SystemTime, UNIX_EPOCH},
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

    pub fn from_vc_jwt(vc_jwt: &str, verify: bool) -> Result<Self> {
        let header = josekit::jwt::decode_header(vc_jwt)
            .map_err(|_| Web5Error::Parameter("failed to decode vc-jwt jose header".to_string()))?;

        let kid = header
            .claim("kid")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| CredentialError::MissingKid)?;

        let did = Did::parse(&kid)?;

        let resolution_result = ResolutionResult::resolve(&did.uri);
        if let Some(err) = resolution_result.resolution_metadata.error.clone() {
            return Err(err.into());
        }

        let public_key_jwk = resolution_result
            .document
            .ok_or_else(|| ResolutionMetadataError::InternalError)?
            .find_verification_method(FindVerificationMethodOptions {
                verification_method_id: Some(kid.to_string()),
            })?
            .public_key_jwk;

        let jose_verifier = &JoseVerifier {
            kid: kid.to_string(),
            // this function currently only supports Ed25519
            verifier: Arc::new(Ed25519Verifier::new(public_key_jwk)),
        };

        let (jwt_payload, _) =
            josekit::jwt::decode_with_verifier(vc_jwt, jose_verifier).map_err(|e| {
                Web5Error::Crypto(format!("vc-jwt failed cryptographic verification {}", e))
            })?;

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

        let now = SystemTime::now();
        match vc_payload.expiration_date {
            Some(ref vc_payload_expiration_date) => match exp {
                None => {
                    return Err(CredentialError::MisconfiguredExpirationDate(
                        "VC has expiration date but no exp in registered claims".to_string(),
                    )
                    .into());
                }
                Some(exp) => {
                    if vc_payload_expiration_date
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| {
                            Web5Error::Unknown(format!("unknown system time error {}", e))
                        })?
                        .as_secs()
                        != exp
                            .duration_since(UNIX_EPOCH)
                            .map_err(|e| {
                                Web5Error::Unknown(format!("unknown system time error {}", e))
                            })?
                            .as_secs()
                    {
                        return Err(
                            CredentialError::ClaimMismatch("expiration_date".to_string()).into(),
                        );
                    }

                    if now > exp {
                        return Err(CredentialError::CredentialExpired.into());
                    }
                }
            },
            None => {
                if let Some(exp) = exp {
                    if now > exp {
                        return Err(CredentialError::CredentialExpired.into());
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

        validate_vc_data_model(&vc)?;

        Ok(vc)
    }

    pub fn sign(&self, key_id: &str, signer: Arc<dyn Signer>) -> Result<String> {
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

    pub fn sign_with_did(
        &self,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        let key_id = verification_method_id
            .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());
        let signer = bearer_did.get_signer(&key_id)?;
        self.sign(&key_id, signer)
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
}
