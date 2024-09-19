use std::collections::HashSet;

use crate::credentials::verifiable_credential_1_1::VerifiableCredential;
use jsonpath_rust::{
    JsonPathFinder,
    JsonPathValue::{NewValue, NoValue, Slice},
};
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};
use uuid::Uuid;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PexError {
    #[error("failed to parse json {0}")]
    JsonError(String),
    #[error("Invalid PEX state: {0}")]
    IllegalState(String),
}

type Result<T> = std::result::Result<T, PexError>;

/// Represents a DIF Presentation Definition defined [here](https://identity.foundation/presentation-exchange/#presentation-definition).
/// Presentation Definitions articulate what proofs a Verifier requires in the form of Input Descriptors and Submission Requirements.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PresentationDefinition {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,

    #[serde(rename = "inputDescriptors")]
    pub input_descriptors: Vec<InputDescriptor>,
    #[serde(rename = "submissionRequirements")]
    pub submission_requirements: Option<Vec<SubmissionRequirement>>,
}

/// Represents a DIF Input Descriptor defined [here](https://identity.foundation/presentation-exchange/#input-descriptor).
/// Input Descriptors describe the information a Verifier requires from a Holder.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputDescriptor {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub constraints: Constraints,
}

/// Contains the requirements for a given Input Descriptor.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Constraints {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
}

/// Contains the requirements for a given field within a proof.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Field {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Optionality>,
}

/// Type alias for the possible values of the predicate field.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Optionality {
    Required,
    Preferred,
}

/// A JSON Schema that is applied against the value of a field.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Filter {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Box<Filter>>,
}

/// Represents a DIF Submission Requirement as defined [here](https://identity.foundation/presentation-exchange/#submission-requirement).
/// Submission Requirements describe what is expected to fulfill the Presentation Definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SubmissionRequirement {
    pub rule: SubmissionRequirementRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromNested")]
    pub from_nested: Option<Vec<SubmissionRequirement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SubmissionRequirementRule {
    All,
    Pick,
}

/// Represents a presentation submission object as defined [here](https://identity.foundation/presentation-exchange/spec/v2.0.0/#presentation-submission).
/// This contains the result of fulfilling a Presentation Definition.
#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationSubmission {
    pub id: String,

    #[serde(rename = "definitionId")]
    pub definition_id: String,

    #[serde(rename = "descriptorMap")]
    pub descriptor_map: Vec<InputDescriptorMapping>,
}

/// Maps input descriptors to verifiable credentials in a presentation submission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputDescriptorMapping {
    pub id: String,
    pub format: String,
    pub path: String,

    #[serde(rename = "pathNested")]
    pub path_nested: Option<Box<InputDescriptorMapping>>,
}

/// Represents the result of a presentation submission process.
///
/// The `PresentationResult` contains the `PresentationSubmission` object, which maps input descriptors
/// to the selected Verifiable Credentials, and a list of `matchedVcJwts` which includes the JWTs of
/// the Verifiable Credentials that were used to fulfill the Presentation Definition.
#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationResult {
    /// The `PresentationSubmission` object that links input descriptors to the Verifiable Credentials (VCs).
    #[serde(rename = "presentationSubmission")]
    pub presentation_submission: PresentationSubmission,

    /// A list of JWT strings representing the Verifiable Credentials (VCs) that were selected.
    #[serde(rename = "matchedVcJwts")]
    pub matched_vc_jwts: Vec<String>,
}

fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

impl PresentationDefinition {
    /// Selects Verifiable Credentials (VCs) that match the input descriptors of the presentation definition.
    ///
    /// # Arguments
    ///
    /// * `vc_jwts` - A reference to a vector of VC JWTs to validate.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of VCs that fulfill the input descriptors. If no VCs match, returns an empty vector.
    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> Result<Vec<String>> {
        let mut matches: HashSet<String> = HashSet::new();

        for input_descriptor in &self.input_descriptors {
            let matching_vc_jwts = input_descriptor.select_credentials(vc_jwts)?;
            if matching_vc_jwts.is_empty() {
                return Ok(vec![]);
            }
            matches.extend(matching_vc_jwts);
        }

        Ok(matches.into_iter().collect())
    }

