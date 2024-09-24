use super::{
    credential_schema::validate_credential_schema,
    credential_subject::CredentialSubject,
    issuer::Issuer,
    verifiable_credential_1_1::{
        VerifiableCredential, VerifiableCredentialCreateOptions, BASE_CONTEXT, BASE_TYPE,
    },
};
use crate::{
    dids::did::Did,
    errors::{Result, Web5Error},
};
use std::time::SystemTime;
use uuid::Uuid;

pub fn create_vc(
    issuer: Issuer,
    credential_subject: CredentialSubject,
    options: Option<VerifiableCredentialCreateOptions>,
) -> Result<VerifiableCredential> {
    let options = options.unwrap_or_default();

    validate_issuer(&issuer)?;
    validate_credential_subject(&credential_subject)?;

    let context = build_context(options.context);
    let r#type = build_type(options.r#type);
    let id = options
        .id
        .unwrap_or_else(|| format!("urn:uuid:{}", Uuid::new_v4()));

    let verifiable_credential = VerifiableCredential {
        context,
        id,
        r#type,
        issuer,
        issuance_date: options.issuance_date.unwrap_or_else(SystemTime::now),
        expiration_date: options.expiration_date,
        credential_status: options.credential_status,
        credential_subject,
        credential_schema: options.credential_schema,
        evidence: options.evidence,
    };

    validate_credential_schema(&verifiable_credential)?;

    Ok(verifiable_credential)
}

fn validate_issuer(issuer: &Issuer) -> Result<()> {
    if issuer.to_string().is_empty() {
        return Err(Web5Error::Parameter("issuer id must not be empty".into()));
    }

    if let Issuer::Object(ref named_issuer) = issuer {
        if named_issuer.name.is_empty() {
            return Err(Web5Error::Parameter(
                "named issuer name must not be empty".into(),
            ));
        }
    }

    if Did::parse(&issuer.to_string()).is_err() {
        return Err(Web5Error::Parameter(
            "issuer must be a valid DID URI".into(),
        ));
    }

    Ok(())
}

fn validate_credential_subject(credential_subject: &CredentialSubject) -> Result<()> {
    if credential_subject.to_string().is_empty() {
        return Err(Web5Error::Parameter("subject id must not be empty".into()));
    }

    if Did::parse(&credential_subject.to_string()).is_err()
        && !credential_subject.to_string().starts_with("urn:uuid:")
    {
        return Err(Web5Error::Parameter(
            "credential subject must be a valid DID URI or start with 'urn:uuid:'".into(),
        ));
    }

    Ok(())
}

fn build_context(context: Option<Vec<String>>) -> Vec<String> {
    let mut contexts = context.unwrap_or_else(|| vec![BASE_CONTEXT.to_string()]);
    if !contexts.contains(&BASE_CONTEXT.to_string()) {
        contexts.insert(0, BASE_CONTEXT.to_string());
    }
    contexts
}

