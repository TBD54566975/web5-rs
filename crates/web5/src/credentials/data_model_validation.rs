use std::time::SystemTime;

use crate::errors::Result;

use super::{
    verifiable_credential_1_1::{VerifiableCredential, BASE_CONTEXT, BASE_TYPE},
    VerificationError,
};

pub fn validate_vc_data_model(vc: &VerifiableCredential) -> Result<()> {
    // Required fields ["@context", "id", "type", "issuer", "issuanceDate", "credentialSubject"]
    if vc.id.is_empty() {
        return Err(VerificationError::DataModelValidationError("missing id".to_string()).into());
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(
            VerificationError::DataModelValidationError("missing context".to_string()).into(),
        );
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(VerificationError::DataModelValidationError("missing type".to_string()).into());
    }

    if vc.issuer.to_string().is_empty() {
        return Err(
            VerificationError::DataModelValidationError("missing issuer".to_string()).into(),
        );
    }

    if vc.credential_subject.id.is_empty() {
        return Err(VerificationError::DataModelValidationError(
            "missing credential subject".to_string(),
        )
        .into());
    }

    let now = SystemTime::now();
    if vc.issuance_date > now {
        return Err(VerificationError::DataModelValidationError(
            "issuance date in future".to_string(),
        )
        .into());
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = &vc.expiration_date {
        if expiration_date < &now {
            return Err(VerificationError::DataModelValidationError(
                "credential expired".to_string(),
            )
            .into());
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}
