use crate::resolver::DidDocumentMetadata;

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
