use super::credential_subject::CredentialSubject;
use super::issuer::Issuer;

use crate::dids::{bearer_did::BearerDid, did::Did};
use crate::errors::{Result, Web5Error};
use crate::json::{FromJson, ToJson};
use crate::rfc3339::{
    deserialize_optional_system_time, deserialize_system_time, serialize_optional_system_time,
    serialize_system_time,
};

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

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
        super::from_vc_jwt::vc_from_vc_jwt(vc_jwt, verify)
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        super::sign::sign_with_did(self, bearer_did, verification_method_id)
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
        use crate::{credentials::issuer::ObjectIssuer, json::JsonObject};

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
}
