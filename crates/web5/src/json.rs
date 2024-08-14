use crate::errors::Web5Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Web5Error>;

pub(crate) trait FromJson: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Web5Error::from)
    }
}

pub(crate) trait ToJson: Serialize {
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Web5Error::from)
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
        JsonObject {
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