    /// Creates a Presentation Submission in which the list of Verifiable Credentials JWTs (VCs) fulfills the given Presentation Definition.
    ///
    /// # Arguments
    ///
    /// * `vc_jwts` - Iterable of VCs in JWT format to validate.
    /// * `presentation_definition` - The Presentation Definition V2 object against which VCs are validated.
    ///
    /// # Returns
    /// A `PresentationResult` which holds the `PresentationSubmission` and a `Vec<String>` which has the vc_jwts that were used
    /// A `PresentationSubmission` object.
    /// A `Vec<String>` which contains the chosen vc_jwts
    pub fn create_presentation_from_credentials(
        &self,
        vc_jwts: &Vec<String>,
    ) -> Result<PresentationResult> {
        // Check if there are submission requirements (not supported in this implementation)
        if self.submission_requirements.is_some() {
            return Err(PexError::IllegalState(
                "Submission requirements are not supported".to_string(),
            ));
        }

        // Select the appropriate credentials that match the presentation definition
        let selected_credentials = self.select_credentials(vc_jwts)?;

        if selected_credentials.is_empty() {
            return Err(PexError::IllegalState(
                "No VCs correspond to any input descriptor".to_string(),
            ));
        }

        let mut descriptor_map = Vec::new();
        let mut used_vc_jwts = Vec::new();

        for input_descriptor in &self.input_descriptors {
            let matching_vcs: Vec<&String> = selected_credentials
                .iter()
                .filter(|vc_jwt| {
                    input_descriptor
                        .select_credentials(&vec![vc_jwt.to_string()])
                        .map(|result| !result.is_empty())
                        .unwrap_or(false)
                })
                .collect();

            if matching_vcs.is_empty() {
                return Err(PexError::IllegalState(format!(
                    "No VC corresponds to input descriptor {}",
                    input_descriptor.id
                )));
            }

            // For simplicity, we're using the first matching VC for each input descriptor
            let vc_jwt = matching_vcs[0];
            let vc_index = vc_jwts
                .iter()
                .position(|jwt| jwt == vc_jwt)
                .ok_or_else(|| PexError::IllegalState("VC index not found".to_string()))?;

            descriptor_map.push(InputDescriptorMapping {
                id: input_descriptor.id.clone(),
                format: "jwt_vc".to_string(),
                path: format!("$.verifiableCredential[{}]", vc_index),
                path_nested: None,
            });

            // Add the selected vc_jwt to the used_vc_jwts list
            used_vc_jwts.push(vc_jwt.to_string());
        }

        if descriptor_map.len() < self.input_descriptors.len() {
            return Err(PexError::IllegalState(
                "The number of input descriptors matched is less than required".to_string(),
            ));
        }

        let presentation_submission = PresentationSubmission {
            id: Uuid::new_v4().to_string(),
            definition_id: self.id.clone(),
            descriptor_map,
        };

        Ok(PresentationResult {
            presentation_submission,
            matched_vc_jwts: used_vc_jwts,
        })
    }
}

struct TokenizedField<'a> {
    pub token: String,
    pub path: &'a String,
}

fn get_value_at_json_path(json: &str, path: &str) -> Option<Value> {
    let finder = JsonPathFinder::from_str(json, path).ok()?;
    let json_path_matches = finder.find_slice();
    let json_path_value = json_path_matches.first()?;

    let val = match json_path_value {
        Slice(val, _) => (*val).clone(),
        NewValue(val) => val.clone(),
        NoValue => return None,
    };

    Some(val)
}

