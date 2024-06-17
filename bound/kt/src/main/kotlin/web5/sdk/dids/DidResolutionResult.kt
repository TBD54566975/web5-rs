package web5.sdk.dids

import web5.sdk.rust.DocumentMetadataData as RustCoreDocumentMetadataData
import web5.sdk.rust.ResolutionMetadataData as RustCoreResolutionMetadataData
import web5.sdk.rust.ResolutionMetadataError as RustCoreResolutionMetadataError
import web5.sdk.rust.ResolutionResultData as RustCoreResolutionResultData

/**
 * Representation of the result of a DID (Decentralized Identifier) resolution.
 *
 * @property didDocument The resolved DID document, if available.
 * @property didDocumentMetadata The metadata associated with the DID document.
 * @property didResolutionMetadata The metadata associated with the DID resolution process.
 */
data class DidResolutionResult(
    val didDocument: DidDocument? = null,
    val didDocumentMetadata: DidDocumentMetadata? = null,
    val didResolutionMetadata: DidResolutionMetadata = DidResolutionMetadata()
) {
    /**
     * Converts the DidResolutionResult instance to a RustCoreResolutionResultData binding.
     *
     * @return RustCoreResolutionResultData the corresponding RustCoreResolutionResultData object.
     */
    fun toBinding(): RustCoreResolutionResultData {
        return RustCoreResolutionResultData(
            document = this.didDocument?.toBinding(),
            documentMetadata = this.didDocumentMetadata?.toBinding(),
            resolutionMetadata = this.didResolutionMetadata.toBinding()
        )
    }

    companion object {
        /**
         * Creates a DidResolutionResult instance from a RustCoreResolutionResultData binding.
         *
         * @param data the RustCoreResolutionResultData object.
         * @return DidResolutionResult the corresponding DidResolutionResult instance.
         */
        fun fromBinding(data: RustCoreResolutionResultData): DidResolutionResult {
            return DidResolutionResult(
                didDocument = data.document?.let { DidDocument.fromBinding(it) },
                didDocumentMetadata = data.documentMetadata?.let { DidDocumentMetadata.fromBinding(it) },
                didResolutionMetadata = DidResolutionMetadata.fromBinding(data.resolutionMetadata)
            )
        }
    }
}

/**
 * Represents the metadata associated with a DID document.
 *
 * @property created The creation timestamp of the DID document.
 * @property updated The last updated timestamp of the DID document.
 * @property deactivated Indicates if the DID document has been deactivated.
 * @property versionId The version identifier of the DID document.
 * @property nextUpdate The timestamp of the next update to the DID document.
 * @property nextVersionId The version identifier of the next version of the DID document.
 * @property equivalentId A list of equivalent identifiers for the DID document.
 * @property canonicalId The canonical identifier for the DID document.
 */
data class DidDocumentMetadata(
    var created: String? = null,
    var updated: String? = null,
    var deactivated: Boolean? = null,
    var versionId: String? = null,
    var nextUpdate: String? = null,
    var nextVersionId: String? = null,
    var equivalentId: List<String>? = null,
    var canonicalId: String? = null
) {
    /**
     * Converts the DidDocumentMetadata instance to a RustCoreDocumentMetadataData binding.
     *
     * @return RustCoreDocumentMetadataData the corresponding RustCoreDocumentMetadataData object.
     */
    fun toBinding(): RustCoreDocumentMetadataData {
        return RustCoreDocumentMetadataData(
            created = this.created,
            updated = this.updated,
            deactivated = this.deactivated,
            versionId = this.versionId,
            nextUpdate = this.nextUpdate,
            nextVersionId = this.nextVersionId,
            equivalentId = this.equivalentId,
            canonicalId = this.canonicalId
        )
    }

    companion object {
        /**
         * Creates a DidDocumentMetadata instance from a RustCoreDocumentMetadataData binding.
         *
         * @param data the RustCoreDocumentMetadataData object.
         * @return DidDocumentMetadata the corresponding DidDocumentMetadata instance.
         */
        fun fromBinding(data: RustCoreDocumentMetadataData): DidDocumentMetadata {
            return DidDocumentMetadata(
                created = data.created,
                updated = data.updated,
                deactivated = data.deactivated,
                versionId = data.versionId,
                nextUpdate = data.nextUpdate,
                nextVersionId = data.nextVersionId,
                equivalentId = data.equivalentId,
                canonicalId = data.canonicalId
            )
        }
    }
}

/**
 * Represents the metadata associated with the DID resolution process.
 *
 * @property error The error associated with the DID resolution process, if any.
 */
data class DidResolutionMetadata(
    var error: String? = null
) {
    /**
     * Converts the DidResolutionMetadata instance to a RustCoreResolutionMetadataData binding.
     *
     * @return RustCoreResolutionMetadataData the corresponding RustCoreResolutionMetadataData object.
     */
    fun toBinding(): RustCoreResolutionMetadataData {
        return RustCoreResolutionMetadataData(
            error = RustCoreResolutionMetadataError.INTERNAL_ERROR // TODO: Add correct errors and not hardcoded
        )
    }

    companion object {
        /**
         * Creates a DidResolutionMetadata instance from a RustCoreResolutionMetadataData binding.
         *
         * @param data the RustCoreResolutionMetadataData object.
         * @return DidResolutionMetadata the corresponding DidResolutionMetadata instance.
         */
        fun fromBinding(data: RustCoreResolutionMetadataData): DidResolutionMetadata {
            return DidResolutionMetadata(
                error = data.error.toString()
            )
        }
    }
}
