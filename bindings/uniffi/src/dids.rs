use crate::inner::dids::{
    Did as InnerDid, Document as InnerDocument, DocumentMetadata as InnerDocumentMetadata,
    ResolutionMetadata as InnerResolutionMetadata, ResolutionMetadataError,
    ResolutionResult as InnerResolutionResult, Service as InnerService,
    VerificationMethod as InnerVerificationMethod,
};
use crate::keys::Jwk;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// 🚧 too many unwrap()'s

pub struct Did(Arc<RwLock<InnerDid>>);

impl Did {
    pub fn new(uri: &str) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerDid::new(uri))),
        }
    }

    pub fn get_uri(&self) -> String {
        self.0.read().unwrap().uri.clone()
    }

    pub fn set_uri(&self, uri: String) {
        let mut did = self.0.write().unwrap();
        did.uri = uri;
    }

    pub fn get_url(&self) -> String {
        self.0.read().unwrap().url.clone()
    }

    pub fn set_url(&self, url: String) {
        let mut did = self.0.write().unwrap();
        did.url = url;
    }

    pub fn get_method(&self) -> String {
        self.0.read().unwrap().method.clone()
    }

    pub fn set_method(&self, method: String) {
        let mut did = self.0.write().unwrap();
        did.method = method;
    }

    pub fn get_id(&self) -> String {
        self.0.read().unwrap().id.clone()
    }

    pub fn set_id(&self, id: String) {
        let mut did = self.0.write().unwrap();
        did.id = id;
    }

    pub fn get_params(&self) -> Option<HashMap<String, String>> {
        self.0.read().unwrap().params.clone()
    }

    pub fn set_params(&self, params: Option<HashMap<String, String>>) {
        let mut did = self.0.write().unwrap();
        did.params = params;
    }

    pub fn get_path(&self) -> Option<String> {
        self.0.read().unwrap().path.clone()
    }

    pub fn set_path(&self, path: Option<String>) {
        let mut did = self.0.write().unwrap();
        did.path = path;
    }

    pub fn get_query(&self) -> Option<String> {
        self.0.read().unwrap().query.clone()
    }

    pub fn set_query(&self, query: Option<String>) {
        let mut did = self.0.write().unwrap();
        did.query = query;
    }

    pub fn get_fragment(&self) -> Option<String> {
        self.0.read().unwrap().fragment.clone()
    }

    pub fn set_fragment(&self, fragment: Option<String>) {
        let mut did = self.0.write().unwrap();
        did.fragment = fragment;
    }
}

pub struct Document(Arc<RwLock<InnerDocument>>);

impl Document {
    pub fn new(
        id: String,
        context: Option<Vec<String>>,
        controller: Option<Vec<String>>,
        also_known_as: Option<Vec<String>>,
        verification_method: Vec<Arc<VerificationMethod>>,
        authentication: Option<Vec<String>>,
        assertion_method: Option<Vec<String>>,
        key_agreement: Option<Vec<String>>,
        capability_invocation: Option<Vec<String>>,
        capability_delegation: Option<Vec<String>>,
        service: Option<Vec<Arc<Service>>>,
    ) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerDocument {
                id,
                context,
                controller,
                also_known_as,
                verification_method: verification_method
                    .into_iter()
                    .map(|vm| vm.to_inner())
                    .collect(),
                authentication,
                assertion_method,
                key_agreement,
                capability_invocation,
                capability_delegation,
                service: service.map(|s| s.into_iter().map(|srv| srv.to_inner()).collect()),
            })),
        }
    }

    pub fn from_inner(inner_document: InnerDocument) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner_document)),
        }
    }

    pub fn to_inner(&self) -> InnerDocument {
        self.0.read().unwrap().clone()
    }

    pub fn get_id(&self) -> String {
        self.0.read().unwrap().id.clone()
    }

    pub fn set_id(&self, id: String) {
        let mut doc = self.0.write().unwrap();
        doc.id = id;
    }

    pub fn get_context(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().context.clone()
    }

    pub fn set_context(&self, context: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.context = context;
    }

    pub fn get_controller(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().controller.clone()
    }

    pub fn set_controller(&self, controller: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.controller = controller;
    }

    pub fn get_also_known_as(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().also_known_as.clone()
    }

    pub fn set_also_known_as(&self, also_known_as: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.also_known_as = also_known_as;
    }

    pub fn get_verification_method(&self) -> Vec<Arc<VerificationMethod>> {
        self.0
            .read()
            .unwrap()
            .verification_method
            .iter()
            .map(|vm| Arc::new(VerificationMethod::from_inner(vm.clone())))
            .collect()
    }

    pub fn set_verification_method(&self, verification_method: Vec<Arc<VerificationMethod>>) {
        let mut doc = self.0.write().unwrap();
        doc.verification_method = verification_method
            .into_iter()
            .map(|vm| vm.to_inner())
            .collect();
    }

    pub fn get_authentication(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().authentication.clone()
    }

    pub fn set_authentication(&self, authentication: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.authentication = authentication;
    }

    pub fn get_assertion_method(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().assertion_method.clone()
    }

    pub fn set_assertion_method(&self, assertion_method: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.assertion_method = assertion_method;
    }

    pub fn get_key_agreement(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().key_agreement.clone()
    }

    pub fn set_key_agreement(&self, key_agreement: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.key_agreement = key_agreement;
    }

    pub fn get_capability_invocation(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().capability_invocation.clone()
    }

    pub fn set_capability_invocation(&self, capability_invocation: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.capability_invocation = capability_invocation;
    }

    pub fn get_capability_delegation(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().capability_delegation.clone()
    }

    pub fn set_capability_delegation(&self, capability_delegation: Option<Vec<String>>) {
        let mut doc = self.0.write().unwrap();
        doc.capability_delegation = capability_delegation;
    }

    pub fn get_service(&self) -> Option<Vec<Arc<Service>>> {
        self.0.read().unwrap().service.as_ref().map(|services| {
            services
                .iter()
                .map(|s| Arc::new(Service::from_inner(s.clone())))
                .collect()
        })
    }

    pub fn set_service(&self, service: Option<Vec<Arc<Service>>>) {
        let mut doc = self.0.write().unwrap();
        doc.service = service.map(|services| services.into_iter().map(|s| s.to_inner()).collect());
    }
}