impl InputDescriptor {
    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> Result<Vec<String>> {
        let mut tokenized_fields: Vec<TokenizedField> = vec![];
        let mut json_schema_builder = JsonSchemaBuilder::new();

        // Create a single JSON Schema from InputDescriptor and
        // generate tokens for each field.paths array
        for field in &self.constraints.fields {
            let token = generate_token();
            for path in &field.path {
                tokenized_fields.push(TokenizedField {
                    token: token.clone(),
                    path,
                });
            }

            // Add each field to "properties" of json schema, including filter if it is present
            match &field.filter {
                Some(filter) => {
                    let json_value = to_value(filter).map_err(|_| {
                        PexError::JsonError(format!(
                            "Failed to convert filter to json value: {:?}",
                            filter
                        ))
                    })?;
                    json_schema_builder.add_property(token, json_value)
                }
                None => json_schema_builder.add_property(token, Value::Object(Map::new())),
            }
        }

        let schema = JSONSchema::compile(&json_schema_builder.to_json()).map_err(|_| {
            PexError::JsonError(format!(
                "Failed to create json schema from {}",
                json_schema_builder.to_json()
            ))
        })?;

        // Validate each vc_jwt against the constructed json schema
        let mut selected_jwts: HashSet<String> = HashSet::new();
        for vc_jwt in vc_jwts {
            let mut selection_candidate: Map<String, Value> = Map::new();

            let vc = match VerifiableCredential::from_vc_jwt(vc_jwt, true) {
                Ok(vc) => vc,
                Err(_) => {
                    continue;
                }
            };

            let payload_json = match serde_json::to_string(&vc) {
                Ok(json) => json,
                Err(_) => {
                    continue;
                }
            };

            // Extract a value from the vc_jwt for each tokenized field
            for tokenized_field in &tokenized_fields {
                if selection_candidate.contains_key(&tokenized_field.token) {
                    continue;
                }

                if let Some(val) = get_value_at_json_path(&payload_json, tokenized_field.path) {
                    selection_candidate.insert(tokenized_field.token.clone(), val);
                }
            }

            let json_value = Value::from(selection_candidate);
            let validation_result = schema.validate(&json_value);
            if validation_result.is_ok() {
                selected_jwts.insert(vc_jwt.clone());
            }
        }

        Ok(selected_jwts.into_iter().collect())
    }
}

struct JsonSchemaBuilder {
    schema: String,
    r#type: String,
    properties: Map<String, Value>,
    required: Vec<String>,
}

impl JsonSchemaBuilder {
    pub fn new() -> Self {
        JsonSchemaBuilder {
            schema: "http://json-schema.org/draft-07/schema#".to_string(),
            r#type: "object".to_string(),
            properties: Map::new(),
            required: Vec::new(),
        }
    }

    pub fn add_property(&mut self, name: String, property: Value) {
        self.properties.insert(name.clone(), property);
        self.required.push(name);
    }