fn build_type(r#type: Option<Vec<String>>) -> Vec<String> {
    let mut types = r#type.unwrap_or_else(|| vec![BASE_TYPE.to_string()]);
    if !types.contains(&BASE_TYPE.to_string()) {
        types.insert(0, BASE_TYPE.to_string());
    }
    types
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::credential_schema::{CredentialSchema, CREDENTIAL_SCHEMA_TYPE};
    use crate::json::JsonValue;
    use mockito::Server;
    use regex::Regex;
    use std::collections::HashMap;

    const ISSUER_DID_URI: &str = "did:web:tbd.website";
    const SUBJECT_DID_URI: &str = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";

    fn issuer() -> Issuer {
        Issuer::from(ISSUER_DID_URI)
    }
    fn credential_subject() -> CredentialSubject {
        CredentialSubject::from(SUBJECT_DID_URI)
    }
    fn mock_json_schema(url: String, schema_override: Option<String>) -> String {
        let schema = schema_override
            .unwrap_or_else(|| "https://json-schema.org/draft/2020-12/schema".to_string());

        format!(
            r###"
                {{
                    "$id": "{url}/schemas/email.json",
                    "$schema": "{schema}",
                    "title": "EmailCredential",
                    "description": "EmailCredential using JsonSchema",
                    "type": "object",
                    "properties": {{
                        "credentialSubject": {{
                            "type": "object",
                            "properties": {{
                                "emailAddress": {{
                                    "type": "string",
                                    "format": "email"
                                }}
                            }},
                            "required": [
                                "emailAddress"
                            ]
                        }}
                    }}
                }}"###
        )
    }
    fn mock_credential_schema(url: String) -> Option<CredentialSchema> {
        Some(CredentialSchema {
            id: format!("{}/schemas/email.json", url),
            r#type: CREDENTIAL_SCHEMA_TYPE.to_string(),
        })
    }

    use crate::{credentials::issuer::ObjectIssuer, json::JsonObject};

    #[test]
    fn test_default_context_added_if_not_supplied() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(vc.context, vec![BASE_CONTEXT]);
    }

    #[test]
    fn test_default_context_not_duplicated_if_supplied() {
        let options = VerifiableCredentialCreateOptions {
            context: Some(vec![BASE_CONTEXT.to_string()]),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.context, vec![BASE_CONTEXT]);
    }

    #[test]
    fn test_developer_provided_context_appended_to_default() {
        let custom_context = "https://example.com/custom-context";
        let options = VerifiableCredentialCreateOptions {
            context: Some(vec![custom_context.to_string()]),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.context, vec![BASE_CONTEXT, custom_context]);
    }

    #[test]
    fn test_default_type_added_if_not_supplied() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(vc.r#type, vec![BASE_TYPE]);
    }

    #[test]
    fn test_default_type_not_duplicated_if_supplied() {
        let options = VerifiableCredentialCreateOptions {
            r#type: Some(vec![BASE_TYPE.to_string()]),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.r#type, vec![BASE_TYPE]);
    }

    #[test]
    fn test_developer_provided_type_appended_to_default() {
        let custom_type = "CustomType";
        let options = VerifiableCredentialCreateOptions {
            r#type: Some(vec![custom_type.to_string()]),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.r#type, vec![BASE_TYPE, custom_type]);
    }

    #[test]
    fn test_id_generated_if_not_supplied() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        let uuid_regex = Regex::new(r"^urn:uuid:[0-9a-fA-F-]{36}$").unwrap();
        assert!(uuid_regex.is_match(&vc.id));
    }

    #[test]
    fn test_id_must_be_set_if_supplied() {
        let custom_id = "custom-id";
        let options = VerifiableCredentialCreateOptions {
            id: Some(custom_id.to_string()),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.id, custom_id);
    }

    #[test]
    fn test_issuer_string_must_not_be_empty() {
        let empty_issuer = Issuer::from("");
        let result = create_vc(
            empty_issuer,
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(err_msg, "issuer id must not be empty");
            }
            _ => panic!("Expected Web5Error::Parameter with specific error message"),
        };
    }

    #[test]
    fn test_issuer_string_must_be_set() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(vc.issuer, issuer());
    }

    #[test]
    fn test_issuer_string_must_be_valid_did() {
        let result = create_vc(
            Issuer::String("did:invalid-123".to_string()),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(err_msg, "issuer must be a valid DID URI")
            }
            _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
        };
    }

    #[test]
    fn test_issuer_object_id_must_not_be_empty() {
        let issuer = Issuer::Object(ObjectIssuer {
            id: "".to_string(),
            name: "Example Name".to_string(),
            additional_properties: None,
        });

        let result = create_vc(
            issuer,
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(err_msg, "issuer id must not be empty");
            }
            _ => panic!("Expected Web5Error::Parameter with specific error message"),
        };
    }

    #[test]
    fn test_issuer_object_id_must_be_valid_did() {
        let result = create_vc(
            issuer(),
            CredentialSubject {
                id: "did:something-invalid".to_string(),
                ..Default::default()
            },
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(
                    err_msg,
                    "credential subject must be a valid DID URI or start with 'urn:uuid:'"
                )
            }
            _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
        };
    }

    #[test]
    fn test_issuer_object_name_must_not_be_empty() {
        let issuer = Issuer::Object(ObjectIssuer {
            id: ISSUER_DID_URI.to_string(),
            name: "".to_string(),
            additional_properties: None,
        });

        let result = create_vc(
            issuer,
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(err_msg, "named issuer name must not be empty");
            }
            _ => panic!("Expected Web5Error::Parameter with specific error message"),
        };
    }

    #[test]
    fn test_issuer_object_must_be_set() {
        let issuer = Issuer::Object(ObjectIssuer {
            id: ISSUER_DID_URI.to_string(),
            name: "Example Name".to_string(),
            additional_properties: None,
        });

        let vc = create_vc(
            issuer.clone(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(vc.issuer, issuer);
    }

    #[test]
    fn test_issuer_object_supports_additional_properties() {
        let additional_properties = JsonObject {
            properties: HashMap::from([(
                "extra_key".to_string(),
                JsonValue::String("extra_value".to_string()),
            )]),
        };

        let issuer = Issuer::Object(ObjectIssuer {
            id: ISSUER_DID_URI.to_string(),
            name: "Example Name".to_string(),
            additional_properties: Some(additional_properties.clone()),
        });

        let vc = create_vc(
            issuer.clone(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        match vc.issuer {
            Issuer::Object(ref obj) => {
                assert_eq!(obj.additional_properties, Some(additional_properties));
            }
            _ => panic!("Issuer is not an ObjectIssuer"),
        };
    }

    #[test]
    fn test_credential_subject_id_must_not_be_empty() {
        let credential_subject = CredentialSubject::from("");

        let result = create_vc(
            issuer(),
            credential_subject,
            Some(VerifiableCredentialCreateOptions::default()),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(err_msg, "subject id must not be empty");
            }
            _ => panic!("Expected Web5Error::Parameter with specific error message"),
        };
    }

    #[test]
    fn test_credential_subject_must_be_set() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(vc.credential_subject, credential_subject());
    }

    #[test]
    fn test_credential_subject_supports_additional_properties() {
        let additional_properties = JsonObject {
            properties: HashMap::from([(
                "extra_key".to_string(),
                JsonValue::String("extra_value".to_string()),
            )]),
        };

        let credential_subject = CredentialSubject {
            id: SUBJECT_DID_URI.to_string(),
            additional_properties: Some(additional_properties.clone()),
        };

        let vc = create_vc(
            issuer(),
            credential_subject.clone(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        assert_eq!(
            vc.credential_subject.additional_properties,
            Some(additional_properties)
        );
    }

    #[test]
    fn test_issuance_date_must_be_set() {
        let issuance_date = SystemTime::now();

        let options = VerifiableCredentialCreateOptions {
            issuance_date: Some(issuance_date),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.issuance_date, issuance_date);
    }

    #[test]
    fn test_issuance_date_must_be_now_if_not_supplied() {
        let vc = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions::default()),
        )
        .unwrap();

        let now = SystemTime::now();
        let hundred_millis_ago = now - std::time::Duration::from_millis(100);

        assert!(vc.issuance_date >= hundred_millis_ago && vc.issuance_date <= now);
    }

    #[test]
    fn test_expiration_date_must_be_set_if_supplied() {
        let expiration_date = SystemTime::now();
        let options = VerifiableCredentialCreateOptions {
            expiration_date: Some(expiration_date),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(vc.expiration_date, Some(expiration_date));
    }

    #[test]
    fn test_evidence_must_be_set_if_supplied() {
        let mut evidence_item = JsonObject::new();
        evidence_item.insert_value("A Key", JsonValue::String("A Value".to_string()));
        let evidence = vec![evidence_item];

        let options = VerifiableCredentialCreateOptions {
            evidence: Some(evidence.clone()),
            ..Default::default()
        };

        let vc = create_vc(issuer(), credential_subject(), Some(options)).unwrap();

        assert_eq!(Some(evidence), vc.evidence);
    }

    #[test]
    fn test_schema_type_must_be_jsonschema() {
        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: Some(CredentialSchema {
                    id: "it doesn't matter".to_string(),
                    r#type: "invalid type".to_string(), // here
                }),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::Parameter(err_msg)) => {
                assert_eq!(format!("type must be {}", CREDENTIAL_SCHEMA_TYPE), err_msg)
            }
            _ => panic!(
                "expected Web5Error::Parameter with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_resolve_network_issue() {
        let url = "http://local".to_string(); // here

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::Http(err_msg)) => {
                assert!(err_msg.contains("get request failed"))
            }
            _ => panic!(
                "expected Web5Error::Http with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_resolve_non_success() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(500) // here
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::Http(err_msg)) => {
                assert!(err_msg.contains("http error status code 500"))
            }
            _ => panic!(
                "expected Web5Error::Http with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_resolve_invalid_response_body() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("invalid response body") // here
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::Http(err_msg)) => {
                assert!(err_msg.contains("failed to parse json"))
            }
            _ => panic!(
                "expected Web5Error::Http with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_invalid_json_schema() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&mock_json_schema(
                url.clone(),
                Some("this is not a valid $schema".to_string()), // here
            ))
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::JsonSchema(err_msg)) => {
                assert!(err_msg.contains("unable to compile json schema"))
            }
            _ => panic!(
                "expected Web5Error::JsonSchema with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_do_not_support_draft04() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&mock_json_schema(
                url.clone(),
                Some("http://json-schema.org/draft-04/schema#".to_string()), // here
            ))
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::JsonSchema(err_msg)) => {
                assert_eq!("draft unsupported Draft4", err_msg)
            }
            _ => panic!(
                "expected Web5Error::JsonSchema with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_do_not_support_draft06() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&mock_json_schema(
                url.clone(),
                Some("http://json-schema.org/draft-06/schema#".to_string()), // here
            ))
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(),
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::JsonSchema(err_msg)) => {
                assert_eq!("draft unsupported Draft6", err_msg)
            }
            _ => panic!(
                "expected Web5Error::JsonSchema with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_fails_validation() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&mock_json_schema(url.clone(), None))
            .create();

        let result = create_vc(
            issuer(),
            credential_subject(), // does not match schema
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        );

        match result {
            Err(Web5Error::JsonSchema(err_msg)) => {
                assert!(err_msg.contains("validation errors"))
            }
            _ => panic!(
                "expected Web5Error::JsonSchema with specific message but got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_schema_example_from_spec() {
        // using Example 1 & Example 2 from here https://www.w3.org/TR/vc-json-schema/#jsonschema

        let mut mock_server = Server::new();
        let url = mock_server.url();

        let _ = mock_server
            .mock("GET", "/schemas/email.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&mock_json_schema(url.clone(), None))
            .create();

        let mut additional_properties = JsonObject::new();
        additional_properties.insert_value(
            "emailAddress",
            JsonValue::String("alice@tbd.email".to_string()),
        );

        let _ = create_vc(
            issuer(),
            CredentialSubject {
                id: credential_subject().id,
                additional_properties: Some(additional_properties),
            },
            Some(VerifiableCredentialCreateOptions {
                credential_schema: mock_credential_schema(url),
                ..Default::default()
            }),
        )
        .unwrap();
    }
}
