use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum AnyError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Default, Clone)]
pub struct Any {
    pub value: Value,
}

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Value::deserialize(deserializer).map(|value| Any { value })
    }
}

impl Any {
    pub fn from_json_string(json_string: String) -> Result<Self, AnyError> {
        let value: Value = serde_json::from_str(&json_string)?;
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialize_simple_values() {
        let any = Any {
            value: json!("hello"),
        };
        let serialized = serde_json::to_string(&any).unwrap();
        assert_eq!(serialized, "\"hello\"");
    }

    #[test]
    fn test_serialize_complex_object() {
        let any = Any {
            value: json!({"key": "value", "number": 10}),
        };
        let serialized = serde_json::to_string(&any).unwrap();
        assert_eq!(serialized, "{\"key\":\"value\",\"number\":10}");
    }

    #[test]
    fn test_deserialize_simple_values() {
        let json_str = "\"hello\"";
        let any: Any = serde_json::from_str(json_str).unwrap();
        assert_eq!(any.value, json!("hello"));
    }

    #[test]
    fn test_deserialize_complex_object() {
        let json_str = "{\"key\":\"value\",\"number\":10}";
        let any: Any = serde_json::from_str(json_str).unwrap();
        assert_eq!(any.value, json!({"key": "value", "number": 10}));
    }

    #[test]
    fn test_from_json_string_error_handling() {
        let json_str = "not a valid json";
        let result = Any::from_json_string(json_str.to_string());
        assert!(result.is_err());
    }
}