    pub fn to_json(&self) -> Value {
        json!({
            "$schema": self.schema,
            "type": self.r#type,
            "properties": self.properties,
            "required": self.required,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::{
        CredentialSubject, Issuer, VerifiablePresentation, VerifiablePresentationCreateOptions,
    };
    use crate::dids::methods::did_jwk::DidJwk;
    use crate::json::{JsonObject, JsonValue};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_create_presentation_from_credentials() {
        let issuer = DidJwk::create(None).unwrap();
        let issuer_uri = issuer.clone().did.uri;

        let presentation_definition = PresentationDefinition {
            id: "test_pd_id".to_string(),
            name: Some("Test Presentation Definition".to_string()),
            purpose: Some("Testing".to_string()),
            input_descriptors: vec![InputDescriptor {
                id: "test_input_1".to_string(),
                name: Some("Test Input 1".to_string()),
                purpose: Some("Testing Input 1".to_string()),
                constraints: Constraints {
                    fields: vec![Field {
                        path: vec!["$.credentialSubject.id".to_string()],
                        filter: Some(Filter {
                            r#type: Some("string".to_string()),
                            pattern: Some("^did:jwk:.*$".to_string()),
                            const_value: None,
                            contains: None,
                        }),
                        id: None,
                        name: None,
                        purpose: None,
                        optional: None,
                        predicate: None,
                    }],
                },
            }],
            submission_requirements: None,
        };

        let vc_1 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject::from(issuer_uri.clone()),
            Default::default(),
        )
        .unwrap();

        let signed_vcjwt_1 = vc_1.sign(&issuer.clone(), None).unwrap();

        let vc_jwts = vec![signed_vcjwt_1];

        let result = presentation_definition.create_presentation_from_credentials(&vc_jwts);

        assert!(result.is_ok(), "Failed to create presentation submission");

        let presentation_result = result.unwrap();

        assert_eq!(
            presentation_result.presentation_submission.definition_id,
            "test_pd_id"
        );
        assert_eq!(
            presentation_result
                .presentation_submission
                .descriptor_map
                .len(),
            1
        );
        assert_eq!(
            presentation_result.presentation_submission.descriptor_map[0].id,
            "test_input_1"
        );
        assert_eq!(
            presentation_result.presentation_submission.descriptor_map[0].format,
            "jwt_vc"
        );
        assert_eq!(
            presentation_result.presentation_submission.descriptor_map[0].path,
            "$.verifiableCredential[0]"
        );
    }

    /// This test demonstrates the full flow of a Presentation Exchange using a Verifiable Credential (VC) and a Presentation Definition (PD).
    /// It covers the following steps:
    ///
    /// 1. Creating an issuer and defining a Presentation Definition (PD) that outlines the criteria for acceptable VCs.
    /// 2. Creating a Verifiable Credential (VC) that matches the requirements specified in the PD.
    /// 3. Signing the VC to generate a Verifiable Credential in JWT format.
    /// 4. Selecting credentials that fulfill the PD's input descriptors using `create_presentation_from_credentials`.
    /// 5. Creating a Verifiable Presentation (VP) using the selected credentials.
    /// 6. Signing the VP to generate a Verifiable Presentation in JWT format.
    /// 7. Decoding and verifying the signed VP to ensure its integrity and correctness.
    ///
    /// This flow illustrates how a holder can create a VP that satisfies a verifier's requirements, based on input descriptors.

    #[test]
    fn test_presentation_exchange_full_flow() {
        // Step 1: Create an issuer (typically the entity that issued the credential)
        let issuer = DidJwk::create(None).unwrap();
        let issuer_uri = issuer.clone().did.uri;

        // Step 2: Define a Presentation Definition (PD) that specifies the required input descriptors
        // In this case, the input descriptor specifies that the credential must contain a `credentialSubject.id` matching a DID JWK pattern.
        let presentation_definition = PresentationDefinition {
            id: "test_pd_id".to_string(),
            name: Some("Test Presentation Definition".to_string()),
            purpose: Some("Testing".to_string()),
            input_descriptors: vec![InputDescriptor {
                id: "test_input_1".to_string(),
                name: Some("Test Input 1".to_string()),
                purpose: Some("Testing Input 1".to_string()),
                constraints: Constraints {
                    fields: vec![Field {
                        path: vec!["$.credentialSubject.id".to_string()],
                        filter: Some(Filter {
                            r#type: Some("string".to_string()),
                            pattern: Some("^did:jwk:.*$".to_string()),
                            const_value: None,
                            contains: None,
                        }),
                        id: None,
                        name: None,
                        purpose: None,
                        optional: None,
                        predicate: None,
                    }],
                },
            }],
            submission_requirements: None,
        };

        // Step 3: Create a Verifiable Credential (VC) that contains a credential subject matching the PD criteria
        let vc_1 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject::from(issuer_uri.clone()),
            Default::default(),
        )
        .unwrap();

        // Step 4: Sign the VC to generate a VC in JWT format
        let signed_vcjwt_1 = vc_1.sign(&issuer.clone(), None).unwrap();

        // Step 5: Collect the JWTs into a vector
        let vc_jwts = vec![signed_vcjwt_1.clone()];

        // Step 6: Select the credentials that match the PD's input descriptors
        let presentation_result = presentation_definition
            .create_presentation_from_credentials(&vc_jwts)
            .unwrap();

        // Step 7: Create the Verifiable Presentation (VP) with the selected credentials and additional data
        let holder = DidJwk::create(None).unwrap();
        let holder_uri = holder.clone().did.uri;

        // Additional data includes the presentation submission, which links the presentation to the PD
        let mut additional_data = HashMap::new();
        additional_data.insert(
            "presentation_submission".to_string(),
            json!(presentation_result.presentation_submission),
        );

        // Create VP with the matched credentials and additional data
        let vp_create_options = VerifiablePresentationCreateOptions {
            additional_data: Some(additional_data),
            ..Default::default()
        };

        // Generate the Verifiable Presentation
        let vp = VerifiablePresentation::create(
            holder_uri.clone(),
            presentation_result.matched_vc_jwts, // Use the selected credentials from the PD
            Some(vp_create_options),
        )
        .expect("Failed to create Verifiable Presentation");

        // Step 8: Sign the VP to generate a JWT format
        let vp_jwt = vp.sign(&holder.clone(), None).unwrap();

        // Step 9: Decode and verify the signed VP to ensure correctness
        let decoded_vp = VerifiablePresentation::from_vp_jwt(&vp_jwt, true)
            .expect("Failed to decode Verifiable Presentation JWT");

        // Step 10: Verify the holder matches the expected holder
        assert_eq!(decoded_vp.holder, holder_uri);

        // Step 11: Verify that the correct Verifiable Credential was included in the presentation
        assert_eq!(decoded_vp.verifiable_credential.len(), 1);
        assert_eq!(decoded_vp.verifiable_credential[0], signed_vcjwt_1);

        // Step 12: Retrieve the presentation_submission from the decoded VP's additional data
        let decoded_presentation_submission = decoded_vp
            .additional_data
            .as_ref()
            .and_then(|data| data.get("presentation_submission"));

        // Step 13: Check if the decoded presentation_submission is equal to the original one
        assert_eq!(
            decoded_presentation_submission,
            Some(&json!(presentation_result.presentation_submission)),
            "The presentation_submission in additional_data does not match the expected value"
        );
    }

    #[test]
    fn test_presentation_exchange_with_multiple_vcs() {
        let issuer = DidJwk::create(None).unwrap();
        let issuer_uri = issuer.clone().did.uri;

        // Define a Presentation Definition with multiple input descriptors
        let presentation_definition = PresentationDefinition {
            id: "test_pd_id".to_string(),
            name: Some("Test Presentation Definition".to_string()),
            purpose: Some("Testing".to_string()),
            input_descriptors: vec![
                InputDescriptor {
                    id: "test_input_1".to_string(),
                    name: Some("Test Input 1".to_string()),
                    purpose: Some("Testing Input 1".to_string()),
                    constraints: Constraints {
                        fields: vec![Field {
                            path: vec!["$.credentialSubject.id".to_string()],
                            filter: Some(Filter {
                                r#type: Some("string".to_string()),
                                pattern: Some("^did:jwk:.*$".to_string()),
                                const_value: None,
                                contains: None,
                            }),
                            id: None,
                            name: None,
                            purpose: None,
                            optional: None,
                            predicate: None,
                        }],
                    },
                },
                InputDescriptor {
                    id: "test_input_2".to_string(),
                    name: Some("Test Input 2".to_string()),
                    purpose: Some("Testing Input 2".to_string()),
                    constraints: Constraints {
                        fields: vec![Field {
                            path: vec!["$.credentialSubject.role".to_string()],
                            filter: Some(Filter {
                                r#type: Some("string".to_string()),
                                pattern: Some("^admin$".to_string()),
                                const_value: None,
                                contains: None,
                            }),
                            id: None,
                            name: None,
                            purpose: None,
                            optional: None,
                            predicate: None,
                        }],
                    },
                },
            ],
            submission_requirements: None,
        };

        // Create 3 Verifiable Credentials, where one doesn't match any input descriptor
        let vc_1 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject {
                id: issuer_uri.clone(),
                additional_properties: None, // No additional properties for this one
            },
            Default::default(),
        )
        .unwrap();

        // For vc_2, we add the "role" property to match the second input descriptor
        let vc_2_credential_subject = CredentialSubject {
            id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            additional_properties: Some(JsonObject {
                properties: [("role".to_string(), JsonValue::String("admin".to_string()))]
                    .into_iter()
                    .collect(),
            }),
        };

        let vc_2 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            vc_2_credential_subject,
            Default::default(),
        )
        .unwrap();

