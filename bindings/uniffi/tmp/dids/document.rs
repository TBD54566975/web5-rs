use dids::document::{
    Document, DocumentError, KeySelector as InternalKeySelector, VerificationMethod,
    VerificationMethodType,
};

pub enum KeySelector {
    KeyId { key_id: String },
    MethodType { method_type: VerificationMethodType },
}

pub fn get_verification_method(
    document: Document,
    key_selector: KeySelector,
) -> Result<VerificationMethod, DocumentError> {
    let key_selector_internal = match key_selector {
        KeySelector::KeyId { key_id } => InternalKeySelector::KeyId(key_id),
        KeySelector::MethodType { method_type } => InternalKeySelector::MethodType(method_type),
    };
    let verification_method = document.get_verification_method(&key_selector_internal)?;
    Ok(verification_method)
}
