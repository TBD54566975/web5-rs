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
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

pub const BASE_PRESENTATION_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const BASE_PRESENTATION_TYPE: &str = "VerifiablePresentation";

/// Represents a Verifiable Presentation according to the [W3C Verifiable Credentials Data Model v1.1](https://www.w3.org/TR/vc-data-model/#presentations-0)
/// and conformant to the [Web5 specification](https://tbd54566975.github.io/web5-spec/#verifiable-presentation-v11-data-model).
/// A Verifiable Presentation allows a holder to present one or more
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiablePresentation {
    /// A list of contexts used to define the semantic meaning of the data contained in the presentation.
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    /// The unique identifier for the Verifiable Presentation.
    pub id: String,

    /// The type(s) of the Verifiable Presentation.
    #[serde(rename = "type")]
    pub r#type: Vec<String>,

    /// The holder of the Verifiable Presentation, identified by a DID or other identifier.
    pub holder: String,

    /// The date and time when the presentation was issued.
    #[serde(
        rename = "issuanceDate",
        serialize_with = "serialize_rfc3339",
        deserialize_with = "deserialize_rfc3339"
    )]
    pub issuance_date: SystemTime,

    /// The optional expiration date and time after which the presentation is no longer valid.
    #[serde(
        rename = "expirationDate",
        serialize_with = "serialize_optional_rfc3339",
        deserialize_with = "deserialize_optional_rfc3339",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expiration_date: Option<SystemTime>,

    /// A list of Verifiable Credentials contained within the presentation.
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<String>,

    /// Additional data that may be included in the presentation, represented as a key-value map.
    #[serde(flatten)]
    pub additional_data: Option<HashMap<String, Value>>,
}

/// Represents the options available when creating a Verifiable Presentation.
/// These options allow customization of various attributes of the presentation during its creation.
#[derive(Default, Clone)]
pub struct VerifiablePresentationCreateOptions {
    /// The unique identifier for the Verifiable Presentation. This is optional.
    /// If not provided then the default value will be of format urn:uuid:{uuid}.
    pub id: Option<String>,

    /// The context(s) for the Verifiable Presentation, which define the meaning of terms within the presentation.
    /// The base context `<https://www.w3.org/2018/credentials/v1>` is always the first value whereafter values provided here will be appended onto.
    /// If the base context is also provided here then it will be de-duplicated against the base context referred to above.
    pub context: Option<Vec<String>>,

    /// The type(s) of the Verifiable Presentation.
    /// The base type VerifiablePresentation will always be the first value whereafter values provided here will be appeneded onto.
    /// If the base type is also provided here then it will be de-duplicated against the base type referred to above.
    pub r#type: Option<Vec<String>>,

    /// The issuance date of the presentation. If not provided, defaults to the current date and time.
    pub issuance_date: Option<SystemTime>,

    /// The optional expiration date for the presentation, after which it is no longer valid.
    pub expiration_date: Option<SystemTime>,

    /// Additional data that may be included in the presentation, represented as a key-value map.
    pub additional_data: Option<HashMap<String, Value>>,
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
    #[serde(flatten)]
    pub additional_data: Option<HashMap<String, Value>>,
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
        map.to_json_value()
    }
}