        // vc_3 won't match either input descriptor
        let vc_3 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject {
                id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
                additional_properties: None, // No matching role
            },
            Default::default(),
        )
        .unwrap();

        // Sign all three VCs
        let signed_vcjwt_1 = vc_1.sign(&issuer.clone(), None).unwrap();
        let signed_vcjwt_2 = vc_2.sign(&issuer.clone(), None).unwrap();
        let signed_vcjwt_3 = vc_3.sign(&issuer.clone(), None).unwrap();

        let vc_jwts = vec![
            signed_vcjwt_1.clone(),
            signed_vcjwt_2.clone(),
            signed_vcjwt_3.clone(), // This VC should not be included in the submission
        ];

        // Unwrap the result of `create_presentation_from_credentials`
        let presentation_result = presentation_definition
            .create_presentation_from_credentials(&vc_jwts)
            .unwrap();

        let holder = DidJwk::create(None).unwrap();
        let holder_uri = holder.clone().did.uri;

        let mut additional_data = HashMap::new();
        additional_data.insert(
            "presentation_submission".to_string(),
            json!(presentation_result.presentation_submission),
        );

        let vp_create_options = VerifiablePresentationCreateOptions {
            additional_data: Some(additional_data),
            ..Default::default()
        };

        // Use the `selected_vcs` directly
        let vp = VerifiablePresentation::create(
            holder_uri.clone(),
            presentation_result.matched_vc_jwts,
            Some(vp_create_options),
        )
        .expect("Failed to create Verifiable Presentation");

        let vp_jwt = vp.sign(&holder.clone(), None).unwrap();

        // Decode the Verifiable Presentation JWT
        let decoded_vp = VerifiablePresentation::from_vp_jwt(&vp_jwt, true)
            .expect("Failed to decode Verifiable Presentation JWT");

        // Check that the holder matches
        assert_eq!(decoded_vp.holder, holder_uri);

        // Check that the verifiable credentials match the selected ones (should only include vc_1 and vc_2)
        assert_eq!(decoded_vp.verifiable_credential.len(), 2);
        assert!(decoded_vp.verifiable_credential.contains(&signed_vcjwt_1));
        assert!(decoded_vp.verifiable_credential.contains(&signed_vcjwt_2));
        assert!(!decoded_vp.verifiable_credential.contains(&signed_vcjwt_3)); // vc_3 should not be included

        // Retrieve the presentation_submission from decoded_vp.additional_data
        let decoded_presentation_submission = decoded_vp
            .additional_data
            .as_ref()
            .and_then(|data| data.get("presentation_submission"));

        // Check if the decoded presentation_submission is equal to the original one
        assert_eq!(
            decoded_presentation_submission,
            Some(&json!(presentation_result.presentation_submission)),
            "The presentation_submission in additional_data does not match the expected value"
        );
    }
}
