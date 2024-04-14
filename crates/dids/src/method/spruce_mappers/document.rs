use crate::document::{Document, Service, VerificationMethod};
use jwk::Jwk;
use ssi_core::one_or_many::OneOrMany;
use ssi_dids::{
    Context as SpruceContext, Contexts as SpruceContexts, Document as SpruceDocument,
    Service as SpruceService, ServiceEndpoint as SpruceServiceEndpoint,
    VerificationMethod as SpruceVerificationMethod,
};
use ssi_jwk::Params;

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

        // Handle the possible error when converting each `SpruceService` to `Service`
        let service = spruce_document
            .service
            .map(|services| {
                services
                    .into_iter()
                    .map(Service::from_spruce)
                    .collect::<Result<Vec<_>, String>>()
            })
            .transpose()?; // Use transpose to convert Option<Result<T, E>> to Result<Option<T>, E>

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
                let alg = spruce_jwk
                    .algorithm
                    .ok_or("spruce alg missing".to_string())?;
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
                    alg: format!("{:?}", alg),
                    kty: kty.to_string(),
                    crv: crv.unwrap_or("".to_string()),
                    x: x.as_ref()
                        .map(|b64| String::from(b64))
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
                SpruceServiceEndpoint::URI(uri) => uri,
                SpruceServiceEndpoint::Map(map) => serde_json::to_string(&map).unwrap_or_default(),
            },
            Some(OneOrMany::Many(endpoints)) => endpoints
                .into_iter()
                .last()
                .map(|endpoint| match endpoint {
                    SpruceServiceEndpoint::URI(uri) => uri,
                    SpruceServiceEndpoint::Map(map) => {
                        serde_json::to_string(&map).unwrap_or_default()
                    }
                })
                .ok_or_else(|| "Service endpoint array was empty".to_string())?,
            None => return Err("Service endpoint is missing".to_string()),
        };

        Ok(Service {
            id: spruce_service.id,
            r#type,
            service_endpoint,
        })
    }
}
