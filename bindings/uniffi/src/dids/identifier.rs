use dids::identifier::{Identifier as InternalIdentifier, IdentifierError};

#[derive(Default)]
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

impl Identifier {
    pub fn new(did_uri: String) -> Result<Self, IdentifierError> {
        let internal_identifier = InternalIdentifier::parse(&did_uri)?;
        Ok(Identifier::from(internal_identifier))
    }
}
