use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::json::{FromJson, JsonObject, ToJson};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<JsonObject>,
}

impl FromJson for CredentialSubject {}
impl ToJson for CredentialSubject {}

impl<I: Into<String>> From<I> for CredentialSubject {
    fn from(s: I) -> Self {
        Self {
            id: s.into(),
            additional_properties: None,
        }
    }
}

impl Display for CredentialSubject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let subject = CredentialSubject::from("test_id");
        assert_eq!(subject.id, "test_id");
        assert!(subject.additional_properties.is_none());
    }

    #[test]
    fn test_display() {
        let subject = CredentialSubject::from("test_id");
        assert_eq!(subject.to_string(), "test_id");
    }
}