pub struct VerificationMethod(Arc<RwLock<InnerVerificationMethod>>);

impl VerificationMethod {
    pub fn new(id: String, r#type: String, controller: String, public_key_jwk: Arc<Jwk>) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerVerificationMethod {
                id,
                r#type,
                controller,
                public_key_jwk: public_key_jwk.to_inner(),
            })),
        }
    }

    pub fn from_inner(inner_verification_method: InnerVerificationMethod) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner_verification_method)),
        }
    }

    pub fn to_inner(&self) -> InnerVerificationMethod {
        self.0.read().unwrap().clone()
    }

    pub fn get_id(&self) -> String {
        self.0.read().unwrap().id.clone()
    }

    pub fn set_id(&self, id: String) {
        let mut vm = self.0.write().unwrap();
        vm.id = id;
    }

    pub fn get_type(&self) -> String {
        self.0.read().unwrap().r#type.clone()
    }

    pub fn set_type(&self, r#type: String) {
        let mut vm = self.0.write().unwrap();
        vm.r#type = r#type;
    }

    pub fn get_controller(&self) -> String {
        self.0.read().unwrap().controller.clone()
    }

    pub fn set_controller(&self, controller: String) {
        let mut vm = self.0.write().unwrap();
        vm.controller = controller;
    }

    pub fn get_public_key_jwk(&self) -> Arc<Jwk> {
        let inner_jwk = self.0.read().unwrap().public_key_jwk.clone();
        Arc::new(Jwk::from_inner(inner_jwk))
    }

    pub fn set_public_key_jwk(&self, public_key_jwk: Arc<Jwk>) {
        let mut vm = self.0.write().unwrap();
        vm.public_key_jwk = public_key_jwk.to_inner();
    }
}

pub struct Service(Arc<RwLock<InnerService>>);

impl Service {
    pub fn new(id: String, r#type: String, service_endpoint: Vec<String>) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerService {
                id,
                r#type,
                service_endpoint,
            })),
        }
    }

    pub fn from_inner(inner_service: InnerService) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner_service)),
        }
    }

    pub fn to_inner(&self) -> InnerService {
        self.0.read().unwrap().clone()
    }

    pub fn get_id(&self) -> String {
        self.0.read().unwrap().id.clone()
    }

    pub fn set_id(&self, id: String) {
        let mut service = self.0.write().unwrap();
        service.id = id;
    }

    pub fn get_type(&self) -> String {
        self.0.read().unwrap().r#type.clone()
    }

    pub fn set_type(&self, r#type: String) {
        let mut service = self.0.write().unwrap();
        service.r#type = r#type;
    }

    pub fn get_service_endpoint(&self) -> Vec<String> {
        self.0.read().unwrap().service_endpoint.clone()
    }

    pub fn set_service_endpoint(&self, service_endpoint: Vec<String>) {
        let mut service = self.0.write().unwrap();
        service.service_endpoint = service_endpoint;
    }
}

pub struct ResolutionMetadata(Arc<RwLock<InnerResolutionMetadata>>);

