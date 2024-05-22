use crate::document::{Document, Service, VerificationMethod};
use jwk::Jwk;
use ssi_core::one_or_many::OneOrMany;
use ssi_dids::{
    Context as SpruceContext, Contexts as SpruceContexts, Document as SpruceDocument,
    Service as SpruceService, ServiceEndpoint as SpruceServiceEndpoint,
    VerificationMethod as SpruceVerificationMethod,
};
use ssi_jwk::{Algorithm, Params};

impl Document {
    pub fn from_spruce(spruce_document: SpruceDocument) -> Result<Self, String> {
        let context = match spruce_document.context {
            SpruceContexts::One(ctx) => match ctx {
                SpruceContext::URI(uri) => Some(vec![uri.to_string()]),
                SpruceContext::Object(obj) => {
                    Some(vec![serde_json::to_string(&obj).unwrap_or_default()])
                }
            },
            SpruceContexts::Many(ctxs) => Some(
                ctxs.into_iter()
                    .map(|ctx| match ctx {
                        SpruceContext::URI(uri) => uri.to_string(),
                        SpruceContext::Object(obj) => {
                            serde_json::to_string(&obj).unwrap_or_default()
                        }
                    })
                    .collect(),
            ),
        };

        let verification_methods = spruce_document
            .verification_method
            .unwrap_or_default()
            .into_iter()
            .map(VerificationMethod::from_spruce)
            .collect::<Result<Vec<_>, String>>()?;

        let authentication = spruce_document.authentication.map(|vms| {
            vms.into_iter()
                .map(|vm| vm.get_id(&spruce_document.id))
                .collect::<Vec<_>>()
        });

        let assertion_method = spruce_document.assertion_method.map(|vms| {
            vms.into_iter()
                .map(|vm| vm.get_id(&spruce_document.id))
                .collect::<Vec<_>>()
        });

        let key_agreement = spruce_document.key_agreement.map(|vms| {
            vms.into_iter()
                .map(|vm| vm.get_id(&spruce_document.id))
                .collect::<Vec<_>>()
        });

        let capability_invocation = spruce_document.capability_invocation.map(|vms| {
            vms.into_iter()
                .map(|vm| vm.get_id(&spruce_document.id))
                .collect::<Vec<_>>()
        });

        let capability_delegation = spruce_document.capability_delegation.map(|vms| {
            vms.into_iter()
                .map(|vm| vm.get_id(&spruce_document.id))
                .collect::<Vec<_>>()
        });

        let service = spruce_document
            .service
            .map(|services| {
                services
                    .into_iter()
                    .map(Service::from_spruce)
                    .collect::<Result<Vec<_>, String>>()
            })
            .transpose()?;

        Ok(Document {
            id: spruce_document.id,
            context,
            controller: spruce_document
                .controller
                .map(|c| c.into_iter().map(|did| did.to_string()).collect()),
            also_known_as: spruce_document
                .also_known_as
                .map(|aka| aka.into_iter().map(|uri| uri.to_string()).collect()),
            verification_method: verification_methods,
            authentication,
            assertion_method,
            key_agreement,
            capability_invocation,
            capability_delegation,
            service,
        })
    }
}

impl VerificationMethod {
    pub fn from_spruce(
        spruce_verification_method: SpruceVerificationMethod,
    ) -> Result<Self, String> {
        match spruce_verification_method {
            SpruceVerificationMethod::Map(ssi_vmm) => {
                let spruce_jwk = ssi_vmm.get_jwk()?;
                let alg = match spruce_jwk.algorithm {
                    Some(Algorithm::ES256K) => "ES256K",
                    Some(Algorithm::EdDSA) => "EdDSA",
                    _ => "",
                }
                .to_string();
                let (kty, crv, x, y) = match &spruce_jwk.params {
                    Params::EC(ec_params) => (
                        "EC",
                        ec_params.curve.clone(),
                        ec_params.x_coordinate.clone(),
                        ec_params.y_coordinate.clone(),
                    ),
                    Params::RSA(_) => ("RSA", None, None, None),
                    Params::Symmetric(_) => ("oct", None, None, None),
                    Params::OKP(okp_params) => (
                        "OKP",
                        Some(okp_params.curve.clone()),
                        Some(okp_params.public_key.clone()),
                        None,
                    ),
                };
                let jwk = Jwk {
                    alg,
                    kty: kty.to_string(),
                    crv: crv.unwrap_or("".to_string()),
                    x: x.as_ref()
                        .map(String::from)
                        .unwrap_or_else(|| "".to_string()),
                    y: y.map(String::from),
                    ..Default::default()
                };

                Ok(VerificationMethod {
                    id: ssi_vmm.id,
                    r#type: ssi_vmm.type_,
                    controller: ssi_vmm.controller,
                    public_key_jwk: jwk,
                })
            }
            _ => Err("Unsupported SpruceVerificationMethod variant".to_string()),
        }
    }
}

