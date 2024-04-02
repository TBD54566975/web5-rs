use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DID {
    // URI represents the complete Decentralized Identifier (DID) URI.
    // Spec: https://www.w3.org/TR/did-core/#did-syntax
    uri: String,

    // URL represents the DID URI + A network location identifier for a specific resource
    // Spec: https://www.w3.org/TR/did-core/#did-url-syntax
    url: String,

    // Method specifies the DID method in the URI, which indicates the underlying
    // method-specific identifier scheme (e.g., jwk, dht, key, etc.).
    // Spec: https://www.w3.org/TR/did-core/#method-schemes
    method: String,

    // ID is the method-specific identifier in the DID URI.
    // Spec: https://www.w3.org/TR/did-core/#method-specific-id
    id: String,

    // Params is a map containing optional parameters present in the DID URI.
    // These parameters are method-specific.
    // Spec: https://www.w3.org/TR/did-core/#did-parameters
    params: Option<HashMap<String, String>>,

    // Path is an optional path component in the DID URI.
    // Spec: https://www.w3.org/TR/did-core/#path
    path: Option<String>,

    // Query is an optional query component in the DID URI, used to express a request
    // for a specific representation or resource related to the DID.
    // Spec: https://www.w3.org/TR/did-core/#query
    query: Option<String>,

    // Fragment is an optional fragment component in the DID URI, used to reference
    // a specific part of a DID document.
    // Spec: https://www.w3.org/TR/did-core/#fragment
    fragment: Option<String>,
}

// relevant ABNF rules: https://www.w3.org/TR/did-core/#did-syntax
lazy_static! {
    static ref PCT_ENCODED_PATTERN: &'static str = r"(?:%[0-9a-fA-F]{2})";
    static ref ID_CHAR_PATTERN: String = format!(r"(?:[a-zA-Z0-9._-]|{})", *PCT_ENCODED_PATTERN);
    static ref METHOD_PATTERN: &'static str = r"([a-z0-9]+)";
    static ref METHOD_ID_PATTERN: String =
        format!(r"((?:{}*:)*({}+))", *ID_CHAR_PATTERN, *ID_CHAR_PATTERN);
    static ref PARAM_CHAR_PATTERN: &'static str = r"[a-zA-Z0-9_.:%-]";
    static ref PARAM_PATTERN: String =
        format!(r";{}+={}*", *PARAM_CHAR_PATTERN, *PARAM_CHAR_PATTERN);
    static ref PARAMS_PATTERN: String = format!(r"(({})*)", *PARAM_PATTERN);
    static ref PATH_PATTERN: &'static str = r"(/[^#?]*)?";
    static ref QUERY_PATTERN: &'static str = r"(\?[^\#]*)?";
    static ref FRAGMENT_PATTERN: &'static str = r"(\#.*)?";
    static ref DID_URI_PATTERN: Regex = Regex::new(&format!(
        r"^did:{}:{}{}{}{}{}$",
        *METHOD_PATTERN,
        *METHOD_ID_PATTERN,
        *PARAMS_PATTERN,
        *PATH_PATTERN,
        *QUERY_PATTERN,
        *FRAGMENT_PATTERN
    ))
    .unwrap();
}

// Parse parses a DID URI in accordance to the ABNF rules specified in the
// specification here: https://www.w3.org/TR/did-core/#did-syntax. Returns
// a DIDURI instance if parsing is successful. Otherwise, returns an error.
pub fn parse(input: &str) -> Result<DID, String> {
    if let Some(captures) = DID_URI_PATTERN.captures(input) {
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
            match parse(input) {
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
