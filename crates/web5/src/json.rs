use crate::errors::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

pub trait FromJson: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonObject {
    #[serde(flatten)]
    pub properties: HashMap<String, JsonValue>,
}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: JsonValue) {
        self.properties.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.properties.get(key)
    }
}

impl Default for JsonObject {
    fn default() -> Self {
        Self::new()
    }
}