impl ResolutionMetadata {
    pub fn new(error: ResolutionMetadataError) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerResolutionMetadata { error })),
        }
    }

    pub fn from_inner(inner: InnerResolutionMetadata) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn to_inner(&self) -> InnerResolutionMetadata {
        self.0.read().unwrap().clone()
    }

    pub fn get_error(&self) -> ResolutionMetadataError {
        self.0.read().unwrap().error.clone()
    }

    pub fn set_error(&self, error: ResolutionMetadataError) {
        let mut inner = self.0.write().unwrap();
        inner.error = error;
    }
}

pub struct DocumentMetadata(Arc<RwLock<InnerDocumentMetadata>>);

impl DocumentMetadata {
    pub fn new(
        created: Option<String>,
        updated: Option<String>,
        deactivated: Option<bool>,
        next_update: Option<String>,
        version_id: Option<String>,
        next_version_id: Option<String>,
        equivalent_id: Option<Vec<String>>,
        canonical_id: Option<String>,
    ) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerDocumentMetadata {
                created,
                updated,
                deactivated,
                next_update,
                version_id,
                next_version_id,
                equivalent_id,
                canonical_id,
            })),
        }
    }

    pub fn from_inner(inner: InnerDocumentMetadata) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn to_inner(&self) -> InnerDocumentMetadata {
        self.0.read().unwrap().clone()
    }

    pub fn get_created(&self) -> Option<String> {
        self.0.read().unwrap().created.clone()
    }

    pub fn set_created(&self, created: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.created = created;
    }

    pub fn get_updated(&self) -> Option<String> {
        self.0.read().unwrap().updated.clone()
    }

    pub fn set_updated(&self, updated: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.updated = updated;
    }

    pub fn get_deactivated(&self) -> Option<bool> {
        self.0.read().unwrap().deactivated.clone()
    }

    pub fn set_deactivated(&self, deactivated: Option<bool>) {
        let mut inner = self.0.write().unwrap();
        inner.deactivated = deactivated;
    }

    pub fn get_next_update(&self) -> Option<String> {
        self.0.read().unwrap().next_update.clone()
    }

    pub fn set_next_update(&self, next_update: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.next_update = next_update;
    }

    pub fn get_version_id(&self) -> Option<String> {
        self.0.read().unwrap().version_id.clone()
    }

    pub fn set_version_id(&self, version_id: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.version_id = version_id;
    }

    pub fn get_next_version_id(&self) -> Option<String> {
        self.0.read().unwrap().next_version_id.clone()
    }

    pub fn set_next_version_id(&self, next_version_id: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.next_version_id = next_version_id;
    }

    pub fn get_equivalent_id(&self) -> Option<Vec<String>> {
        self.0.read().unwrap().equivalent_id.clone()
    }

    pub fn set_equivalent_id(&self, equivalent_id: Option<Vec<String>>) {
        let mut inner = self.0.write().unwrap();
        inner.equivalent_id = equivalent_id;
    }

    pub fn get_canonical_id(&self) -> Option<String> {
        self.0.read().unwrap().canonical_id.clone()
    }

    pub fn set_canonical_id(&self, canonical_id: Option<String>) {
        let mut inner = self.0.write().unwrap();
        inner.canonical_id = canonical_id;
    }
}

pub struct ResolutionResult(Arc<RwLock<InnerResolutionResult>>);

impl ResolutionResult {
    pub fn new(
        document: Arc<Document>,
        document_metadata: Arc<DocumentMetadata>,
        resolution_metadata: Arc<ResolutionMetadata>,
    ) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerResolutionResult {
                document: document.to_inner(),
                document_metadata: document_metadata.to_inner(),
                resolution_metadata: resolution_metadata.to_inner(),
            })),
        }
    }

    pub fn get_document(&self) -> Arc<Document> {
        Arc::new(Document::from_inner(
            self.0.read().unwrap().document.clone(),
        ))
    }

    pub fn set_document(&self, document: Arc<Document>) {
        let mut inner = self.0.write().unwrap();
        inner.document = document.to_inner();
    }

    pub fn get_document_metadata(&self) -> Arc<DocumentMetadata> {
        Arc::new(DocumentMetadata::from_inner(
            self.0.read().unwrap().document_metadata.clone(),
        ))
    }

    pub fn set_document_metadata(&self, document_metadata: Arc<DocumentMetadata>) {
        let mut inner = self.0.write().unwrap();
        inner.document_metadata = document_metadata.to_inner();
    }

    pub fn get_resolution_metadata(&self) -> Arc<ResolutionMetadata> {
        Arc::new(ResolutionMetadata::from_inner(
            self.0.read().unwrap().resolution_metadata.clone(),
        ))
    }

    pub fn set_resolution_metadata(&self, resolution_metadata: Arc<ResolutionMetadata>) {
        let mut inner = self.0.write().unwrap();
        inner.resolution_metadata = resolution_metadata.to_inner();
    }
}
