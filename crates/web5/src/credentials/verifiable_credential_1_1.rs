use super::credential_schema::validate_credential_schema;
use super::credential_schema::CredentialSchema;
use super::data_model_validation::validate_vc_data_model;
use super::decode::decode;
use super::CredentialSubject;
use super::Issuer;
use crate::datetime::{
    deserialize_optional_rfc3339, deserialize_rfc3339, serialize_optional_rfc3339,
    serialize_rfc3339,
};
use crate::dids::bearer_did::BearerDid;
use crate::errors::Result;
use crate::json::JsonObject;
use crate::json::{FromJson, ToJson};

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const BASE_TYPE: &str = "VerifiableCredential";

/// Represents a Verifiable Credential according to the [W3C Verifiable Credentials Data Model v1.1](https://www.w3.org/TR/vc-data-model/)
/// and conformant to the [Web5 specification](https://tbd54566975.github.io/web5-spec/#verifiable-credentials-vcs).
/// A Verifiable Credential is a tamper-evident credential that has authorship that can be cryptographically verified.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    /// A list of contexts used to define the semantic meaning of the data contained in the Verifiable Credential.
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    /// The unique identifier for the Verifiable Credential.
    pub id: String,

    /// The type(s) of the Verifiable Credential.
    #[serde(rename = "type")]
    pub r#type: Vec<String>,

    /// The entity (either a string or an object) that issued the credential.
    pub issuer: Issuer,

    /// The subject of the credential, containing claims about the entity being described by the credential.
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,

    /// The date and time when the credential was issued.
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_rfc3339",
        deserialize_with = "deserialize_rfc3339"
    )]
    pub issuance_date: SystemTime,

    /// The optional expiration date and time after which the credential is no longer valid.
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_rfc3339",
        deserialize_with = "deserialize_optional_rfc3339",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<SystemTime>,

    /// The credential status information, if applicable (e.g., revoked or suspended).
    #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<CredentialStatus>,

    /// The credential schema, used to validate the data structure of the credential.
    #[serde(rename = "credentialSchema", skip_serializing_if = "Option::is_none")]
    pub credential_schema: Option<CredentialSchema>,

    /// An array of evidence supporting the claims made in the credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<JsonObject>>,
}

impl FromJson for VerifiableCredential {}
impl ToJson for VerifiableCredential {}

/// Represents the options available when creating a Verifiable Credential.
/// These options allow customization of various attributes of the credential during its creation.
#[derive(Default, Clone)]
pub struct VerifiableCredentialCreateOptions {
    /// The unique identifier for the Verifiable Credential. This is optional.
    /// If not provided then the default value will be of format urn:uuid:{uuid}.
    pub id: Option<String>,

    /// The context(s) for the Verifiable Credential, which define the meaning of terms within the credential.
    /// The base context `<https://www.w3.org/2018/credentials/v1>` is always the first value whereafter values provided here will be appended onto.
    /// If the base context is also provided here then it will be de-duplicated against the base context referred to above.
    pub context: Option<Vec<String>>,

    /// The type(s) of the Verifiable Credential.
    /// The base type VerifiableCredential will always be the first value whereafter values provided here will be appeneded onto.
    /// If the base type is also provided here then it will be de-duplicated against the base type referred to above.
    pub r#type: Option<Vec<String>>,

    /// The issuance date of the credential. If not provided, defaults to the current date and time.
    pub issuance_date: Option<SystemTime>,

    /// The optional expiration date for the credential, after which it is no longer valid.
    pub expiration_date: Option<SystemTime>,

    /// The optional credential status, which may indicate revocation or suspension information.
    pub credential_status: Option<CredentialStatus>,

    /// The credential schema, used to validate the data structure of the credential. This is optional.
    /// JSON Schema validation is performed if the value is provided, and creation will fail if validation fails.
    pub credential_schema: Option<CredentialSchema>,

    /// An optional array of evidence supporting the claims made in the credential.
    pub evidence: Option<Vec<JsonObject>>,
}

/// Represents the status information of a Verifiable Credential.
/// CredentialStatus is used to indicate the revocation or suspension status of a credential.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CredentialStatus {
    /// The unique identifier for the credential status.
    pub id: String,

    /// The type(s) of the credential status.
    #[serde(rename = "type")]
    pub r#type: String,

    /// The purpose of the status (e.g., "revocation" or "suspension").
    #[serde(rename = "statusPurpose")]
    pub status_purpose: String,

    /// The index in the status list indicating the credential's position.
    #[serde(rename = "statusListIndex")]
    pub status_list_index: String,

    /// The unique identifier for the Verifiable Credential that lists the status of the credential.
    #[serde(rename = "statusListCredential")]
    pub status_list_credential: String,
}

impl VerifiableCredential {
    /// Creates a new Verifiable Credential with the specified issuer, subject, and optional creation options.
    ///
    /// # Arguments
    ///
    /// * `issuer` - The entity issuing the credential. The `issuer` must be a valid DID.
    /// * `credential_subject` - The subject of the credential containing claims. The subject must be a valid DID.
    /// * `options` - Optional parameters for creating the credential, such as schema or status.
    ///
    /// # Example
    /// ```
    /// let issuer_bearer_did = DidJwk::create(None).unwrap();
    /// let subject_did_uri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy";
    ///
    /// let verifiable_credential = VerifiableCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     CredentialSubject {
    ///         id: subject_did_uri.to_string(),
    ///         additional_properties: None,
    ///     },
    ///     None,
    /// ).unwrap();
    /// ```
    pub fn create(
        issuer: Issuer,
        credential_subject: CredentialSubject,
        options: Option<VerifiableCredentialCreateOptions>,
    ) -> Result<Self> {
        super::create::create_vc(issuer, credential_subject, options)
    }

