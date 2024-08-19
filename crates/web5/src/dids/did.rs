use crate::errors::{Result, Web5Error};
use regex::Regex;
use std::fmt;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Did {
    pub uri: String,
    pub url: String,
    pub method: String,
    pub id: String,
    pub params: Option<HashMap<String, String>>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl fmt::Display for Did {
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

static DID_URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
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
    .map_err(|e| {
        Web5Error::Parameter(format!(
            "DID_URL_PATTERN regex instantiation failure: {}",
            e
        ))
    })
    .unwrap() // immediately panic on startup if regex is faulty, this will assist in shift-left development
});

impl Did {
    pub fn parse(uri: &str) -> Result<Did> {
        let captures = DID_URL_PATTERN
            .captures(uri)
            .ok_or(Web5Error::Parameter(format!(
                "identifier regex match failure {}",
                uri
            )))?;

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

        let did = Did {
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

        Ok(did)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;
        use crate::{test_helpers::UnitTestSuite, test_name};

        static TEST_SUITE: LazyLock<UnitTestSuite> =
            LazyLock::new(|| UnitTestSuite::new("did_new"));

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_did_empty_string_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_incomplete_scheme_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_missing_id_part_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:uport";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_missing_id_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:uport:";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_invalid_characters_in_id_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:uport:1234_12313***";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_invalid_bare_id_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "2nQtiQG6Cgm1GYTBaaKAgr76uY7iSexUkqX";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_invalid_percent_encoding_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:method:%12%1";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_invalid_percent_encoding_incomplete_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:method:%1233%Ay";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_capitalized_method_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:CAP:id";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_invalid_additional_id_should_error() {
            TEST_SUITE.include(test_name!());

            let uri = "did:method:id::anotherid%r9";
            let result = Did::parse(uri);
            assert!(result.is_err(), "Expected error for input: {}", uri);
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter(format!("identifier regex match failure {}", uri))
            );
        }

        #[test]
        fn test_did_valid_did_no_params_path_query_fragment() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi";
            let expected = Did {
                uri: uri.to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_params() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi;foo=bar;baz=qux";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                params: Some(HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("baz".to_string(), "qux".to_string()),
                ])),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_query() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi?foo=bar&baz=qux";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                query: Some("foo=bar&baz=qux".to_string()),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_fragment() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi#keys-1";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                fragment: Some("keys-1".to_string()),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_query_and_fragment() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                query: Some("foo=bar&baz=qux".to_string()),
                fragment: Some("keys-1".to_string()),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_params_query_and_fragment() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                params: Some(HashMap::from([
                    ("foo".to_string(), "bar".to_string()),
                    ("baz".to_string(), "qux".to_string()),
                ])),
                query: Some("foo=bar&baz=qux".to_string()),
                fragment: Some("keys-1".to_string()),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }

        #[test]
        fn test_did_valid_did_with_path() {
            TEST_SUITE.include(test_name!());

            let uri = "did:example:123456789abcdefghi/path/to/resource";
            let expected = Did {
                uri: "did:example:123456789abcdefghi".to_string(),
                url: uri.to_string(),
                method: "example".to_string(),
                id: "123456789abcdefghi".to_string(),
                path: Some("/path/to/resource".to_string()),
                ..Default::default()
            };
            let result = Did::parse(uri).unwrap();
            assert_eq!(result, expected);
        }
    }
}
