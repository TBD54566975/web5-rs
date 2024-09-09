use crate::errors::{Result, Web5Error};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub trait FromJson: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    fn from_json_byte_array(json: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(json)?)
    }
}

pub trait ToJson: Serialize {
    fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub trait FromJsonValue: Sized {
    fn from_json_value(value: &JsonValue) -> Result<Self>;
}

pub(crate) fn json_value_type_name(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool(_) => "bool",
        JsonValue::Number(_) => "number",
        JsonValue::String(_) => "string",
        JsonValue::Array(_) => "array",
        JsonValue::Object(_) => "object",
    }
}

impl FromJsonValue for bool {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::Bool(ref b) = *value {
            Ok(b.clone())
        } else {
            Err(Web5Error::Json(format!(
                "expected bool, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl FromJsonValue for f64 {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::Number(ref n) = *value {
            Ok(n.clone())
        } else {
            Err(Web5Error::Json(format!(
                "expected number, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl FromJsonValue for String {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::String(ref s) = *value {
            Ok(s.clone())
        } else {
            Err(Web5Error::Json(format!(
                "expected string, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl FromJsonValue for Vec<JsonValue> {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::Array(ref arr) = *value {
            Ok(arr.clone())
        } else {
            Err(Web5Error::Json(format!(
                "expected array, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl FromJsonValue for HashMap<String, JsonValue> {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::Object(ref obj) = *value {
            Ok(obj.clone())
        } else {
            Err(Web5Error::Json(format!(
                "expected object, but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

impl FromJsonValue for SystemTime {
    fn from_json_value(value: &JsonValue) -> Result<Self> {
        if let JsonValue::String(ref s) = *value {
            let datetime =
                DateTime::parse_from_rfc3339(s).map_err(|e| Web5Error::DateTime(e.to_string()))?;
            let system_time = datetime.with_timezone(&Utc).timestamp();
            Ok(UNIX_EPOCH + std::time::Duration::from_secs(system_time as u64))
        } else {
            Err(Web5Error::Json(format!(
                "expected string (RFC3339 datetime), but found {}",
                json_value_type_name(value)
            )))
        }
    }
}

pub trait ToJsonValue: Sized {
    fn to_json_value(&self) -> Result<JsonValue>;
}

impl ToJsonValue for bool {
    fn to_json_value(&self) -> Result<JsonValue> {
        Ok(JsonValue::Bool(*self))
    }
}

impl ToJsonValue for f64 {
    fn to_json_value(&self) -> Result<JsonValue> {
        Ok(JsonValue::Number(*self))
    }
}

impl ToJsonValue for String {
    fn to_json_value(&self) -> Result<JsonValue> {
        Ok(JsonValue::String(self.clone()))
    }
}

impl ToJsonValue for Vec<JsonValue> {
    fn to_json_value(&self) -> Result<JsonValue> {
        Ok(JsonValue::Array(self.clone()))
    }
}

impl ToJsonValue for HashMap<String, JsonValue> {
    fn to_json_value(&self) -> Result<JsonValue> {
        Ok(JsonValue::Object(self.clone()))
    }
}

impl ToJsonValue for SystemTime {
    fn to_json_value(&self) -> Result<JsonValue> {
        let duration_since_epoch = self
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Web5Error::DateTime(e.to_string()))?;
        let datetime = DateTime::<Utc>::from(UNIX_EPOCH + duration_since_epoch);
        Ok(JsonValue::String(datetime.to_rfc3339()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonObject {
    #[serde(flatten)]
    pub properties: HashMap<String, JsonValue>,
}

impl FromJson for JsonObject {}
impl ToJson for JsonObject {}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, key: &str, value: JsonValue) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn insert<T>(&mut self, key: &str, value: &T) -> Result<()>
    where
        T: ToJsonValue,
    {
        self.properties
            .insert(key.to_string(), value.to_json_value()?);
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&JsonValue> {
        self.properties.get(key)
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromJsonValue,
    {
        let value = match self.get_value(key) {
            None => None,
            Some(v) => Some(T::from_json_value(v)?),
        };
        Ok(value)
    }
}

impl Default for JsonObject {
    fn default() -> Self {
        Self::new()
    }
}
