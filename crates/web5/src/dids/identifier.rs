use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

/// Errors that can occur when working with DID methods.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum IdentifierError {
    #[error("Failure initializing regex pattern")]
    RegexPatternFailure(String),
    #[error("Failure parsing URI {0}")]
    ParseFailure(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Identifier {
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

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uri)
    }
}

static METHOD_INDEX: usize = 1;
static METHOD_ID_INDEX: usize = 2;
static PARAMS_INDEX: usize = 4;
static PATH_INDEX: usize = 6;
static QUERY_INDEX: usize = 7;
static FRAGMENT_INDEX: usize = 8;

static DID_URL_PATTERN: OnceLock<Result<Regex, IdentifierError>> = OnceLock::new();

fn regex_pattern() -> &'static Result<Regex, IdentifierError> {
    DID_URL_PATTERN.get_or_init(|| {
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
        .map_err(|e| IdentifierError::RegexPatternFailure(e.to_string()))
    })
}

impl Identifier {
    // Parse parses a DID URI in accordance to the ABNF rules specified in the
    // specification here: https://www.w3.org/TR/did-core/#did-syntax. Returns
    // a DIDURI instance if parsing is successful. Otherwise, returns an error.
    pub fn parse(uri: &str) -> Result<Identifier, IdentifierError> {
        let pattern = regex_pattern().as_ref().map_err(|e| e.clone())?;

        let captures = pattern
            .captures(uri)
            .ok_or(IdentifierError::ParseFailure(uri.to_string()))?;

        let params = captures
            .get(PARAMS_INDEX)
            .filter(|params_match| !params_match.as_str().is_empty())
            .map(|params_match| {
                let params_str = params_match.as_str();
                let params = params_str[1..].split(';');
                params
                    .map(|p| {
                        let kv: Vec<&str> = p.split('=').collect();
                        (kv[0].to_string(), kv[1].to_string())
                    })
                    .collect::<HashMap<_, _>>()
            });

        let path = captures
            .get(PATH_INDEX)
            .map(|path_match| path_match.as_str().to_string());
        let query = captures
            .get(QUERY_INDEX)
            .map(|query_match| query_match.as_str()[1..].to_string());
        let fragment = captures
            .get(FRAGMENT_INDEX)
            .map(|fragment_match| fragment_match.as_str()[1..].to_string());

        let identifier = Identifier {
            uri: format!(
                "did:{}:{}",
                &captures[METHOD_INDEX], &captures[METHOD_ID_INDEX]
            ),
            url: uri.to_string(),
            method: captures[METHOD_INDEX].to_string(),
            id: captures[METHOD_ID_INDEX].to_string(),
            params,
            path,
            query,
            fragment,
        };

        Ok(identifier)
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
                Some(Identifier {
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
                Some(Identifier {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi;foo=bar;baz=qux".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    params: Some(HashMap::from([
                        ("foo".to_string(), "bar".to_string()),
                        ("baz".to_string(), "qux".to_string()),
                    ])),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi?foo=bar&baz=qux",
                false,
                Some(Identifier {
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
                Some(Identifier {
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
                Some(Identifier {
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
                Some(Identifier {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1"
                        .to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    params: Some(HashMap::from([
                        ("foo".to_string(), "bar".to_string()),
                        ("baz".to_string(), "qux".to_string()),
                    ])),
                    query: Some("foo=bar&baz=qux".to_string()),
                    fragment: Some("keys-1".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "did:example:123456789abcdefghi/path/to/resource",
                false,
                Some(Identifier {
                    uri: "did:example:123456789abcdefghi".to_string(),
                    url: "did:example:123456789abcdefghi/path/to/resource".to_string(),
                    method: "example".to_string(),
                    id: "123456789abcdefghi".to_string(),
                    path: Some("/path/to/resource".to_string()),
                    ..Default::default()
                }),
            ),
        ];

        for (uri, is_error, expected) in test_cases {
            match Identifier::parse(uri) {
                Ok(did) => {
                    assert!(!is_error, "Expected error for input: {}", uri);
                    assert_eq!(did, expected.unwrap(), "Unexpected result for uri: {}", uri);
                }
                Err(e) => {
                    assert!(is_error, "Unexpected success for input: {}", uri);
                    assert_eq!(
                        e,
                        IdentifierError::ParseFailure(uri.to_string()),
                        "Unexpected error result for uri: {}",
                        uri
                    );
                }
            }
        }
    }
}