impl VerifiablePresentation {
    /// Creates a new Verifiable Presentation with the specified holder, Verifiable Credential JWTs,
    /// and optional creation options.
    ///
    /// # Arguments
    ///
    /// * `holder` - The entity holding and presenting the Verifiable Presentation. The holder must be a valid DID.
    /// * `vc_jwts` - A list of Verifiable Credential JWTs to include in the presentation. All `vc_jwt` values are verified, and the call will fail if any fail verification.
    /// * `options` - Optional parameters for creating the presentation, such as context or expiration.
    ///
    /// # Example
    /// ```rust
    /// use web5::credentials::VerifiablePresentation;
    /// use web5::dids::methods::did_jwk::DidJwk;
    ///
    /// let holder_bearer_did = DidJwk::create(None).unwrap();
    /// let vc_jwts = vec![String::from(
    ///     r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw"#
    /// )];
    /// let verifiable_presentation = VerifiablePresentation::create(
    ///     holder_bearer_did.did.uri.clone(),
    ///     vc_jwts,
    ///     None,
    /// ).unwrap();
    /// ```
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
            additional_data: options.additional_data,
        };

        Ok(verifiable_presentation)
    }

    /// Constructs a Verifiable Presentation from a VP JWT (JSON Web Token).
    ///
    /// # Arguments
    ///
    /// * `vp_jwt` - The Verifiable Presentation in JWT format, serialized as a compact JWS.
    /// * `verify` - If true, verifies the integrity of the JWT by performing cryptographic verification
    ///              against the signature and validating the Data Model.
    ///
    /// # Example
    /// ```rust
    /// use web5::credentials::VerifiablePresentation;
    ///
    /// let vp_jwt = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKYWNUaFJaR05XYlRrMlluZGpRa3R3WVhwV2RGQmlkekJ6U1c4NE0wbG9XRXAyVGtoV1VIUnpWWFYzSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUphY1RoUlpHTldiVGsyWW5kalFrdHdZWHBXZEZCaWR6QnpTVzg0TTBsb1dFcDJUa2hXVUhSelZYVjNJbjAiLCJqdGkiOiJ1cm46dXVpZDowZDg5YTcxMS0zNTdjLTQzNTQtOWYzMS02OWQ0NDE1NWQ1ZTMiLCJuYmYiOjE3MjYyMzQwODEsImlhdCI6MTcyNjIzNDA4MSwidnAiOnsiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMzoyODowMVoiLCJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVQcmVzZW50YXRpb24iXSwidmVyaWZpYWJsZUNyZWRlbnRpYWwiOlsiZXlKMGVYQWlPaUpLVjFRaUxDSmhiR2NpT2lKRlpESTFOVEU1SWl3aWEybGtJam9pWkdsa09tcDNhenBsZVVwb1lrZGphVTlwU2taYVJFa3hUbFJGTlVscGQybGhNMUkxU1dwdmFWUXdkRkZKYVhkcFdUTktNa2xxYjJsU1YxRjVUbFJWZUU5VFNYTkpibWRwVDJsS1VWRnNiRTVTYlRreFdUQnpOVk16WkZCVFNGSjZUbXBvVTA1RlZuZGpiVmw1VFhwT1RFNVVVazFOVmxaSlRqRlNTV05VVW1aaE1HaE9TVzR3SXpBaWZRLmV5SnBjM01pT2lKa2FXUTZhbmRyT21WNVNtaGlSMk5wVDJsS1JscEVTVEZPVkVVMVNXbDNhV0V6VWpWSmFtOXBWREIwVVVscGQybFpNMG95U1dwdmFWSlhVWGxPVkZWNFQxTkpjMGx1WjJsUGFVcFJVV3hzVGxKdE9URlpNSE0xVXpOa1VGTklVbnBPYW1oVFRrVldkMk50V1hsTmVrNU1UbFJTVFUxV1ZrbE9NVkpKWTFSU1ptRXdhRTVKYmpBaUxDSnFkR2tpT2lKMWNtNDZkWFZwWkRwaE1UaGlOREppWVMwMk1UVTVMVFExWVRrdFlXTXpZaTB5TnpaaVlqQmtORGRpWmpZaUxDSnpkV0lpT2lKa2FXUTZaR2gwT201bk5HaHRjWFJ5WjNWcWIzZzBZV2R3WmpodmEzaHBhRzU1ZVRGNmNXNXhPVGR4Wm1WeE1UVjRPRzloY2pkNVpYQjZhSGtpTENKdVltWWlPakUzTWpZeU16RTVOeklzSW1saGRDSTZNVGN5TmpJek1UazNNaXdpZG1NaU9uc2lRR052Ym5SbGVIUWlPbHNpYUhSMGNITTZMeTkzZDNjdWR6TXViM0puTHpJd01UZ3ZZM0psWkdWdWRHbGhiSE12ZGpFaVhTd2lZM0psWkdWdWRHbGhiRk4xWW1wbFkzUWlPbnNpYVdRaU9pSmthV1E2WkdoME9tNW5OR2h0Y1hSeVozVnFiM2cwWVdkd1pqaHZhM2hwYUc1NWVURjZjVzV4T1RkeFptVnhNVFY0T0c5aGNqZDVaWEI2YUhraWZTd2lhWE56ZFdWeUlqb2laR2xrT21wM2F6cGxlVXBvWWtkamFVOXBTa1phUkVreFRsUkZOVWxwZDJsaE0xSTFTV3B2YVZRd2RGRkphWGRwV1ROS01rbHFiMmxTVjFGNVRsUlZlRTlUU1hOSmJtZHBUMmxLVVZGc2JFNVNiVGt4V1RCek5WTXpaRkJUU0ZKNlRtcG9VMDVGVm5kamJWbDVUWHBPVEU1VVVrMU5WbFpKVGpGU1NXTlVVbVpoTUdoT1NXNHdJaXdpYVhOemRXRnVZMlZFWVhSbElqb2lNakF5TkMwd09TMHhNMVF4TWpvMU1qbzFNbG9pTENKMGVYQmxJanBiSWxabGNtbG1hV0ZpYkdWRGNtVmtaVzUwYVdGc0lsMHNJbWxrSWpvaWRYSnVPblYxYVdRNllURTRZalF5WW1FdE5qRTFPUzAwTldFNUxXRmpNMkl0TWpjMlltSXdaRFEzWW1ZMkluMTkuaUNkN1FsQWlCTkxDZnZ0VWJCdGstOVBUcUZmdWNxWjQ0S3hoRnZqR2NSU2prR0pyNjEwLTBqTFZzTlNBX0NQOGdibFljZncxZTVqeDNwR2VFckMtQnciXSwiaG9sZGVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKYWNUaFJaR05XYlRrMlluZGpRa3R3WVhwV2RGQmlkekJ6U1c4NE0wbG9XRXAyVGtoV1VIUnpWWFYzSW4wIiwiaWQiOiJ1cm46dXVpZDowZDg5YTcxMS0zNTdjLTQzNTQtOWYzMS02OWQ0NDE1NWQ1ZTMifX0.f-kdfbIIms3Gg2dMKUMayeU1rQnaO_o0io33kLzy-uPqI6vsdsJZvSmDIilx7scRqlia7Pmnnj6bnF2x8F2fAw"#;
    /// let verifiable_presentation = VerifiablePresentation::from_vp_jwt(vp_jwt, true).unwrap();
    /// ```
    pub fn from_vp_jwt(vp_jwt: &str, verify: bool) -> Result<Self> {
        let verifiable_presentation = decode_vp_jwt(vp_jwt, verify)?;

        if verify {
            validate_vp_data_model(&verifiable_presentation)?;
        }

        Ok(verifiable_presentation)
    }

    /// Signs the Verifiable Presentation using the specified Bearer DID and optional verification method.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID used to sign the presentation.
    /// * `verification_method_id` - Optional identifier of the Verification Method to sign with.
    ///
    /// # Returns
    ///
    /// A string representing the signed JWT, serialized as a compact JWS, of the Verifiable Presentation.
    ///
    /// # Example
    /// ```ignore
    /// let holder_bearer_did = DidJwk::create(None).unwrap();
    /// let vc_jwts = vec![String::from(
    ///     r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw"#
    /// )];
    /// let verifiable_presentation = VerifiablePresentation::create(
    ///     holder_bearer_did.did.uri.clone(),
    ///     vc_jwts,
    ///     None,
    /// ).unwrap();
    ///
    /// let vp_jwt = verifiable_presentation
    ///     .sign(&holder_bearer_did, None)
    ///     .unwrap();
    /// ```
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
        additional_data: vp.additional_data.clone(),
    };

    let mut additional_properties: HashMap<String, JsonValue> = HashMap::new();
    additional_properties.insert("vp".to_string(), vp_claims.to_json_value()?);

    let claims = JwtClaims {
        aud: None,
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
        additional_data: vp_payload.additional_data,
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
