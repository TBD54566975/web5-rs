use crate::resolver::DocumentMetadata;
use ssi_dids::did_resolve::DocumentMetadata as SpruceDocumentMetadata;

impl DocumentMetadata {
    pub fn from_spruce(spruce_document_metadata: SpruceDocumentMetadata) -> Result<Self, String> {
        Ok(DocumentMetadata {
            created: spruce_document_metadata.created.map(|dt| dt.to_rfc3339()),
            updated: spruce_document_metadata.updated.map(|dt| dt.to_rfc3339()),
            deactivated: spruce_document_metadata.deactivated,
            next_update: spruce_document_metadata
                .property_set
                .as_ref()
                .and_then(|props| {
                    props.get("nextUpdate").and_then(|value| match value {
                        ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                        _ => None,
                    })
                }),
            version_id: spruce_document_metadata
                .property_set
                .as_ref()
                .and_then(|props| {
                    props.get("versionId").and_then(|value| match value {
                        ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                        _ => None,
                    })
                }),
            next_version_id: spruce_document_metadata
                .property_set
                .as_ref()
                .and_then(|props| {
                    props.get("nextVersionId").and_then(|value| match value {
                        ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                        _ => None,
                    })
                }),
            equivalent_id: spruce_document_metadata
                .property_set
                .as_ref()
                .and_then(|props| {
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
            canonical_id: spruce_document_metadata
                .property_set
                .as_ref()
                .and_then(|props| {
                    props.get("canonicalId").and_then(|value| match value {
                        ssi_dids::did_resolve::Metadata::String(s) => Some(s.clone()),
                        _ => None,
                    })
                }),
        })
    }
}
