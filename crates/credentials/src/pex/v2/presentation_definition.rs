use jsonschema::{Draft, JSONSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Presentation Exchange
///
/// Presentation Exchange specification codifies a Presentation Definition data format Verifiers
/// can use to articulate proof requirements, and a Presentation Submission data format Holders can
/// use to describe proofs submitted in accordance with them.
///
/// See [Presentation Definition](https://identity.foundation/presentation-exchange/spec/v2.0.0/#presentation-definition)
/// for more information.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PresentationDefinition {
    pub id: String,
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub format: Option<Format>,
    pub submission_requirements: Option<Vec<SubmissionRequirement>>,
    pub input_descriptors: Vec<InputDescriptor>,
    pub frame: Option<HashMap<String, JsonValue>>,
}

/// Represents an input descriptor in a presentation definition.
///
/// See [Input Descriptor](https://identity.foundation/presentation-exchange/spec/v2.0.0/#input-descriptor-object)
/// for more information.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDescriptor {
    pub id: String,
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub format: Option<Format>,
    pub constraints: Constraints,
}

/// Represents constraints for an input descriptor.
///
/// See 'constraints object' defined in
/// [Input Descriptor](https://identity.foundation/presentation-exchange/spec/v2.0.0/#input-descriptor-object)
/// for more information.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Constraints {
    pub fields: Option<Vec<Field>>,
    pub limit_disclosure: Option<ConformantConsumerDisclosure>,
}

/// Represents a field in a presentation input descriptor.
///
/// See 'fields object' as defined in
/// [Input Descriptor](https://identity.foundation/presentation-exchange/spec/v2.0.0/#input-descriptor-object)
/// for more information.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Field {
    pub id: Option<String>,
    pub path: Vec<String>,
    pub purpose: Option<String>,
    pub filter: Option<JsonValue>,
    pub predicate: Option<Optionality>,
    pub name: Option<String>,
    pub optional: Option<bool>,
}

impl Field {
    pub fn filter_schema(&self) -> Option<JSONSchema> {
        self.filter
            .as_ref()
            .map(|json| {
                JSONSchema::options()
                    .with_draft(Draft::Draft7)
                    .compile(json)
                    .ok()
            })
            .flatten()
    }
}

/// Enumeration representing consumer disclosure options.
///
/// Represents the possible values of `limit_disclosure' property as defined in
// [Input Descriptor](https://identity.foundation/presentation-exchange/spec/v2.0.0/#input-descriptor-object)
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformantConsumerDisclosure {
    Required,
    Preferred,
}

/// Represents the format of a presentation definition
///
/// See `format` as defined in
/// [Input Descriptor](https://identity.foundation/presentation-exchange/spec/v2.0.0/#input-descriptor-object)
/// and [Registry](https://identity.foundation/claim-format-registry/#registry)
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Format {
    pub jwt: Option<JwtObject>,
    pub jwt_vc: Option<JwtObject>,
    pub jwt_vp: Option<JwtObject>,
}

/// Represents a JWT object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct JwtObject {
    pub alg: Vec<String>,
}

/// Represents submission requirements for a presentation definition.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubmissionRequirement {
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub rule: Rule,
    pub count: Option<u32>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub from: Option<String>,
    pub from_nested: Option<Vec<SubmissionRequirement>>,
}

/// Enumeration representing presentation rule options.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    #[default]
    All,
    Pick,
}

/// Enumeration representing optionality.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Optionality {
    Required,
    Preferred,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_canonical_json::CanonicalFormatter;
    use serde_json::Serializer;
    use serde_json::{json, Value};
    use std::fs;
    use std::path::Path;

    #[test]
    fn can_serialize() {
        let pd = PresentationDefinition {
            id: "tests-pd-id".to_string(),
            name: "simple PD".to_string().into(),
            purpose: "pd for testing".to_string().into(),
            input_descriptors: vec![InputDescriptor {
                id: "whatever".to_string(),
                purpose: "purpose".to_string().into(),
                constraints: Constraints {
                    fields: vec![Field {
                        id: "field-id".to_string().into(),
                        path: vec!["$.issuer".to_string()],
                        purpose: "purpose".to_string().into(),
                        filter: json!({"type": "string", "const": "123"}).into(),
                        ..Default::default()
                    }]
                    .into(),
                    limit_disclosure: Some(ConformantConsumerDisclosure::Required),
                },
                ..Default::default()
            }],
            ..Default::default()
        };

        let serialized_pd = serde_json::to_string(&pd).unwrap();

        assert!(serialized_pd.contains("input_descriptors"));
        assert!(serialized_pd.contains("123"));
    }

    #[test]
    fn serialized_and_deserialized_is_idempotent() {
        let mut ser = Serializer::with_formatter(Vec::new(), CanonicalFormatter::new());
        let raw_string = load_json("tests/resources/pd_sanctions.json");
        let deserialized_value: Value = serde_json::from_str(&raw_string).unwrap();
        deserialized_value.serialize(&mut ser).unwrap();
        let json_value = String::from_utf8(ser.into_inner()).unwrap();

        let mut ser = Serializer::with_formatter(Vec::new(), CanonicalFormatter::new());
        let deserialized_with_type: PresentationDefinition =
            serde_json::from_str(&raw_string).unwrap();
        deserialized_with_type.serialize(&mut ser).unwrap();
        let json_serialized_from_type = String::from_utf8(ser.into_inner()).unwrap();

        assert_eq!(json_value, json_serialized_from_type);
    }

    fn load_json(path: &str) -> String {
        let path = Path::new(path);
        let json = fs::read_to_string(path).expect("Unable to load json file");
        json
    }
}
