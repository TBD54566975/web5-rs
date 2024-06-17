package web5.sdk.dids

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.rust.DocumentMetadataData as RustCoreDocumentMetadataData
import web5.sdk.rust.ResolutionMetadataData as RustCoreResolutionMetadataData
import web5.sdk.rust.ResolutionMetadataError as RustCoreResolutionMetadataError
import web5.sdk.rust.ResolutionResultData as RustCoreResolutionResultData

data class DidResolutionResult(
    @JsonProperty("@context")
    val context: String? = null,
    val didDocument: DidDocument? = null,
    val didDocumentMetadata: DidDocumentMetadata = DidDocumentMetadata(),
    val didResolutionMetadata: DidResolutionMetadata = DidResolutionMetadata()
) {
    fun toBinding(): RustCoreResolutionResultData {
        return RustCoreResolutionResultData(
            document = this.didDocument?.toBinding()!!,
            documentMetadata = this.didDocumentMetadata.toBinded(),
            resolutionMetadata = this.didResolutionMetadata.toBinded()
        )
    }

    companion object {
        fun fromBinding(data: RustCoreResolutionResultData): DidResolutionResult {
            return DidResolutionResult(
                didDocument = data.document?.let { DidDocument.fromBinding(it) },
                didDocumentMetadata = DidDocumentMetadata.fromBinded(data.documentMetadata!!),
                didResolutionMetadata = DidResolutionMetadata.fromBinded(data.resolutionMetadata)
            )
        }
    }
}

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
    fun toBinded(): RustCoreDocumentMetadataData {
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
        fun fromBinded(data: RustCoreDocumentMetadataData): DidDocumentMetadata {
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



data class DidResolutionMetadata(
    var error: String? = null,
) {
    fun toBinded(): RustCoreResolutionMetadataData {
        return RustCoreResolutionMetadataData(
            error = RustCoreResolutionMetadataError.INTERNAL_ERROR, // TOOD: Add correct errors and not hardcoded
        )
    }

    companion object {
        fun fromBinded(data: RustCoreResolutionMetadataData): DidResolutionMetadata {
            return DidResolutionMetadata(
                error = data.error.toString(),
            )
        }
    }
}