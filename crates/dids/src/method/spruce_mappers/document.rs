use crate::document::{DidDocument, Service, VerificationMethod};
use crypto::key::public_key::PublicKey;
use ssi_core::one_or_many::OneOrMany;
use ssi_dids::{
    Context as SpruceContext, Contexts as SpruceContexts, Document as SpruceDocument,
    Service as SpruceService, ServiceEndpoint as SpruceServiceEndpoint,
    VerificationMethod as SpruceVerificationMethod,
};

impl From<SpruceDocument> for DidDocument {
    fn from(ssi_doc: SpruceDocument) -> Self {
        let id = ssi_doc.id.clone();
        DidDocument {
            id: ssi_doc.id,
            context: match ssi_doc.context {
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
            },
            controller: ssi_doc
                .controller
                .map(|c| c.into_iter().map(|did| did.to_string()).collect()),
            also_known_as: ssi_doc
                .also_known_as
                .map(|aka| aka.into_iter().map(|uri| uri.to_string()).collect()),
            verification_method: ssi_doc
                .verification_method
                .unwrap_or_default()
                .into_iter()
                .map(VerificationMethod::from)
                .collect(),
            authentication: ssi_doc
                .authentication
                .map(|auth| auth.into_iter().map(|vm| vm.get_id(&id)).collect()),
            assertion_method: ssi_doc
                .assertion_method
                .map(|am| am.into_iter().map(|vm| vm.get_id(&id)).collect()),
            key_agreement: ssi_doc
                .key_agreement
                .map(|ka| ka.into_iter().map(|vm| vm.get_id(&id)).collect()),
            capability_invocation: ssi_doc
                .capability_invocation
                .map(|ci| ci.into_iter().map(|vm| vm.get_id(&id)).collect()),
            capability_delegation: ssi_doc
                .capability_delegation
                .map(|cd| cd.into_iter().map(|vm| vm.get_id(&id)).collect()),
            service: ssi_doc
                .service
                .map(|s| s.into_iter().map(Service::from).collect()),
        }
    }
}

impl From<SpruceVerificationMethod> for VerificationMethod {
    fn from(ssi_vm: SpruceVerificationMethod) -> Self {
        match ssi_vm {
            SpruceVerificationMethod::Map(ssi_vmm) => VerificationMethod {
                id: ssi_vmm.id.clone(),
                r#type: ssi_vmm.type_.clone(),
                controller: ssi_vmm.controller.clone(),
                public_key_jwk: ssi_vmm.get_jwk().map(PublicKey::from).unwrap(),
            },
            _ => unimplemented!("Unsupported SsiVerificationMethod variant"),
        }
    }
}

impl From<SpruceService> for Service {
    fn from(ssi_service: SpruceService) -> Self {
        Service {
            id: ssi_service.id,
            r#type: match ssi_service.type_ {
                OneOrMany::One(t) => t,
                OneOrMany::Many(mut t) => t.pop().unwrap_or_default(),
            },
            service_endpoint: match ssi_service.service_endpoint {
                Some(OneOrMany::One(endpoint)) => match endpoint {
                    SpruceServiceEndpoint::URI(uri) => uri,
                    SpruceServiceEndpoint::Map(map) => {
                        serde_json::to_string(&map).unwrap_or_default()
                    }
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
                    .unwrap_or_default(),
                None => "".to_string(),
            },
        }
    }
}
