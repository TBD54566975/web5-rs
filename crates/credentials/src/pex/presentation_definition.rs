use std::collections::HashSet;

use hex;
use jsonpath_rust::{
    JsonPathFinder,
    JsonPathValue::{NewValue, NoValue, Slice},
};
use jsonschema::JSONSchema;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};

use crate::pex::PexError;

use super::Result;

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
    pub optional: bool,
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
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.gen_range(0..256) as u8).collect();
    hex::encode(bytes)
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

// #[derive(Copy)]
struct TokenizedField<'a> {
    pub token: String,
    pub path: &'a String,
}

impl InputDescriptor {
    pub fn select_credentials(&self, vc_jwts: &Vec<String>) -> Result<Vec<String>> {
        let mut tokenized_fields: Vec<TokenizedField> = vec![];
        let mut json_schema_builder = JsonSchemaBuilder::new();

        for field in &self.constraints.fields {
            let token = generate_token();
            for path in &field.path {
                tokenized_fields.push(TokenizedField {
                    token: token.clone(),
                    path: path,
                });
            }

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

        let mut selected_jwts: HashSet<String> = HashSet::new();

        for vc_jwt in vc_jwts {
            let mut selection_candidate: Map<String, Value> = Map::new();

            for tokenized_field in &tokenized_fields {
                if selection_candidate.contains_key(&tokenized_field.token) {
                    continue;
                }

                let finder = if let Ok(f) = JsonPathFinder::from_str(&vc_jwt, tokenized_field.path)
                {
                    f
                } else {
                    continue;
                };

                let json_path_matches = finder.find_slice();
                let json_path_value = if let Some(val) = json_path_matches.first() {
                    val
                } else {
                    continue;
                };

                let val = match json_path_value {
                    Slice(val, _) => (*val).clone(),
                    NewValue(val) => val.clone(),
                    NoValue => continue,
                };
                selection_candidate.insert(tokenized_field.token.clone(), val);
            }

            let schema = JSONSchema::compile(&json_schema_builder.to_json()).map_err(|_| {
                PexError::JsonError(format!(
                    "Failed to create json schema from {}",
                    json_schema_builder.to_json()
                ))
            })?;
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
        self.properties[&name] = property;
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
    // todo
}