    /// Constructs a Verifiable Credential from a VC JWT (JSON Web Token).
    ///
    /// # Arguments
    ///
    /// * `vc_jwt` - The Verifiable Credential in JWT format, serialized as a compact JWS.
    /// * `verify` - If true, verifies the integrity of the JWT by performing cryptographic verification against the signature, validating the VC Data Model, and validates the JSON Schema if present.
    ///
    /// # Example
    /// ```
    /// let vc_jwt = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw"#;
    /// let verifiable_credential =
    ///     VerifiableCredential::from_vc_jwt(vc_jwt, true).unwrap();
    /// ```
    pub fn from_vc_jwt(vc_jwt: &str, verify: bool) -> Result<Self> {
        let verifiable_credential = decode(vc_jwt, verify)?;

        if verify {
            validate_vc_data_model(&verifiable_credential)?;
            validate_credential_schema(&verifiable_credential)?;
        }

        Ok(verifiable_credential)
    }

    /// Signs the Verifiable Credential using the specified Bearer DID and optional verification method.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID used to sign the credential.
    /// * `verification_method_id` - Optional identifier of the Verification Method for which to sign with.
    ///
    /// # Returns
    ///
    /// A string representing the signed JWT, serialized as a compact JWS, of the Verifiable Credential.
    ///
    /// # Example
    /// ```
    /// use web5::credentials::VerifiableCredential;
    /// use web5::credentials::CredentialSubject;
    /// use web5::credentials::Issuer;
    /// use web5::dids::methods::did_jwk::DidJwk;
    ///
    /// let issuer_bearer_did = DidJwk::create(None).unwrap();
    /// let subject_did_uri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy";
    ///
    /// let verifiable_credential = VerifiableCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     CredentialSubject {
    ///         id: subject_did_uri.to_string(),
    ///         additional_properties: None,
    ///     },
    ///     None,
    /// ).unwrap();
    ///
    /// let vc_jwt = verifiable_credential
    ///     .sign(&issuer_bearer_did, None)
    ///     .unwrap();
    /// ```
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

    mod from_vc_jwt {
        use super::*;
        use crate::credentials::credential_schema::CREDENTIAL_SCHEMA_TYPE;
        use crate::json::JsonValue;
        use crate::{credentials::VerificationError, errors::Web5Error};
        use crate::{
            dids::resolution::resolution_metadata::ResolutionMetadataError,
            test_helpers::UnitTestSuite, test_name,
        };
        use lazy_static::lazy_static;
        use mockito::{Server, ServerOpts};

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("verifiable_credential_1_1_from_vc_jwt");
        }

