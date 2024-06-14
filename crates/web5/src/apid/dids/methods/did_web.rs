use crate::apid::dids::{did::Did, document::Document, resolution_result::ResolutionResult};

#[derive(Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn from_uri(uri: &str) -> Self {
        let resolution_result = DidWeb::resolve(uri);
        match resolution_result.document {
            None => panic!(),
            Some(document) => {
                let identifer = Did::new(uri).unwrap();
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