impl Service {
    pub fn from_spruce(spruce_service: SpruceService) -> Result<Self, String> {
        let r#type = match spruce_service.type_ {
            OneOrMany::One(t) => t,
            OneOrMany::Many(mut t) => t
                .pop()
                .ok_or_else(|| "Service type array was empty".to_string())?,
        };

        let service_endpoint = match spruce_service.service_endpoint {
            Some(OneOrMany::One(endpoint)) => match endpoint {
                SpruceServiceEndpoint::URI(uri) => vec![uri],
                SpruceServiceEndpoint::Map(map) => {
                    vec![serde_json::to_string(&map).unwrap_or_default()]
                }
            },
            Some(OneOrMany::Many(endpoints)) => endpoints
                .into_iter()
                .map(|endpoint| match endpoint {
                    SpruceServiceEndpoint::URI(uri) => uri,
                    SpruceServiceEndpoint::Map(map) => {
                        serde_json::to_string(&map).unwrap_or_default()
                    }
                })
                .collect(),
            None => return Err("Service endpoint is missing".to_string()),
        };

        Ok(Service {
            id: spruce_service.id,
            r#type,
            service_endpoint,
        })
    }
}

#[cfg(test)]
mod tests {
    use ssi_core::one_or_many::OneOrMany;
    use ssi_dids::ServiceEndpoint;

    use super::*;

    #[test]
    fn test_document_from_spruce() {
        let spruce_document_str = r##"{
            "@context": [
              "https://www.w3.org/ns/did/v1",
              "https://w3id.org/security/suites/jws-2020/v1"
            ],
            "id": "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9",
            "verificationMethod": [
              {
                "id": "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0",
                "type": "JsonWebKey2020",
                "controller": "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9",
                "publicKeyJwk": {
                  "alg": "EdDSA",
                  "kty": "OKP",
                  "crv": "Ed25519",
                  "x": "V_gejRDkNyOIL2J1uaFuDYstaB15Zbmg5mckR6GCb4M"
                }
              }
            ],
            "authentication": [
              "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0"
            ],
            "assertionMethod": [
              "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0"
            ],
            "keyAgreement": [
              "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0"
            ],
            "capabilityInvocation": [
              "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0"
            ],
            "capabilityDelegation": [
              "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9#0"
            ]
          }"##;
        let spruce_document: SpruceDocument = serde_json::from_str(&spruce_document_str).unwrap();
        let document = Document::from_spruce(spruce_document).unwrap();

        let expected_did_uri = "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoiVl9nZWpSRGtOeU9JTDJKMXVhRnVEWXN0YUIxNVpibWc1bWNrUjZHQ2I0TSJ9".to_string();

        assert_eq!(document.id, expected_did_uri);
        assert_eq!(
            document.context,
            Some(vec![
                "https://www.w3.org/ns/did/v1".to_string(),
                "https://w3id.org/security/suites/jws-2020/v1".to_string()
            ])
        );
        assert_eq!(document.controller, None);
        assert_eq!(document.also_known_as, None);
        assert_eq!(document.verification_method.len(), 1);

        let vm = &document.verification_method[0];
        assert_eq!(vm.id, format!("{}#0", expected_did_uri));
        assert_eq!(vm.r#type, "JsonWebKey2020".to_string());
        assert_eq!(vm.controller, expected_did_uri);
        assert_eq!(vm.public_key_jwk.alg, "EdDSA".to_string());
        assert_eq!(vm.public_key_jwk.kty, "OKP".to_string());
        assert_eq!(vm.public_key_jwk.crv, "Ed25519".to_string());
        assert_eq!(
            vm.public_key_jwk.x,
            "V_gejRDkNyOIL2J1uaFuDYstaB15Zbmg5mckR6GCb4M".to_string()
        );
        assert_eq!(vm.public_key_jwk.y, None);
    }

    #[test]
    fn test_service_from_spruce() {
        let spruce_service = SpruceService {
            id: "did:example:123#service1".to_string(),
            type_: OneOrMany::One("Example".to_string()),
            service_endpoint: Some(OneOrMany::One(ServiceEndpoint::URI(
                "https://example.com/service1".to_string(),
            ))),
            property_set: None,
        };
        let service = Service::from_spruce(spruce_service).unwrap();

        assert_eq!(service.id, "did:example:123#service1".to_string());
        assert_eq!(service.r#type, "Example".to_string());
        assert_eq!(
            service.service_endpoint,
            vec!["https://example.com/service1".to_string()]
        );
    }
}