        fn assert_credential_error<T>(result: Result<T>, expected_error: VerificationError)
        where
            T: std::fmt::Debug,
        {
            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, expected_error, "Unexpected CredentialError variant");
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }
        fn mock_json_schema(url: String, schema_override: Option<String>) -> String {
            let schema = schema_override
                .unwrap_or_else(|| "https://json-schema.org/draft/2020-12/schema".to_string());

            format!(
                r###"
                    {{
                        "$id": "{url}/schemas/email.json",
                        "$schema": "{schema}",
                        "title": "EmailCredential",
                        "description": "EmailCredential using JsonSchema",
                        "type": "object",
                        "properties": {{
                            "credentialSubject": {{
                                "type": "object",
                                "properties": {{
                                    "emailAddress": {{
                                        "type": "string",
                                        "format": "email"
                                    }}
                                }},
                                "required": [
                                    "emailAddress"
                                ]
                            }}
                        }}
                    }}"###
            )
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

            assert_credential_error(result, VerificationError::MissingKid);
        }

        #[test]
        fn test_empty_kid_jose_header() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_kid = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IiJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKNE5XODFYM3BHTTE5a1QzaDBlbFZzTUVjNGREQnZkVlUyZFRsVFFVdEpiVkZXZFRaa1lsSlpiMUJqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yN1QyMDozODo0Ni45MjcxMzYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjROVzgxWDNwR00xOWtUM2gwZWxWc01FYzRkREJ2ZFZVMmRUbFRRVXRKYlZGV2RUWmtZbEpaYjFCakluMCIsImp0aSI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDc5MTEyNiwiaWF0IjoxNzI0NzkxMTI2fQ.0LzNrPzFY4CsEWRqYdo8pogGDonZqjRqfx9k30NEoWASw8pas6YC-mlDSAQ-4qQaE-otQ6p7zoMeopfw9M1CCQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_kid, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::MissingKid)
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
                Err(Web5Error::Resolution(err)) => {
                    assert_eq!(err, ResolutionMetadataError::InvalidDid)
                }
                _ => panic!("Expected Web5Error::Resolution, but got: {:?}", result),
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

            let vc_jwt_with_invalid_signature = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKMmJGOUNOVTB6UzFwclNXdDNTMDg1VGpKRlVFTTFjVE5IVGxoamQwWktObFl0VlU5RkxTMUlVa3MwSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoyYkY5Q05VMHpTMXByU1d0M1MwODVUakpGVUVNMWNUTkhUbGhqZDBaS05sWXRWVTlGTFMxSVVrczBJbjAiLCJqdGkiOiJ1cm46dXVpZDoyMWUxNWRjYi0xM2MzLTQwYTYtYWJiNS01NTA3Nzg5Zjk4YmEiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjU4OTU2NTYsImlhdCI6MTcyNTg5NTY1NiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoyYkY5Q05VMHpTMXByU1d0M1MwODVUakpGVUVNMWNUTkhUbGhqZDBaS05sWXRWVTlGTFMxSVVrczBJbjAiLCJpZCI6InVybjp1dWlkOjIxZTE1ZGNiLTEzYzMtNDBhNi1hYmI1LTU1MDc3ODlmOThiYSIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDktMDlUMTU6Mjc6MzZaIn19.6AR3aNzlMDgRpniSMhOGfXN3wUS0IkIoWa_KpZprOWwVbSyVjcI_Ndo3SGCutUSiBboYH9sFomdGb7_0AeVDCg"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_signature, true);

            match result {
                Err(Web5Error::Crypto(err_msg)) => {
                    assert_eq!("cryptographic verification failure", err_msg)
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
        fn test_can_skip_credential_schema_validation() {
            TEST_SUITE.include(test_name!());

            // expired would throw an error, but since verify=false it doesn't
            let vc_jwt_with_invalid_schema = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXlkMjFXVjFwblMySk1iM0JPVEhWTmRYUlNSV2gwTWtWMmJEbExkVkpFTFY5MlVrRnpWMlZwUm01TkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjE1OTY4MDA4LTI1OTEtNGY0MS1hOWI1LWU2YmUxNDU5ZDcxNiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJeWQyMVdWMXBuUzJKTWIzQk9USFZOZFhSU1JXaDBNa1YyYkRsTGRWSkVMVjkyVWtGelYyVnBSbTVOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0zMFQxNDo1NTozNS41MTc5NjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6ImludmFsaWQgdXJsL3NjaGVtYXMvZW1haWwuanNvbiIsInR5cGUiOiJKc29uU2NoZW1hIn19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUl5ZDIxV1YxcG5TMkpNYjNCT1RIVk5kWFJTUldoME1rVjJiRGxMZFZKRUxWOTJVa0Z6VjJWcFJtNU5JbjAiLCJqdGkiOiJ1cm46dXVpZDoxNTk2ODAwOC0yNTkxLTRmNDEtYTliNS1lNmJlMTQ1OWQ3MTYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjUwMjk3MzUsImlhdCI6MTcyNTAyOTczNX0.8zNS9RWIpvTlMvAzaI9dNSjKKM1drFig7bKhQeQ6Mv9hWXwazvDhxthy3D25EmITWAPiJfGcMPDqoDobETf8DA"#;

            let vc = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_schema, false)
                .expect("vc_jwt should be valid");
            assert_eq!("urn:uuid:15968008-2591-4f41-a9b5-e6be1459d716", vc.id)
        }

        #[test]
        fn test_decode_issuer_string() {
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
        fn test_decode_issuer_object() {
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
        fn test_decode_evidence() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_evidence = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmljbll0T1VKSGRUVlhNVFV4VkVKVk9GY3hVVkozU1dwSWRXVmlVVGc1TlRCQ2VuRTFjR1ZxV25wSkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjkxYWM1NzBmLTRjMDMtNDIxZi1iZGY4LWQ3Y2YyNzQ1YzVmNSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKaWNuWXRPVUpIZFRWWE1UVXhWRUpWT0ZjeFVWSjNTV3BJZFdWaVVUZzVOVEJDZW5FMWNHVnFXbnBKSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0zMFQxMDowMToyNC4yNTgzNjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiZXZpZGVuY2UiOlt7IkEgS2V5IjoiQSBWYWx1ZSJ9XX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmljbll0T1VKSGRUVlhNVFV4VkVKVk9GY3hVVkozU1dwSWRXVmlVVGc1TlRCQ2VuRTFjR1ZxV25wSkluMCIsImp0aSI6InVybjp1dWlkOjkxYWM1NzBmLTRjMDMtNDIxZi1iZGY4LWQ3Y2YyNzQ1YzVmNSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTAxMjA4NCwiaWF0IjoxNzI1MDEyMDg0fQ.M7t4Ox08v-rC-naPSfIqlE1KKhZ1nrx_QA2HbuW38AkgxnSZOYEpXEG1UTAzh6mdwKZin9jwoGrj29u24K1ABA"#;
            let vc = VerifiableCredential::from_vc_jwt(&vc_jwt_with_evidence, false)
                .expect("should be valid vc jwt");

            let mut evidence_item = JsonObject::new();
            evidence_item.insert_value("A Key", JsonValue::String("A Value".to_string()));
            let expected_evidence = vec![evidence_item];
            assert_eq!(Some(expected_evidence), vc.evidence);
        }

        #[test]
        fn test_decode_credential_schema() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_credential_schema = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTFUMXB2TVd4bU9VcHlOeTFZTkdGWU4yRmxka2N5WVU5MGEwWmxlSFZuYzBwcVZVbEtWVU5UYVVkUkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjNhMDU2NjE1LWNlZDMtNGQ4Zi05ODRhLTUwMzQ2Y2FlNDQ2ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMVQxcHZNV3htT1VweU55MVlOR0ZZTjJGbGRrY3lZVTkwYTBabGVIVm5jMHBxVlVsS1ZVTlRhVWRSSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0zMFQxNDo1OToyMi4xMDMzODUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHBzOi8vZXhhbXBsZS5jb20vc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTFUMXB2TVd4bU9VcHlOeTFZTkdGWU4yRmxka2N5WVU5MGEwWmxlSFZuYzBwcVZVbEtWVU5UYVVkUkluMCIsImp0aSI6InVybjp1dWlkOjNhMDU2NjE1LWNlZDMtNGQ4Zi05ODRhLTUwMzQ2Y2FlNDQ2ZiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTAyOTk2MiwiaWF0IjoxNzI1MDI5OTYyfQ.ZQkusfYLJSpfLVF9OuWrrhw8NdcBnjlalMFZbsfAxJp8i74KH47RkMsVVPadLPuKwbozgcDRCKPsokrl33TuCw"#;
            let vc = VerifiableCredential::from_vc_jwt(&vc_jwt_with_credential_schema, false)
                .expect("should be valid vc jwt");

            let credential_schema = CredentialSchema {
                id: "https://example.com/schemas/email.json".to_string(),
                r#type: CREDENTIAL_SCHEMA_TYPE.to_string(),
            };

            assert_eq!(Some(credential_schema), vc.credential_schema)
        }

        #[test]
        fn test_decode_missing_vc_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_vc_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlNSbkZSVlVWS1RFOVhlbXh3T1ZaRk1rdEtSalp6UjBwT00yVnpaWHBsY0hSSE0ySTFlbTh4YjAwNEluMCMwIn0.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpTUm5GUlZVVktURTlYZW14d09WWkZNa3RLUmpaelIwcE9NMlZ6WlhwbGNIUkhNMkkxZW04eGIwMDRJbjAiLCJqdGkiOiJ1cm46dXVpZDozNmU0ZjllNi0yYzdjLTQ0NGMtOTI4OS0zNDhmY2IxNDZlYjYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTA1MjIsImlhdCI6MTcyNDg1MDUyMn0.SqwZC0q9RuHp9hAtFmE6sBYeJ1uHuuq1hyijF0NmW9nksSBqtDpfNroNlitK_Tl-CLWtwbTpK3b3JduTfzGEAw"#;
            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_vc_claim, true);
            assert_credential_error(result, VerificationError::MissingClaim("vc".to_string()));
        }

        #[test]
        fn test_decode_missing_jti_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_jti_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjEwODM2MzgwLWI2MmMtNGVmZC04YmU0LTZhNzJiMDZjYWI4NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbmMyOVNkazFQWVcweWIyUlBOVjg1ZWpsTGVXbHNXVXMzVjNvdFlrWjBZbnB2VWtabWJVOVRNVEk0SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMDo1NS4yMDYwOTIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDY1NSwiaWF0IjoxNzI0ODUwNjU1fQ.1XDmdvB1GDsCHw9Qwp0HA5r8W-JnZB4lz9Yqo0C2V_EEe-uk88bQSl8P9HV8ViNyBC_YaYatLiPTD4jBZY77DA"#;
            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_jti_claim, true);
            assert_credential_error(result, VerificationError::MissingClaim("jti".to_string()));
        }

        #[test]
        fn test_decode_missing_issuer_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_iss_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjamREVWtVek1HbzNjVVU0Y2taVVJYQXdSbFJzYnpKVVVXVmlZa1ZHTVVvelJHaHRTVWhaVTNFd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNqZERVa1V6TUdvM2NVVTRja1pVUlhBd1JsUnNiekpVVVdWaVlrVkdNVW96UkdodFNVaFpVM0V3SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMTo1Mi4zMjg4MTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImp0aSI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDcxMiwiaWF0IjoxNzI0ODUwNzEyfQ.hwR6edt6ItlN0HHkDcxzhE3N5hLk-5-VYDLrqkalUoTKB41vsfaPvGnt_UQK3EAuekQgrTQ0SuCq-6ut0EdlBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_iss_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::MissingClaim("issuer".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_missing_subject_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_sub_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKV2JEWmtOMUUxZFc5M1IxWTVNbGxGVVZKTE4ydE5aR1EzWVhwUmIwbGxTSFpzUVdoU1IxUmZOVEkwSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMjo0NS40NTg4MjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCIsImp0aSI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsIm5iZiI6MTcyNDg1MDc2NSwiaWF0IjoxNzI0ODUwNzY1fQ.61IFQhdASbbcYKUzMfhO7WPmikBd8AoE468FTlqRysxXck7kNa3bAAow3jK2uhYrIWLyRu3kuBp7JyYhLavjBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_sub_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::MissingClaim("subject".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_missing_nbf_claim() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_missing_nbf_claim = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd05HVnNkbEZ2UlZsMFlsQkhPRGxaVW1oalNHZElPVzAyUzNKNmJFWTJRakZSV2tkbE5HZEZSMkpqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMzoyNi4zMzQzNjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCIsImp0aSI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsImlhdCI6MTcyNDg1MDgwNn0.ZXfuZmvddH1nvmub8WDpQ2UEOhuiLaN6WL2q3XDhn0eouM_bNVa7vmCUCUZc3sfJ1YCtnAGCJOlJxSGnD3tOCw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_missing_nbf_claim, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        VerificationError::MissingClaim("not_before".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_claim_mismatch_id() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_id = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnZTWHBSUjJkTmNGTlNPSEpRWTNkd1IxZEJTRnBaV0hwUFdYRlRiMFkyTWtoM09HTlJRamRJUzIxM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InNvbWV0aGluZyBpbnZhbGlkIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUp2U1hwUlIyZE5jRk5TT0hKUVkzZHdSMWRCU0ZwWldIcFBXWEZUYjBZMk1raDNPR05SUWpkSVMyMTNJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE2OjAwLjcyMjgxOSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKdlNYcFJSMmROY0ZOU09ISlFZM2R3UjFkQlNGcFpXSHBQV1hGVGIwWTJNa2gzT0dOUlFqZElTMjEzSW4wIiwianRpIjoidXJuOnV1aWQ6ZGFkM2Y2MjktMzFiMS00NDcxLWFhYTMtMWE4MGZjN2I1YmU2Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUwOTYwLCJpYXQiOjE3MjQ4NTA5NjB9.P8-Z3KsMxIk7-Dz9a5odVhbGJZtWsWp4mDVYLlVxuZTNJl-Km-j2S1KusTjRTDkg1DqQoiVvp2Is0kr5WoAFBA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_id, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::ClaimMismatch("id".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_claim_mismatch_issuer() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_issuer = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXpWRVZsYWs0emIzSXpUbXR4WkZWVllYQjZaMVZ5TFcxblZFTkNkWEZRWVZkT1JWcE9lRXcwWkhRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiNzQzNWY0LWU0YjctNGQyZC1iN2M2LTVkOTE5ODRlNDlhOCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoic29tZXRoaW5nIGludmFsaWQiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE3OjQ1LjI4ODk2NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJelZFVmxhazR6YjNJelRtdHhaRlZWWVhCNloxVnlMVzFuVkVOQ2RYRlFZVmRPUlZwT2VFdzBaSFF3SW4wIiwianRpIjoidXJuOnV1aWQ6MmI3NDM1ZjQtZTRiNy00ZDJkLWI3YzYtNWQ5MTk4NGU0OWE4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMDY1LCJpYXQiOjE3MjQ4NTEwNjV9.x0UY38J4lEwmrXR4qrzhnk58btjZfMf8DVhdgBoj9M0JOgJqCDFCzwcS5weVCpNAv3gN72Qo32RH9Tx0eYyoDA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_issuer, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::ClaimMismatch("issuer".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_claim_mismatch_subject() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_subject = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdVRmh0UkVNMlNIWnVia1E0Vmw5QkxWbDVSelZ1TWtSa2IxQkdTVFkxY2tkb2MwVTVZWFZsWW5CckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjAwNDJiYTQ4LWU0ZGYtNGVhMS04ZmJjLWJjYmI4ODY3ZjFhMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxOToxMC4xNjM0ODkrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJzb21ldGhpbmcgaW52YWxpZCJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwianRpIjoidXJuOnV1aWQ6MDA0MmJhNDgtZTRkZi00ZWExLThmYmMtYmNiYjg4NjdmMWEwIiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMTUwLCJpYXQiOjE3MjQ4NTExNTB9.bAm9kKJX2-Rcw679VS7cUPbqg9awuq5Lwu9wiZoGcE0TCSc59rQTIP4nvxlP22o3V-VVs_DbfpJU-qB4duDSCA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_subject, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(err, VerificationError::ClaimMismatch("subject".to_string()))
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_claim_misconfigured_exp() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_misconfigured_exp = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKcmRYUjZXbVYzTURVMExWVTBRVEEzYWxjMlluSTFSVXg1TVRCWk5IbE9VMUJpWTI5M1dqQndOMlZqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyMzo0My45NDg4MzQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MjM6NDMuOTQ4NzYwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCIsImp0aSI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTQyMywiaWF0IjoxNzI0ODUxNDIzfQ.AWYyvLRISXwLH5gAXb5CcwBXNwaRKwacGqstXjnk-xIHx9gmm5xj8zGONvcKE2Xx0t9j3pNHicrhkp5wcOkABQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_misconfigured_exp, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        VerificationError::MisconfiguredExpirationDate(
                            "VC has expiration date but no exp in registered claims".to_string()
                        )
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        #[test]
        fn test_decode_claim_mismatch_exp() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_mismatch_exp = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKTWVYQmZSMlV6VURka2NtVmFhMlJXVWxOclluZE5WV2RxWTFRNGEwd3pVRVUyTUdzelkwaDNVMnRySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyNzozMy40Mjg1NjMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6Mjc6MzMuNDI4NDgyKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCIsImp0aSI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTY1MywiaWF0IjoxNzI0ODUxNjUzLCJleHAiOjE4ODUxMjM2NTN9.lAaTG8RhL2D92iNI6psZrv1uhtHYAO0m0AacGIQrW0XIThg-Livef36_CN9t4Lz2Ta5US2Be2VP6D3lCA-z1DQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_mismatch_exp, true);

            match result {
                Err(Web5Error::CredentialError(err)) => {
                    assert_eq!(
                        err,
                        VerificationError::ClaimMismatch("expiration_date".to_string())
                    )
                }
                _ => panic!("Expected Web5Error::CredentialError, but got: {:?}", result),
            };
        }

        // --- Data Model Validation Tests ---

        #[test]
        fn test_validate_dm_empty_id() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_id = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6IiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbGNrSlhOamhNWldaVVlsaERTRnByTlV4bk5VWXlTelJqUmtWaU1WaE1iVVZrVVU1bFNtUkpXamROSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozMDo1My44NDQ2ODMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MzA6NTMuODQ0NjMwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCIsImp0aSI6IiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTg1MywiaWF0IjoxNzI0ODUxODUzLCJleHAiOjE4ODIwOTk4NTN9.X_jkleLbhdAo0vm7KtN0qr6nR6hvWrXxk08UslfZAhZCkDN2kqLvWhoHps3GNznmGAuhJxwhZ0SN60OV7pp1DQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_id, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing id")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_empty_context() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjFSbXRGTVdkblNGcENaME4yY1doa1VqRkJZelZqUkd0Q2IybDFRMnhOYm5CTVVFNDRYM1ZOVjBRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6W10sImlkIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4IiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoxUm10Rk1XZG5TRnBDWjBOMmNXaGtVakZCWXpWalJHdENiMmwxUTJ4TmJuQk1VRTQ0WDNWTlYwUXdJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjMyOjM1LjI2MzM1NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKMVJtdEZNV2RuU0ZwQ1owTjJjV2hrVWpGQll6VmpSR3RDYjJsMVEyeE5ibkJNVUU0NFgzVk5WMFF3SW4wIiwianRpIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxOTU1LCJpYXQiOjE3MjQ4NTE5NTV9.2GaazffucPj-LfdnO9OtMwij0PQK9crDC7rMMcwV9nt50Q3ACc1UtYCruMWsfYMc_CKfl5g7m6-zwDW8SpDzAw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_context, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing context")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_context_base_mismatch() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_without_base_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJhIGNvbnRleHQiXSwiaWQiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzQ6MjcuODk4MDkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpxUlVkZlNVeE5WQzFVVDBoMk4yMXhSelJSVlZkTE1rNXVPRXBpVFVOTlpXSkNaV1Y0ZFRsSGJYVlpJbjAiLCJqdGkiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIwNjcsImlhdCI6MTcyNDg1MjA2N30.cgkGQF5CXqHw_C1KNaKLFeIzPzmBuWRzRk-7KgvEYc1jJzwoXoOWB6cn-8I3MjWAgd_NeM1Yt656lJ60gy0RAQ"#;
            let result = VerifiableCredential::from_vc_jwt(vc_jwt_without_base_context, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing context")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_empty_type() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTJSV2Q2T0RZeGExRjNVMTh3U25SWVVqQlJha3BtWldOemQyVkJiRWN3UzBadGMwZGxUa0ZoZVdwQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmNmNjQ1MTNiLTIwODQtNDliNC1iNWMzLTgxZTk1ODNjOTcyOCIsInR5cGUiOltdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUkyUldkNk9EWXhhMUYzVTE4d1NuUllVakJSYWtwbVpXTnpkMlZCYkVjd1MwWnRjMGRsVGtGaGVXcEJJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjM1OjM2LjkxMzQyNyswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMlJXZDZPRFl4YTFGM1UxOHdTblJZVWpCUmFrcG1aV056ZDJWQmJFY3dTMFp0YzBkbFRrRmhlV3BCSW4wIiwianRpIjoidXJuOnV1aWQ6Y2Y2NDUxM2ItMjA4NC00OWI0LWI1YzMtODFlOTU4M2M5NzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUyMTM2LCJpYXQiOjE3MjQ4NTIxMzZ9.EY1q2nZHnPk-hnzdScvf6QYA0ko_sshHWOnPxU9tkU-RhxdklRoO9JQgmoHZC1FdDgEfgs4nDFNUKyX-FlJPBw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_type, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing type")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_type_base_mismatch() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_without_base_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmQ1NTdkODY3LWRlNTgtNDE3Ny1iZjE4LTM1ZjQ3NDA5NDlmMSIsInR5cGUiOlsiYSB0eXBlIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzY6MzUuNDgwMTkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpSUzI1clQzb3dkRE5KUVhkalVrRmpkMk4xZFVsSlJGbE9jR1pYZFhSb2NtUlRORFZLYnpkRVNXbHJJbjAiLCJqdGkiOiJ1cm46dXVpZDpkNTU3ZDg2Ny1kZTU4LTQxNzctYmYxOC0zNWY0NzQwOTQ5ZjEiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIxOTUsImlhdCI6MTcyNDg1MjE5NX0.S3vchUrNfdgXTQFeu7HcI5F0ZdkQdYkd4IqAXF8_uhcOe_sX9joDWspBSxwP3BY6ESCPIpJoms_dPIp01RWABA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_without_base_type, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing type")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_empty_issuer() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_issuer = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnRPSEZRTVhoR1p6RndZMlpqZUY5UWVrUnJOMjFPYVhoak9YQTFWamN4WlVZelYwRTViSGwzTWpsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozOToxNS42MjMzOTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6IiIsImp0aSI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjM1NSwiaWF0IjoxNzI0ODUyMzU1fQ.mRYZKF3qNz_Vyg8xpemuBOipGLOliYy9xJ6b9ZqcMNjZbb8GtEyiaBv8rgF2jqmHreRT71wHaT3P6mV9GsQOCA"#;

            let vc = decode(vc_jwt_with_empty_issuer, true).unwrap();
            let result = validate_vc_data_model(&vc);

            match result {
                Err(VerificationError::DataModelValidationError(msg)) => {
                    assert_eq!(msg, "missing issuer")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_empty_subject() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_empty_subject = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKek5qRXpTWGhsYVZaT1YxbzVjMFozU2t0TU5IQjZaR1pGVGxZdFZURnFiQzB5U0hwWldXaEJRbFYzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0MToxMC4xNzM2NzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiIifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCIsImp0aSI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInN1YiI6IiIsIm5iZiI6MTcyNDg1MjQ3MCwiaWF0IjoxNzI0ODUyNDcwfQ.Ek9NMfHyb8BzJ7GnV0JRQPVl-UQyMOCMZ2_ABMx9Cvh8d8T81wMjrYUPp6v57-veqKntYFO_WZciL2FC_VZWAw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_empty_subject, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "missing credential subject")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_issuance_in_future() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_issuance_date_in_future = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKaGFrMU9OazVmTWs5VFZtWXlMV0Z3VWw5bVVtZHBUbU4xU1UxemFVYzNNWFozYVhWcFVIZ3hhMU5OSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyOS0wOC0yMlQxMzo0Mjo1Ni43OTA2OTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCIsImp0aSI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTg4MjEwMDU3NiwiaWF0IjoxNzI0ODUyNTc2fQ.QM4LHyJ8wW1_A0PcuhpsorI3FOA9NLX9-u7a6MkAMXrQoxwNFIfHZeHuwLGVBshmco2emUievVAfKWUQFpOvBQ"#;

            let result =
                VerifiableCredential::from_vc_jwt(vc_jwt_with_issuance_date_in_future, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "issuance date in future")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_validate_dm_credential_expired() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_expired = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_expired, true);

            match result {
                Err(Web5Error::CredentialError(VerificationError::DataModelValidationError(
                    msg,
                ))) => {
                    assert_eq!(msg, "credential expired")
                }
                _ => panic!(
                    "Expected CredentialError::DataModelValidationError, but got: {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_schema_type_must_be_jsonschema() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_wrong_schema_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXhlbTlYY0VWTWN6TnhiV0p5VW5GblRFbzBjbDlCZUhCYVNFSmpjMUZJVGtSaVRGYzBOM1JmVGpkSkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjQ1NGY1NWJmLWYzMjAtNDQyOS1iNmViLTRkMzdlNTMzNDkwYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJeGVtOVhjRVZNY3pOeGJXSnlVbkZuVEVvMGNsOUJlSEJhU0VKamMxRklUa1JpVEZjME4zUmZUamRKSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0zMFQxNTowMjo1NC4zNjg1NjMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHBzOi8vZXhhbXBsZS5jb20iLCJ0eXBlIjoic29tZXRoaW5nIGludmFsaWFkIn19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUl4ZW05WGNFVk1jek54YldKeVVuRm5URW8wY2w5QmVIQmFTRUpqYzFGSVRrUmlURmMwTjNSZlRqZEpJbjAiLCJqdGkiOiJ1cm46dXVpZDo0NTRmNTViZi1mMzIwLTQ0MjktYjZlYi00ZDM3ZTUzMzQ5MGMiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjUwMzAxNzQsImlhdCI6MTcyNTAzMDE3NH0.8gxVx3_Qd1Lvao-y5PZ56XS3lMQvrFtBVMgfNDIdW9eoQkBQMNv79YKIxFCig0LHanzg_vyzX7tBviW6xJUuDw"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_wrong_schema_type, true);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(format!("type must be {}", CREDENTIAL_SCHEMA_TYPE), err_msg)
                }
                _ => panic!(
                    "expected Web5Error::Parameter with specific message but got {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_schema_resolve_network_issue() {
            TEST_SUITE.include(test_name!());

            let vc_jwt_with_invalid_url = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmZYelYxVEU1bWNVWTRRbTB6ZVhnMmJVRndMVlJJV25sSk5WcDJWQzFmYVVKbExWZDJiMHRuTTFwakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmRlNDY2N2YxLTMzM2ItNDg4OC1hMDc5LTdkMGU1N2JiZmFlZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKZlh6VjFURTVtY1VZNFFtMHplWGcyYlVGd0xWUklXbmxKTlZwMlZDMWZhVUpsTFZkMmIwdG5NMXBqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0zMFQxNTowNToyMC43NjQ0MDgrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6ImludmFsaWQgdXJsIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmZYelYxVEU1bWNVWTRRbTB6ZVhnMmJVRndMVlJJV25sSk5WcDJWQzFmYVVKbExWZDJiMHRuTTFwakluMCIsImp0aSI6InVybjp1dWlkOmRlNDY2N2YxLTMzM2ItNDg4OC1hMDc5LTdkMGU1N2JiZmFlZiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTAzMDMyMCwiaWF0IjoxNzI1MDMwMzIwfQ.3sH7qzI7QrQMdkWIvqf7k8Mr2dMGjWBLrv4QB8gEz0t83RSFMtG-fWT-YVkUlo1qMvC4gNjT2Jc0eObCAA7VDQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_with_invalid_url, true);

            match result {
                Err(Web5Error::Network(err_msg)) => {
                    assert!(err_msg.contains("unable to resolve json schema"))
                }
                _ => panic!(
                    "expected Web5Error::Network with specific message but got {:?}",
                    result
                ),
            };
        }

        #[test]
        fn test_schema_resolve_non_success() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40001,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(500) // here
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm9jMmhVZEU4M2F5MVNjVE5oVWtWR1lUVTRhUzFoWlZVdGRWaHNUVXB4UkdkallteEhhMTlpTW5OM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmE5Nzk1YTdmLTRmNzktNDU3OC1hYTkxLTcwYmYxM2YxZWVkNiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKb2MyaFVkRTgzYXkxU2NUTmhVa1ZHWVRVNGFTMWhaVlV0ZFZoc1RVcHhSR2RqWW14SGExOWlNbk4zSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo0MDowMS4wNDg1MzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDEvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm9jMmhVZEU4M2F5MVNjVE5oVWtWR1lUVTRhUzFoWlZVdGRWaHNUVXB4UkdkallteEhhMTlpTW5OM0luMCIsImp0aSI6InVybjp1dWlkOmE5Nzk1YTdmLTRmNzktNDU3OC1hYTkxLTcwYmYxM2YxZWVkNiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODAwMSwiaWF0IjoxNzI1Mzc4MDAxfQ.9739UnhTfGXr2tsbsjun7FQfFuXNtqmzfxhP_okbywVDoh6nsBGk8smLUU_D0VYwtiMBTo1ujDs1QtKPbCZDDA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert!(err_msg.contains("non-200 response when resolving json schema"))
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_resolve_invalid_response_body() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40002,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body("invalid response body") // here
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXhkek4zU25CUlZIVkNhRUZuV2tSd2JtbHZWME51ZW5kalp6bDBkMFZ4WVZGWldFUTVOblJXUTA1QkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmQ1NmYxMzRjLThjN2QtNDkyOC04OWYwLWQ5NWEzYjllZmU3YiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJeGR6TjNTbkJSVkhWQ2FFRm5Xa1J3Ym1sdlYwTnVlbmRqWnpsMGQwVnhZVkZaV0VRNU5uUldRMDVCSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo0Mzo1OC40MTE0NTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDIvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXhkek4zU25CUlZIVkNhRUZuV2tSd2JtbHZWME51ZW5kalp6bDBkMFZ4WVZGWldFUTVOblJXUTA1QkluMCIsImp0aSI6InVybjp1dWlkOmQ1NmYxMzRjLThjN2QtNDkyOC04OWYwLWQ5NWEzYjllZmU3YiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODIzOCwiaWF0IjoxNzI1Mzc4MjM4fQ.vYZ5YeXa4ZXaomhVp2obgJwlgjwScFctNAJBqTf2hJOUr1v-jN1C5huK4JL_e16_dRCJd_ysmiOpgFOJD2MOCQ"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert!(err_msg.contains("unable to parse json schema from response body"))
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_invalid_json_schema() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40003,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&mock_json_schema(
                    "http://localhost:40003/schemas/email.json".to_string(),
                    Some("this is not a valid $schema".to_string()), // here
                ))
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlRNemRDWVdaR01FTnRla2xmVTJ4WlEyTXlNSHBKZGt4eVprTnFjM1ptTUVWUWFtbDVkV2N5Wmt0WkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJjZWFmODUxLTBiYzktNDFkYy04NzNmLThhMGMyN2Y0ZDZkOCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKVE16ZENZV1pHTUVOdGVrbGZVMnhaUTJNeU1IcEpka3h5WmtOcWMzWm1NRVZRYW1sNWRXY3laa3RaSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo0NTozMC4zMDY4MDYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDMvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlRNemRDWVdaR01FTnRla2xmVTJ4WlEyTXlNSHBKZGt4eVprTnFjM1ptTUVWUWFtbDVkV2N5Wmt0WkluMCIsImp0aSI6InVybjp1dWlkOjJjZWFmODUxLTBiYzktNDFkYy04NzNmLThhMGMyN2Y0ZDZkOCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODMzMCwiaWF0IjoxNzI1Mzc4MzMwfQ.Drh8iEOdWWeL6l9KdmKMv9qfbBxWln-TW0KNwOJN3lZatyaSkwlBO_1o2FIWj0WIDiD_TP2EestMFazf4XFQDA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert!(err_msg.contains("unable to compile json schema"))
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_do_not_support_draft04() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40004,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&mock_json_schema(
                    "http://localhost:40004/schemas/email.json".to_string(),
                    Some("http://json-schema.org/draft-04/schema#".to_string()), // here
                ))
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnBNemMwTFVkc1lVSmZiMmxQZG1aR1ZteGtiVWhhVXpNNVpEZEtlalUxUm0xU2R6WkVjbmswVkRjd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmYzNDc3ZDdkLWJiOWUtNGI5Ny1iYjhkLTk4NDMwNjgzN2RmMyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKcE16YzBMVWRzWVVKZmIybFBkbVpHVm14a2JVaGFVek01WkRkS2VqVTFSbTFTZHpaRWNuazBWRGN3SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo0Nzo0MS4wNjgxNjIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDQvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnBNemMwTFVkc1lVSmZiMmxQZG1aR1ZteGtiVWhhVXpNNVpEZEtlalUxUm0xU2R6WkVjbmswVkRjd0luMCIsImp0aSI6InVybjp1dWlkOmYzNDc3ZDdkLWJiOWUtNGI5Ny1iYjhkLTk4NDMwNjgzN2RmMyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODQ2MSwiaWF0IjoxNzI1Mzc4NDYxfQ.X7pZBMqPeBO0oTq1QNtMcSrYcIpDxCavPEoPDiB1A9GOqCohx7KCgOerXaJGSyklAkmNJod7ssmL4DMM-l3uDA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert_eq!("draft unsupported Draft4", err_msg)
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_do_not_support_draft06() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40005,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&mock_json_schema(
                    "http://localhost:40005/schemas/email.json".to_string(),
                    Some("http://json-schema.org/draft-06/schema#".to_string()), // here
                ))
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhSemxDUVU1QlpXUkplR2RpYzJkVlprOWZRWFpPWVZsWmFHMWxZbmhXYWpOZlVuQnBWREp4ZG5WVkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjQ4MGM0ZjQ5LTAyMmEtNDIwMi1hYjFiLTc1ZThjZjQ1NDEyMyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFJ6bENRVTVCWldSSmVHZGljMmRWWms5ZlFYWk9ZVmxaYUcxbFluaFdhak5mVW5CcFZESnhkblZWSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo0ODo1MC4wNjM4NjIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDUvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhSemxDUVU1QlpXUkplR2RpYzJkVlprOWZRWFpPWVZsWmFHMWxZbmhXYWpOZlVuQnBWREp4ZG5WVkluMCIsImp0aSI6InVybjp1dWlkOjQ4MGM0ZjQ5LTAyMmEtNDIwMi1hYjFiLTc1ZThjZjQ1NDEyMyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODUzMCwiaWF0IjoxNzI1Mzc4NTMwfQ.Id6rsvMkqjocv6x5g_s8fnfR74HdXIpIdL3bMR33f1FYPFQ9CZRArMc1ZTh3xL3QfUggY8AUkRraQSAJh_onBA"#;

            let result = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert_eq!("draft unsupported Draft6", err_msg)
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_fails_validation() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40006,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&mock_json_schema(
                    "http://localhost/schemas/email.json".to_string(),
                    None,
                ))
                .create();

            let vc_jwt_at_port_with_invalid_schema = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnBlRkpyUVhodk5GTmtVMXAwTW1VMGFWQm5WRTV0T1dnelFYWnJYMU42Wm1WUlNVeFNkbFJHU0RSSkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmY4NThhZGM2LTZhMDQtNDY5ZC04MGJiLWM2MmI1N2MyNWI5NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKcGVGSnJRWGh2TkZOa1UxcDBNbVUwYVZCblZFNXRPV2d6UVhaclgxTjZabVZSU1V4U2RsUkdTRFJKSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo1MDozOS4yMTM3NzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDYvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnBlRkpyUVhodk5GTmtVMXAwTW1VMGFWQm5WRTV0T1dnelFYWnJYMU42Wm1WUlNVeFNkbFJHU0RSSkluMCIsImp0aSI6InVybjp1dWlkOmY4NThhZGM2LTZhMDQtNDY5ZC04MGJiLWM2MmI1N2MyNWI5NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODYzOSwiaWF0IjoxNzI1Mzc4NjM5fQ.zxz0OZO1umdlxgRyrwyMJis4t4esE_7Zo6nma4Q8T4josAkw6vnQJFI_cPoZV4usQ1vve5bB5OOiMcf_tca6Cw"#;

            let result =
                VerifiableCredential::from_vc_jwt(vc_jwt_at_port_with_invalid_schema, true);
            match result {
                Err(Web5Error::JsonSchema(err_msg)) => {
                    assert!(err_msg.contains("validation errors"))
                }
                _ => panic!(
                    "expected Web5Error::JsonSchema with specific message but got {:?}",
                    result
                ),
            }
        }

        #[test]
        fn test_schema_example_from_spec() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new_with_opts(ServerOpts {
                port: 40007,
                ..Default::default()
            });

            let _ = mock_server
                .mock("GET", "/schemas/email.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&mock_json_schema(
                    "http://localhost:40007/schemas/email.json".to_string(),
                    None,
                ))
                .create();

            let vc_jwt_at_port = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkZjVm96YVdGVldqWnpUMlZ5VEVGMWEzcEpRbkV6Ym1sNVEzZHVVWEF0WWtkNk1EQlZMVk15YURGakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjM4OWE2OWYyLWEwOTYtNDc0Ni1hNzU0LTRlNmE0ZThiZDEzYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKRmNWb3phV0ZWV2paelQyVnlURUYxYTNwSlFuRXpibWw1UTNkdVVYQXRZa2Q2TURCVkxWTXlhREZqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wM1QxNTo1MzoxNy43NjgzNzQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJlbWFpbEFkZHJlc3MiOiJhbGljZUB0YmQuZW1haWwifSwiY3JlZGVudGlhbFNjaGVtYSI6eyJpZCI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMDcvc2NoZW1hcy9lbWFpbC5qc29uIiwidHlwZSI6Ikpzb25TY2hlbWEifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkZjVm96YVdGVldqWnpUMlZ5VEVGMWEzcEpRbkV6Ym1sNVEzZHVVWEF0WWtkNk1EQlZMVk15YURGakluMCIsImp0aSI6InVybjp1dWlkOjM4OWE2OWYyLWEwOTYtNDc0Ni1hNzU0LTRlNmE0ZThiZDEzYyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNTM3ODc5NywiaWF0IjoxNzI1Mzc4Nzk3fQ.AC1OGOJ-MxfRsyNJZEQM4PW_t1eNCiSdTNtEtiPPOnIDGYnDl7JGtVIki9tHdWduQHoanrV0dWDeB5dnTTmZAw"#;

            let _ = VerifiableCredential::from_vc_jwt(vc_jwt_at_port, true).unwrap();
        }
    }
}
