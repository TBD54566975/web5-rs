use std::collections::HashSet;

use jsonpath_rust::{
    JsonPathFinder,
    JsonPathValue::{NewValue, NoValue, Slice},
};
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};
use uuid::Uuid;
use crate::credentials::verifiable_credential_1_1::VerifiableCredential;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PexError {
    #[error("Failed to parse JSON: {0}")]
    JsonError(String),
    #[error("Invalid PEX state: {0}")]
    IllegalState(String),
}

type Result<T> = std::result::Result<T, PexError>;

/// Represents a DIF Presentation Definition defined [here](https://identity.foundation/presentation-exchange/#presentation-definition).
/// Presentation Definitions are objects that articulate what proofs a Verifier requires.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PresentationDefinition {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub input_descriptors: Vec<InputDescriptor>,
    pub submission_requirements: Option<Vec<SubmissionRequirement>>,
}

/// Represents a DIF Input Descriptor defined [here](https://identity.foundation/presentation-exchange/#input-descriptor).
/// Input Descriptors are used to describe the information a Verifier requires of a Holder.
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SubmissionRequirement {
    pub rule: SubmissionRequirementRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Represents a presentation submission object.
///
/// See [Presentation Submission](https://identity.foundation/presentation-exchange/spec/v2.0.0/#presentation-submission)
#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationSubmission {
    pub id: String,

    #[serde(rename = "definition_id")]
    pub definition_id: String,

    #[serde(rename = "descriptor_map")]
    pub descriptor_map: Vec<InputDescriptorMapping>,
}

/// Represents descriptor map for a presentation submission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputDescriptorMapping {
    pub id: String,
    pub format: String,
    pub path: String,

    #[serde(rename = "path_nested")]
    pub path_nested: Option<Box<InputDescriptorMapping>>,
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
    ///
    /// A `PresentationSubmission` object.
    /// A `Vec<String>` which contains the chosen vc_jwts
    pub fn create_presentation_from_credentials(
        &self,
        vc_jwts: &Vec<String>,
    ) -> Result<(PresentationSubmission, Vec<String>)> {
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

        // Return both the presentation submission and the list of used vc_jwts
        Ok((presentation_submission, used_vc_jwts))
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
    use std::collections::HashMap;
    use super::*;
    use serde_json::json;
    use crate::credentials::{CredentialSubject, Issuer, VerifiablePresentation, VerifiablePresentationCreateOptions};
    use crate::dids::methods::did_jwk::DidJwk;
    use crate::json::{JsonObject, JsonValue};

    #[test]
    fn test_create_presentation_from_credentials() {
        let issuer = DidJwk::create(None).unwrap();
        let issuer_uri = issuer.clone().did.uri;

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
                        fields: vec![
                            Field {
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
                            }
                        ],
                    },
                }
            ],
            submission_requirements: None,
        };

        let vc_1 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject::from(issuer_uri.clone()),
            Default::default(),
        ).unwrap();

        let signed_vcjwt_1 = vc_1.sign(&issuer.clone(), None).unwrap();

        let vc_jwts = vec![
            signed_vcjwt_1
        ];

        let result = presentation_definition.create_presentation_from_credentials(&vc_jwts);

        assert!(result.is_ok(), "Failed to create presentation submission");

        let presentation_submission = result.unwrap();

        assert_eq!(presentation_submission.0.definition_id, "test_pd_id");
        assert_eq!(presentation_submission.0.descriptor_map.len(), 1);
        assert_eq!(presentation_submission.0.descriptor_map[0].id, "test_input_1");
        assert_eq!(presentation_submission.0.descriptor_map[0].format, "jwt_vc");
        assert_eq!(presentation_submission.0.descriptor_map[0].path, "$.verifiableCredential[0]");
    }

    #[test]
    fn test_presentation_exchange_full_flow() {
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

        let vc_jwts = vec![signed_vcjwt_1.clone()];

        let (presentation_submission, selected_vcs) = presentation_definition
            .create_presentation_from_credentials(&vc_jwts)
            .unwrap();

        let holder = DidJwk::create(None).unwrap();
        let holder_uri = holder.clone().did.uri;

        let mut additional_data = HashMap::new();
        additional_data.insert(
            "presentation_submission".to_string(),
            json!(presentation_submission),
        );

        let vp_create_options = VerifiablePresentationCreateOptions {
            additional_data: Some(additional_data),
            ..Default::default()
        };

        let vp = VerifiablePresentation::create(holder_uri.clone(), selected_vcs, Some(vp_create_options))
            .expect("Failed to create Verifiable Presentation");

        let vp_jwt = vp.sign(&holder.clone(), None).unwrap();

        let decoded_vp = VerifiablePresentation::from_vp_jwt(&vp_jwt, true)
            .expect("Failed to decode Verifiable Presentation JWT");

        // Check that the holder matches
        assert_eq!(decoded_vp.holder, holder_uri);

        // Check that the verifiable credential matches
        assert_eq!(decoded_vp.verifiable_credential.len(), 1);
        assert_eq!(decoded_vp.verifiable_credential[0], signed_vcjwt_1);

        // Retrieve the presentation_submission from decoded_vp.additional_data
        let decoded_presentation_submission = decoded_vp
            .additional_data
            .as_ref()
            .and_then(|data| data.get("presentation_submission"));

        // Check if the decoded presentation_submission is equal to the original one
        assert_eq!(
            decoded_presentation_submission,
            Some(&json!(presentation_submission)),
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
        ).unwrap();

        // vc_3 won't match either input descriptor
        let vc_3 = VerifiableCredential::create(
            Issuer::from(issuer_uri.clone()),
            CredentialSubject {
                id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
                additional_properties: None, // No matching role
            },
            Default::default(),
        ).unwrap();

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
        let (presentation_submission, selected_vcs) = presentation_definition
            .create_presentation_from_credentials(&vc_jwts)
            .unwrap();

        let holder = DidJwk::create(None).unwrap();
        let holder_uri = holder.clone().did.uri;

        let mut additional_data = HashMap::new();
        additional_data.insert(
            "presentation_submission".to_string(),
            json!(presentation_submission),
        );

        let vp_create_options = VerifiablePresentationCreateOptions {
            additional_data: Some(additional_data),
            ..Default::default()
        };

        // Use the `selected_vcs` directly
        let vp = VerifiablePresentation::create(holder_uri.clone(), selected_vcs, Some(vp_create_options))
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
            Some(&json!(presentation_submission)),
            "The presentation_submission in additional_data does not match the expected value"
        );
    }
}
