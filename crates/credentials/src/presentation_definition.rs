use std::collections::HashSet;

use jsonpath_rust::{
    JsonPathFinder,
    JsonPathValue::{NewValue, NoValue, Slice},
};
use jsonschema::JSONSchema;
use jws::{CompactJws, JwsError};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum PexError {
    #[error(transparent)]
    JwsError(#[from] JwsError),
    #[error("Failed to parse JSON: {0}")]
    JsonError(String),
}

type Result<T> = std::result::Result<T, PexError>;

/// Represents a DIF Presentation Definition defined [here](https://identity.foundation/presentation-exchange/#presentation-definition).
/// Presentation Definitions are objects that articulate what proofs a Verifier requires.
#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationDefinition {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub input_descriptors: Vec<InputDescriptor>,
}

/// Represents a DIF Input Descriptor defined [here](https://identity.foundation/presentation-exchange/#input-descriptor).
/// Input Descriptors are used to describe the information a Verifier requires of a Holder.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptor {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub constraints: Constraints,
}

/// Contains the requirements for a given Input Descriptor.
#[derive(Debug, Serialize, Deserialize)]
pub struct Constraints {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
}

/// Contains the requirements for a given field within a proof.
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub enum Optionality {
    Required,
    Preferred,
}

/// A JSON Schema that is applied against the value of a field.
#[derive(Debug, Serialize, Deserialize)]
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

fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

impl PresentationDefinition {
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
}

struct TokenizedField<'a> {
    pub token: String,
    pub path: &'a String,
}

fn get_value_at_json_path(json: &str, path: &str) -> Option<Value> {
    let finder = if let Ok(f) = JsonPathFinder::from_str(&json, path) {
        f
    } else {
        return None;
    };

    let json_path_matches = finder.find_slice();
    let json_path_value = if let Some(val) = json_path_matches.first() {
        val
    } else {
        return None;
    };

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
                    path: path,
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

            let decoded_jws = CompactJws::decode(vc_jwt)?;
            let payload_json = String::from_utf8(decoded_jws.payload).map_err(|_| {
                PexError::JsonError(
                    "Could not create json string from vc jwt payload bytes".to_string(),
                )
            })?;

            // Extract a value from the vc_jwt for each tokenized field
            for tokenized_field in &tokenized_fields {
                if selection_candidate.contains_key(&tokenized_field.token) {
                    continue;
                }

                if let Some(val) = get_value_at_json_path(&payload_json, &tokenized_field.path) {
                    selection_candidate.insert(tokenized_field.token.clone(), val);
                }
            }

            let json_value = Value::from(selection_candidate);
            let validation_result = schema.validate(&json_value);
            if let Ok(_) = validation_result {
                selected_jwts.insert(vc_jwt.clone());
            }
        }

        Ok(selected_jwts.into_iter().collect())
    }
}

struct JsonSchemaBuilder {
    schema: String,
    _type: String,
    properties: Map<String, Value>,
    required: Vec<String>,
}

impl JsonSchemaBuilder {
    pub fn new() -> Self {
        JsonSchemaBuilder {
            schema: "http://json-schema.org/draft-07/schema#".to_string(),
            _type: "object".to_string(),
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
            "type": self._type,
            "properties": self.properties,
            "required": self.required,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;

    use super::PresentationDefinition;

    #[derive(Debug, serde::Deserialize)]
    struct SelectCredentialsVectorInput {
        #[serde(rename = "presentationDefinition")]
        pub presentation_definition: PresentationDefinition,
        #[serde(rename = "credentialJwts")]
        pub credential_jwts: Vec<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct SelectCredentialsVectorOutput {
        #[serde(rename = "selectedCredentials")]
        pub selected_credentials: Vec<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct SelectCredentialsVector {
        pub description: String,
        pub input: SelectCredentialsVectorInput,
        pub output: SelectCredentialsVectorOutput,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Vectors {
        pub vectors: Vec<SelectCredentialsVector>,
    }

    fn load_json_fixture(file_path: &str) -> Vectors {
        let data = fs::read_to_string(file_path).unwrap();
        let json = serde_json::from_str(&data).unwrap();
        json
    }

    #[test]
    fn test_web5_spec_test_vectors() {
        let json_path =
            "../../web5-spec/test-vectors/presentation_exchange/select_credentials.json";
        let vectors = load_json_fixture(json_path);

        for vector in vectors.vectors {
            let presentation_definition = vector.input.presentation_definition;
            let vc_jwts = vector.input.credential_jwts;
            let error_msg = format!(
                "Selected Credential test vector ({}) should not have thrown error",
                vector.description
            );

            let selected_credentials = presentation_definition
                .select_credentials(&vc_jwts)
                .expect(&error_msg);

            let set1: HashSet<_> = selected_credentials.iter().collect();
            let set2: HashSet<_> = vector.output.selected_credentials.iter().collect();
            assert_eq!(
                set1, set2,
                "Vectors do not contain the same elements: {}",
                error_msg
            );
        }
    }
}
