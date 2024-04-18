use dids::identifier::{Identifier, IdentifierError};

pub fn identifier_parse(did_uri: String) -> Result<Identifier, IdentifierError> {
    Identifier::parse(&did_uri)
}
