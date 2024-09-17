use super::{service::Service, verification_method::VerificationMethod};
use crate::{
    errors::{Result, Web5Error},
    json::{FromJson, ToJson},
};
use serde::{Deserialize, Serialize};

/// Represents a DID Document as per the [W3C DID Core specification](https://www.w3.org/TR/did-core/).
///
/// A DID Document provides a set of data that describes the DID subject, including public keys for
/// authentication and authorization, and services for communication or interaction with the DID subject.
/// The document is retrieved by resolving a DID URI and contains the mechanisms for verification and
/// key management.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Document {
    /// The DID URI representing the subject of the DID document.
    pub id: String,

    /// A list of URIs defining the schema version used in the document (optional).
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<String>>,

    /// A list of entities authorized to make changes to the DID document (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<String>>,

    /// A list of alternative identifiers for the DID subject (optional).
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,

    /// A list of cryptographic public keys for authentication and authorization.
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,

    /// Methods for authenticating the DID subject (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Vec<String>>,

    /// Methods for expressing claims, such as issuing Verifiable Credentials (optional).
    #[serde(rename = "assertionMethod", skip_serializing_if = "Option::is_none")]
    pub assertion_method: Option<Vec<String>>,

    /// Methods for establishing secure communication channels (optional).
    #[serde(rename = "keyAgreement", skip_serializing_if = "Option::is_none")]
    pub key_agreement: Option<Vec<String>>,

    /// Methods used by the DID subject to invoke cryptographic capabilities (optional).
    #[serde(
        rename = "capabilityInvocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub capability_invocation: Option<Vec<String>>,

    /// Methods used by the DID subject to delegate cryptographic capabilities (optional).
    #[serde(
        rename = "capabilityDelegation",
        skip_serializing_if = "Option::is_none"
    )]
    pub capability_delegation: Option<Vec<String>>,

    /// A list of services provided by the DID subject (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Service>>,
}

impl FromJson for Document {}
impl ToJson for Document {}

pub(crate) struct FindVerificationMethodOptions {
    pub verification_method_id: Option<String>,
}

impl Document {
    /// Finds a verification method in the DID document by its ID.
    ///
    /// This method searches the `verification_method` field for a method matching the given ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Contains the ID of the verification method to find.
    ///
    /// # Returns
    ///
    /// * `Result<VerificationMethod>` - The verification method if found, or an error if not found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let doc = Document {
    ///     id: "did:example:123".to_string(),
    ///     verification_method: vec![VerificationMethod {
    ///         id: "did:example:123#key-1".to_string(),
    ///         r#type: "JsonWebKey".to_string(),
    ///         controller: "did:example:123".to_string(),
    ///         public_key_jwk: jwk,
    ///     }],
    ///     ..Default::default()
    /// };
    /// let options = FindVerificationMethodOptions {
    ///     verification_method_id: Some("did:example:123#key-1".to_string()),
    /// };
    /// let vm = doc.find_verification_method(options)?;
    /// println!("Found verification method: {:?}", vm);
    /// ```
    pub(crate) fn find_verification_method(
        &self,
        options: FindVerificationMethodOptions,
    ) -> Result<VerificationMethod> {
        let verification_method_id = options.verification_method_id.unwrap_or_default();
        if verification_method_id.is_empty() {
            return Err(Web5Error::Parameter(
                "verification method id cannot be empty".to_string(),
            ));
        }

        for vm in &self.verification_method {
            if vm.id == verification_method_id {
                return Ok(vm.clone());
            }
        }

        Err(Web5Error::NotFound(
            "verification method not found".to_string(),
        ))
    }
}
