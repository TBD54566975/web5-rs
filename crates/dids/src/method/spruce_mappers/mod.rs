use crate::{
    document::{DidDocument, Service, VerificationMethod},
    resolver::{DidDocumentMetadata, DidResolutionError, DidResolutionMetadata},
};
use crypto::key::public_key::PublicKey;
use ssi_core::one_or_many::OneOrMany;
use ssi_dids::{
    Context, Contexts, Document as SsiDocument, Service as SsiService, ServiceEndpoint,
    VerificationMethod as SsiVerificationMethod,
};

impl From<SsiDocument> for DidDocument {
    fn from(ssi_doc: SsiDocument) -> Self {
        let id = ssi_doc.id.clone();
        DidDocument {
            id: ssi_doc.id,
            context: match ssi_doc.context {
                Contexts::One(ctx) => match ctx {
                    Context::URI(uri) => Some(vec![uri.to_string()]),
                    Context::Object(obj) => {
                        Some(vec![serde_json::to_string(&obj).unwrap_or_default()])
                    }
                },
                Contexts::Many(ctxs) => Some(
                    ctxs.into_iter()
                        .map(|ctx| match ctx {
                            Context::URI(uri) => uri.to_string(),
                            Context::Object(obj) => serde_json::to_string(&obj).unwrap_or_default(),
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

impl From<SsiVerificationMethod> for VerificationMethod {
    fn from(ssi_vm: SsiVerificationMethod) -> Self {
        match ssi_vm {
            SsiVerificationMethod::Map(ssi_vmm) => VerificationMethod {
                id: ssi_vmm.id.clone(),
                r#type: ssi_vmm.type_.clone(),
                controller: ssi_vmm.controller.clone(),
                public_key_jwk: ssi_vmm.get_jwk().map(PublicKey::from).unwrap(),
            },
            _ => unimplemented!("Unsupported SsiVerificationMethod variant"),
        }
    }
}

impl From<SsiService> for Service {
    fn from(ssi_service: SsiService) -> Self {
        Service {
            id: ssi_service.id,
            r#type: match ssi_service.type_ {
                OneOrMany::One(t) => t,
                OneOrMany::Many(mut t) => t.pop().unwrap_or_default(),
            },
            service_endpoint: match ssi_service.service_endpoint {
                Some(OneOrMany::One(endpoint)) => match endpoint {
                    ServiceEndpoint::URI(uri) => uri,
                    ServiceEndpoint::Map(map) => serde_json::to_string(&map).unwrap_or_default(),
                },
                Some(OneOrMany::Many(endpoints)) => endpoints
                    .into_iter()
                    .last()
                    .map(|endpoint| match endpoint {
                        ServiceEndpoint::URI(uri) => uri,
                        ServiceEndpoint::Map(map) => {
                            serde_json::to_string(&map).unwrap_or_default()
                        }
                    })
                    .unwrap_or_default(),
                None => "".to_string(),
            },
        }
    }
}

impl From<ssi_dids::did_resolve::ResolutionMetadata> for DidResolutionMetadata {
    fn from(metadata: ssi_dids::did_resolve::ResolutionMetadata) -> Self {
        DidResolutionMetadata {
            error: metadata.error.map(|err| match err.as_str() {
                ssi_dids::did_resolve::ERROR_INVALID_DID => DidResolutionError::InvalidDid,
                ssi_dids::did_resolve::ERROR_NOT_FOUND => DidResolutionError::NotFound,
                ssi_dids::did_resolve::ERROR_REPRESENTATION_NOT_SUPPORTED => {
                    DidResolutionError::RepresentationNotSupported
                }
                ssi_dids::did_resolve::ERROR_METHOD_NOT_SUPPORTED => {
                    DidResolutionError::MethodNotSupported
                }
                _ => DidResolutionError::InternalError,
            }),
        }
    }
}

impl From<ssi_dids::did_resolve::DocumentMetadata> for DidDocumentMetadata {
    fn from(metadata: ssi_dids::did_resolve::DocumentMetadata) -> Self {
        DidDocumentMetadata {
            created: metadata.created.map(|dt| dt.to_rfc3339()),
            updated: metadata.updated.map(|dt| dt.to_rfc3339()),
            deactivated: metadata.deactivated,
            next_update: metadata.property_set.as_ref().and_then(|props| {
                props.get("nextUpdate").and_then(|value| match value {
                    ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                    _ => None,
                })
            }),
            version_id: metadata.property_set.as_ref().and_then(|props| {
                props.get("versionId").and_then(|value| match value {
                    ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                    _ => None,
                })
            }),
            next_version_id: metadata.property_set.as_ref().and_then(|props| {
                props.get("nextVersionId").and_then(|value| match value {
                    ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                    _ => None,
                })
            }),
            equivalent_id: metadata.property_set.as_ref().and_then(|props| {
                props.get("equivalentId").and_then(|value| match value {
                    ssi_dids::did_resolve::Metadata::List(list) => {
                        let mut ids = Vec::new();
                        for item in list {
                            if let ssi_dids::did_resolve::Metadata::String(s) = item {
                                ids.push(s.clone());
                            }
                        }
                        Some(ids)
                    }
                    _ => None,
                })
            }),
            canonical_id: metadata.property_set.as_ref().and_then(|props| {
                props.get("canonicalId").and_then(|value| match value {
                    ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                    _ => None,
                })
            }),
        }
    }
}
