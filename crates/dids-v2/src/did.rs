use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DID {
    // URI represents the complete Decentralized Identifier (DID) URI.
    // Spec: https://www.w3.org/TR/did-core/#did-syntax
    pub uri: String,

    // URL represents the DID URI + A network location identifier for a specific resource
    // Spec: https://www.w3.org/TR/did-core/#did-url-syntax
    pub url: String,

    // Method specifies the DID method in the URI, which indicates the underlying
    // method-specific identifier scheme (e.g., jwk, dht, key, etc.).
    // Spec: https://www.w3.org/TR/did-core/#method-schemes
    pub method: String,

    // ID is the method-specific identifier in the DID URI.
    // Spec: https://www.w3.org/TR/did-core/#method-specific-id
    pub id: String,

    // Params is a map containing optional parameters present in the DID URI.
    // These parameters are method-specific.
    // Spec: https://www.w3.org/TR/did-core/#did-parameters
    pub params: Option<HashMap<String, String>>,

    // Path is an optional path component in the DID URI.
    // Spec: https://www.w3.org/TR/did-core/#path
    pub path: Option<String>,

    // Query is an optional query component in the DID URI, used to express a request
    // for a specific representation or resource related to the DID.
    // Spec: https://www.w3.org/TR/did-core/#query
    pub query: Option<String>,

    // Fragment is an optional fragment component in the DID URI, used to reference
    // a specific part of a DID document.
    // Spec: https://www.w3.org/TR/did-core/#fragment
    pub fragment: Option<String>,
}

impl fmt::Display for DID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uri)
    }
}

static DID_URL_PATTERN: OnceLock<Regex> = OnceLock::new();

impl DID {
    // Parse parses a DID URI in accordance to the ABNF rules specified in the
    // specification here: https://www.w3.org/TR/did-core/#did-syntax. Returns
    // a DIDURI instance if parsing is successful. Otherwise, returns an error.
    pub fn parse(input: &str) -> Result<DID, String> {
        let pattern = DID_URL_PATTERN.get_or_init(|| {
            // relevant ABNF rules: https://www.w3.org/TR/did-core/#did-syntax
            let pct_encoded_pattern: &str = r"(?:%[0-9a-fA-F]{2})";
            let method_pattern: &str = r"([a-z0-9]+)";
            let param_char_pattern: &str = r"[a-zA-Z0-9_.:%-]";
            let path_pattern: &str = r"(/[^#?]*)?";
            let query_pattern: &str = r"(\?[^\#]*)?";
            let fragment_pattern: &str = r"(\#.*)?";
            let id_char_pattern = format!(r"(?:[a-zA-Z0-9._-]|{})", pct_encoded_pattern);
            let method_id_pattern = format!(r"((?:{}*:)*({}+))", id_char_pattern, id_char_pattern);
            let param_pattern = format!(r";{}+={}*", param_char_pattern, param_char_pattern);
            let params_pattern = format!(r"(({})*)", param_pattern);

            Regex::new(&format!(
                r"^did:{}:{}{}{}{}{}$",
                method_pattern,
                method_id_pattern,
                params_pattern,
                path_pattern,
                query_pattern,
                fragment_pattern
            ))
            .expect("Failed to compile DID_URL_PATTERN Regex")
        });

        if let Some(captures) = pattern.captures(input) {
            let mut did = DID {
                uri: format!("did:{}:{}", &captures[1], &captures[2]),
                url: input.to_string(),
                method: captures[1].to_string(),
                id: captures[2].to_string(),
                ..Default::default()
            };

            if let Some(params_match) = captures.get(4) {
                let params_str = params_match.as_str();
                if !params_str.is_empty() {
                    let params = params_str[1..].split(';');
                    let mut parsed_params = HashMap::new();
                    for p in params {
                        let kv: Vec<&str> = p.split('=').collect();
                        parsed_params.insert(kv[0].to_string(), kv[1].to_string());
                    }
                    did.params = Some(parsed_params);
                }
            }

            if let Some(path_match) = captures.get(6) {
                did.path = Some(path_match.as_str().to_string());
            }
            if let Some(query_match) = captures.get(7) {
                did.query = Some(query_match.as_str()[1..].to_string());
            }
            if let Some(fragment_match) = captures.get(8) {
                did.fragment = Some(fragment_match.as_str()[1..].to_string());
            }

            Ok(did)
        } else {
            Err("invalid DID URI".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_cases = vec![
            ("", true, None),
            ("did:", true, None),
            ("did:uport", true, None),
            ("did:uport:", true, None),
            ("did:uport:1234_12313***", true, None),
            ("2nQtiQG6Cgm1GYTBaaKAgr76uY7iSexUkqX", true, None),
            ("did:method:%12%1", true, None),
            ("did:method:%1233%Ay", true, None),
            ("did:CAP:id", true, None),
            ("did:method:id::anotherid%r9", true, None),
            (
                "did:example:123456789abcdefghi",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi;foo=bar;baz=qux",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi;foo=bar;baz=qux".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    params: Some(
                        [
                            ("foo".to_string(), "bar".to_string()),
                            ("baz".to_string(), "qux".to_string()),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    ),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi?foo=bar&baz=qux",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi?foo=bar&baz=qux".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    query: Some("foo=bar&baz=qux".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi#keys-1",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi#keys-1".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    fragment: Some("keys-1".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    query: Some("foo=bar&baz=qux".to_string()),
                    fragment: Some("keys-1".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1"
                        .to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    params: Some(
                        [
                            ("foo".to_string(), "bar".to_string()),
                            ("baz".to_string(), "qux".to_string()),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    ),
                    query: Some("foo=bar&baz=qux".to_string()),
                    fragment: Some("keys-1".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi/path/to/resource",
                false,
                Some(DID {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi/path/to/resource".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    path: Some("/path/to/resource".to_string()),
                    ..Default::default()
                }),
            ),
        ];

        for (input, error, expected) in test_cases {
            match DID::parse(input) {
                Ok(did) => {
                    assert!(!error, "Expected error for input: {}", input);
                    assert_eq!(
                        did,
                        expected.unwrap(),
                        "Unexpected result for input: {}",
                        input
                    );
                }
                Err(e) => {
                    assert!(error, "Unexpected success for input: {}", input);
                    assert_eq!(
                        e,
                        "invalid DID URI".to_string(),
                        "Unexpected error message for input: {}",
                        input
                    );
                }
            }
        }
    }
}
