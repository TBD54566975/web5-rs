use dids::identifier::Identifier as InternalIdentifier;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Identifier(InternalIdentifier);

impl From<InternalIdentifier> for Identifier {
    fn from(value: InternalIdentifier) -> Self {
        Self(value)
    }
}

impl From<&Identifier> for InternalIdentifier {
    fn from(value: &Identifier) -> Self {
        Self {
            uri: value.0.uri.clone(),
            url: value.0.url.clone(),
            method: value.0.method.clone(),
            id: value.0.id.clone(),
            params: value.0.params.clone(),
            path: value.0.path.clone(),
            query: value.0.query.clone(),
            fragment: value.0.fragment.clone(),
        }
    }
}

#[wasm_bindgen]
impl Identifier {
    #[wasm_bindgen]
    pub fn parse(did_uri: String) -> Result<Identifier, JsValue> {
        let internal_identifier = InternalIdentifier::parse(&did_uri)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(Identifier::from(internal_identifier))
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.0.uri.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.0.url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.0.method.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.0.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn params(&self) -> JsValue {
        match &self.0.params {
            Some(params) => {
                let obj = Object::new();
                for (key, value) in params {
                    Reflect::set(&obj, &JsValue::from_str(key), &JsValue::from_str(value))
                        .unwrap();
                }
                obj.into()
            }
            None => JsValue::UNDEFINED,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<String> {
        self.0.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn query(&self) -> Option<String> {
        self.0.query.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn fragment(&self) -> Option<String> {
        self.0.fragment.clone()
    }
}
