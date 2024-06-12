package web5.sdk.dids.did.methods

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.DocumentMetadataData
import web5.sdk.ResolutionMetadataData
import web5.sdk.ResolutionMetadataError
import web5.sdk.ResolutionResultData

public data class DidResolutionResult(
    @JsonProperty("@context")
    val context: String? = null,
    val didDocument: DidDocument? = null,
    val didDocumentMetadata: DidDocumentMetadata = DidDocumentMetadata(),
    val didResolutionMetadata: DidResolutionMetadata = DidResolutionMetadata()
) {
    fun toBinded(): ResolutionResultData {
        return ResolutionResultData(
            document = this.didDocument?.toBinded()!!,
            documentMetadata = this.didDocumentMetadata.toBinded(),
            resolutionMetadata = this.didResolutionMetadata.toBinded()
        )
    }

    companion object {
        fun fromBinded(data: ResolutionResultData): DidResolutionResult {
            return DidResolutionResult(
                didDocument = data.document?.let { DidDocument.fromBinded(it) },
                didDocumentMetadata = DidDocumentMetadata.fromBinded(data.documentMetadata),
                didResolutionMetadata = DidResolutionMetadata.fromBinded(data.resolutionMetadata)
            )
        }
    }
}

public open class DidDocumentMetadata(
    var created: String? = null,
    var updated: String? = null,
    var deactivated: Boolean? = null,
    var versionId: String? = null,
    var nextUpdate: String? = null,
    var nextVersionId: String? = null,
    var equivalentId: List<String>? = null,
    var canonicalId: String? = null
) {
    fun toBinded(): DocumentMetadataData {
        return DocumentMetadataData(
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
        fun fromBinded(data: DocumentMetadataData): DidDocumentMetadata {
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



public class DidResolutionMetadata(
    var error: String? = null,
) {
    fun toBinded(): ResolutionMetadataData {
        return ResolutionMetadataData(
            error = ResolutionMetadataError.INTERNAL_ERROR, // TOOD: Add correct errors and not hardcoded
        )
    }

    companion object {
        fun fromBinded(data: ResolutionMetadataData): DidResolutionMetadata {
            return DidResolutionMetadata(
                error = data.error.toString(),
            )
        }
    }
}