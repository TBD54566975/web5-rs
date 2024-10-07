use crate::errors::{map_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    dids::data_model::{
        document::Document, service::Service, verification_method::VerificationMethod,
    },
    json::{FromJson, ToJson},
};
use crate::crypto::jwk::WasmJwk;

#[wasm_bindgen]
pub struct WasmDocument {
    inner: Document,
}

impl From<WasmDocument> for Document {
    fn from(value: WasmDocument) -> Self {
        value.inner
    }
}

impl From<Document> for WasmDocument {
    fn from(value: Document) -> Self {
        WasmDocument { inner: value }
    }
}

#[wasm_bindgen]
impl WasmDocument {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        context: Option<Vec<String>>,
        controller: Option<Vec<String>>,
        also_known_as: Option<Vec<String>>,
        verification_method: Vec<WasmVerificationMethod>,
        authentication: Option<Vec<String>>,
        assertion_method: Option<Vec<String>>,
        key_agreement: Option<Vec<String>>,
        capability_invocation: Option<Vec<String>>,
        capability_delegation: Option<Vec<String>>,
        service: Option<Vec<WasmService>>,
    ) -> Self {
        Self {
            inner: Document {
                id,
                context,
                controller,
                also_known_as,
                verification_method: verification_method
                    .into_iter()
                    .map(|wvm| wvm.into())
                    .collect(),
                authentication,
                assertion_method,
                key_agreement,
                capability_invocation,
                capability_delegation,
                service: service.map(|wss| wss.into_iter().map(|ws| ws.into()).collect()),
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_json_string(json: &str) -> Result<WasmDocument> {
        Ok(Self {
            inner: Document::from_json_string(json).map_err(map_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn context(&self) -> Option<Vec<String>> {
        self.inner.context.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn controller(&self) -> Option<Vec<String>> {
        self.inner.controller.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn also_known_as(&self) -> Option<Vec<String>> {
        self.inner.also_known_as.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn verification_method(&self) -> Vec<WasmVerificationMethod> {
        self.inner
            .verification_method
            .clone()
            .into_iter()
            .map(|vm| vm.into())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn authentication(&self) -> Option<Vec<String>> {
        self.inner.authentication.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn assertion_method(&self) -> Option<Vec<String>> {
        self.inner.assertion_method.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn key_agreement(&self) -> Option<Vec<String>> {
        self.inner.key_agreement.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn capability_invocation(&self) -> Option<Vec<String>> {
        self.inner.capability_invocation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn capability_delegation(&self) -> Option<Vec<String>> {
        self.inner.capability_delegation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn service(&self) -> Option<Vec<WasmService>> {
        self.inner
            .service
            .clone()
            .map(|services| services.into_iter().map(|s| s.into()).collect())
    }
}

#[wasm_bindgen]
pub struct WasmVerificationMethod {
    inner: VerificationMethod,
}

impl From<WasmVerificationMethod> for VerificationMethod {
    fn from(value: WasmVerificationMethod) -> Self {
        value.inner
    }
}

impl From<VerificationMethod> for WasmVerificationMethod {
    fn from(value: VerificationMethod) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmVerificationMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, r#type: String, controller: String, public_key_jwk: WasmJwk) -> Self {
        Self {
            inner: VerificationMethod {
                id,
                r#type,
                controller,
                public_key_jwk: public_key_jwk.into(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn r#type(&self) -> String {
        self.inner.r#type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn controller(&self) -> String {
        self.inner.controller.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key_jwk(&self) -> WasmJwk {
        self.inner.public_key_jwk.clone().into()
    }
}

#[wasm_bindgen]
pub struct WasmService {
    inner: Service,
}

impl From<WasmService> for Service {
    fn from(value: WasmService) -> Self {
        value.inner
    }
}

impl From<Service> for WasmService {
    fn from(value: Service) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmService {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, r#type: String, service_endpoint: Vec<String>) -> Self {
        Self {
            inner: Service {
                id,
                r#type,
                service_endpoint,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn r#type(&self) -> String {
        self.inner.r#type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn service_endpoint(&self) -> Vec<String> {
        self.inner.service_endpoint.clone()
    }
}
