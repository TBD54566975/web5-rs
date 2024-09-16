package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Representation of the result of a DID (Decentralized Identifier) resolution.
 *
 * The `ResolutionResult` holds the data returned from resolving a DID. It includes the resolved DID document,
 * metadata about the DID document, and metadata about the resolution process itself.
 *
 * @property document The resolved DID document (optional).
 * @property documentMetadata Metadata related to the resolved DID document (optional).
 * @property resolutionMetadata Metadata about the resolution process, including errors if any occurred.
 */
data class ResolutionResult(
    val document: Document? = null,
    val documentMetadata: DocumentMetadata? = null,
    val resolutionMetadata: ResolutionMetadata) {

    companion object {
        /**
         * Resolves a DID URI into a `ResolutionResult`.
         *
         * This method attempts to resolve the provided DID URI using Rust core functionality. It returns
         * a `ResolutionResult` that contains the DID document, document metadata, and resolution metadata.
         *
         * @param uri The DID URI to resolve.
         * @return A `ResolutionResult` containing the resolved DID document and metadata.
         * @throws Web5Exception If an error occurs during the resolution process.
         *
         * @example
         * ```
         * val resolutionResult = ResolutionResult.resolve("did:example:123456")
         * println(resolutionResult.document?.id)  // Output: The resolved DID document ID
         * ```
         */
        fun resolve(uri: String): ResolutionResult {
            try {
                val rustCoreResolutionResult = web5.sdk.rust.ResolutionResult.resolve(uri)
                return fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
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

/**
 * Metadata associated with a DID document.
 *
 * `DocumentMetadata` contains information about the DID document, including creation and update timestamps,
 * versioning, and any potential deactivation status.
 *
 * @property created The timestamp when the DID document was created (optional).
 * @property updated The timestamp when the DID document was last updated (optional).
 * @property deactivated Indicates whether the DID document has been deactivated (optional).
 * @property nextUpdate The timestamp for the next expected update to the DID document (optional).
 * @property versionId The version identifier of the current DID document (optional).
 * @property nextVersionId The version identifier of the next expected version of the DID document (optional).
 * @property equivalentId A list of equivalent identifiers for the DID document (optional).
 * @property canonicalId The canonical ID for the DID document, if applicable (optional).
 */
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


/**
 * Metadata about the DID resolution process.
 *
 * `ResolutionMetadata` holds information about the resolution process, including any errors encountered
 * while resolving the DID.
 *
 * @property error The error that occurred during resolution, if applicable (optional).
 */
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

/**
 * Enum representing possible errors that can occur during DID resolution.
 */
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