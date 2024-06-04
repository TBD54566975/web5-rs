use crate::dids::resolver::DocumentMetadata;
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

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use ssi_dids::did_resolve::Metadata;
    use std::collections::HashMap;

    #[test]
    fn test_from_spruce() {
        let now = Utc::now();
        let now_string = now.to_rfc3339();
        let version_id = "v1.0".to_string();
        let next_version_id = "v1.1".to_string();
        let canonical_id = "cid123".to_string();
        let equivalent_id = "eid123".to_string();
        let mut property_set = HashMap::new();
        property_set.insert(
            "nextUpdate".to_string(),
            Metadata::String(now_string.clone()),
        );
        property_set.insert(
            "versionId".to_string(),
            Metadata::String(version_id.clone()),
        );
        property_set.insert(
            "nextVersionId".to_string(),
            Metadata::String(next_version_id.clone()),
        );
        property_set.insert(
            "canonicalId".to_string(),
            Metadata::String(canonical_id.clone()),
        );
        property_set.insert(
            "equivalentId".to_string(),
            Metadata::List(vec![Metadata::String(equivalent_id.clone())]),
        );

        let spruce_document_metadata = SpruceDocumentMetadata {
            created: Some(now),
            updated: Some(now),
            deactivated: Some(false),
            property_set: Some(property_set),
        };
        let document_metadata = DocumentMetadata::from_spruce(spruce_document_metadata)
            .expect("failed to map spruce document metadata");

        assert_eq!(document_metadata.created, Some(now_string.clone()));
        assert_eq!(document_metadata.updated, Some(now_string.clone()));
        assert_eq!(document_metadata.deactivated, Some(false));
        assert_eq!(document_metadata.next_update, Some(now_string.clone()));
        assert_eq!(document_metadata.version_id, Some(version_id.clone()));
        assert_eq!(
            document_metadata.next_version_id,
            Some(next_version_id.clone())
        );
        assert_eq!(document_metadata.canonical_id, Some(canonical_id.clone()));
        assert_eq!(
            document_metadata.equivalent_id,
            Some(vec![equivalent_id.clone()])
        );
    }
}
