use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::json::{FromJson, JsonObject, ToJson};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten)]
    pub additional_properties: Option<JsonObject>,
}

impl FromJson for CredentialSubject {}
impl ToJson for CredentialSubject {}

impl<I> From<I> for CredentialSubject
where
    I: Into<String>,
{
    fn from(s: I) -> Self {
        CredentialSubject {
            id: s.into(),
            ..Default::default()
        }
    }
}

impl Display for CredentialSubject {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
