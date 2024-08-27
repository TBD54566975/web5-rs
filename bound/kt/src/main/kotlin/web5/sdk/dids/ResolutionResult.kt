package web5.sdk.dids

/**
 * Representation of the result of a DID (Decentralized Identifier) resolution.
 */
class ResolutionResult(
    val document: Document? = null,
    val documentMetadata: DocumentMetadata? = null,
    val resolutionMetadata: ResolutionMetadata) {

    companion object {
        fun resolve(uri: String): ResolutionResult {
            val rustCoreResolutionResult = web5.sdk.rust.ResolutionResult.resolve(uri)
            return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
        }

        internal fun fromRustCoreResolutionResult(rustCoreResolutionResult: web5.sdk.rust.ResolutionResult): ResolutionResult {
            val data = rustCoreResolutionResult.getData()
            return ResolutionResult(
                data.document?.let { Document.fromRustCore(it) },
                data.documentMetadata?.let { DocumentMetadata.fromRustCore(it) },
                ResolutionMetadata.fromRustCore(data.resolutionMetadata)
            )
        }
    }
}

data class DocumentMetadata(
    val created: String?,
    val updated: String?,
    val deactivated: Boolean?,
    val nextUpdate: String?,
    val versionId: String?,
    val nextVersionId: String?,
    val equivalentId: List<String>?,
    val canonicalId: String?
) {
    companion object {
        internal fun fromRustCore(documentMetadata: web5.sdk.rust.DocumentMetadataData): DocumentMetadata {
            return DocumentMetadata(
                documentMetadata.created,
                documentMetadata.updated,
                documentMetadata.deactivated,
                documentMetadata.nextUpdate,
                documentMetadata.versionId,
                documentMetadata.nextVersionId,
                documentMetadata.equivalentId,
                documentMetadata.canonicalId
            )
        }
    }
}

data class ResolutionMetadata(
    val error: ResolutionMetadataError?
) {
    companion object {
        internal fun fromRustCore(resolutionMetadata: web5.sdk.rust.ResolutionMetadataData): ResolutionMetadata {
            return ResolutionMetadata(
                resolutionMetadata.error?.let {
                    when (it) {
                        web5.sdk.rust.ResolutionMetadataError.INVALID_DID -> ResolutionMetadataError.INVALID_DID
                        web5.sdk.rust.ResolutionMetadataError.NOT_FOUND -> ResolutionMetadataError.NOT_FOUND
                        web5.sdk.rust.ResolutionMetadataError.REPRESENTATION_NOT_SUPPORTED -> ResolutionMetadataError.REPRESENTATION_NOT_SUPPORTED
                        web5.sdk.rust.ResolutionMetadataError.METHOD_NOT_SUPPORTED -> ResolutionMetadataError.METHOD_NOT_SUPPORTED
                        web5.sdk.rust.ResolutionMetadataError.INVALID_DID_DOCUMENT -> ResolutionMetadataError.INVALID_DID_DOCUMENT
                        web5.sdk.rust.ResolutionMetadataError.INVALID_PUBLIC_KEY -> ResolutionMetadataError.INVALID_PUBLIC_KEY
                        web5.sdk.rust.ResolutionMetadataError.INVALID_DID_DOCUMENT_LENGTH -> ResolutionMetadataError.INVALID_DID_DOCUMENT_LENGTH
                        web5.sdk.rust.ResolutionMetadataError.INTERNAL_ERROR -> ResolutionMetadataError.INTERNAL_ERROR
                    }
                }
            )
        }
    }
}

enum class ResolutionMetadataError {
    INVALID_DID,
    NOT_FOUND,
    REPRESENTATION_NOT_SUPPORTED,
    METHOD_NOT_SUPPORTED,
    INVALID_DID_DOCUMENT,
    INVALID_PUBLIC_KEY,
    INVALID_DID_DOCUMENT_LENGTH,
    INTERNAL_ERROR;
}