use crate::apid::dids::{
    document::Document, identifier::Identifier, resolution_result::ResolutionResult,
};

pub struct DidWeb {
    pub did: Identifier,
    pub document: Document,
}

impl DidWeb {
    pub fn new(uri: &str) -> Self {
        let resolution_result = DidWeb::resolve(uri);
        match resolution_result.document {
            None => panic!(),
            Some(document) => {
                let identifer = Identifier::new(uri).unwrap();
                Self {
                    did: identifer,
                    document,
                }
            }
        }
    }

    pub fn resolve(uri: &str) -> ResolutionResult {
        // 🚧 use existing PR which replaces spruce dep
        unimplemented!()
    }
}
