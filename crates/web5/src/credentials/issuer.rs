use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::json::{FromJson, JsonObject, ToJson};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ObjectIssuer {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: Option<JsonObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Issuer {
    String(String),
    Object(ObjectIssuer),
}

impl FromJson for Issuer {}
impl ToJson for Issuer {}

impl<I> From<I> for Issuer
where
    I: Into<String>,
{
    fn from(s: I) -> Self {
        Issuer::String(s.into())
    }
}

impl Display for Issuer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Issuer::String(s) => write!(f, "{}", s),
            Issuer::Object(ni) => write!(f, "{}", ni.id),
        }
    }
}
