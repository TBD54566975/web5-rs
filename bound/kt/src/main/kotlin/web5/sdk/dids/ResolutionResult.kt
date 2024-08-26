package web5.sdk.dids

import web5.sdk.rust.DocumentMetadataData
import web5.sdk.rust.ResolutionMetadataData
import web5.sdk.rust.ResolutionResult as RustCoreResolutionResult

data class ResolutionResultResolveOptions(
    val didDhtGatewayUrl: String? = null,
)

/**
 * Representation of the result of a DID (Decentralized Identifier) resolution.
 */
class ResolutionResult(
    val document: Document? = null,
    val documentMetadata: DocumentMetadataData? = null,
    val resolutionMetadata: ResolutionMetadataData) {

    companion object {
        fun resolve(uri: String): ResolutionResult {
            val rustCoreResolutionResult = RustCoreResolutionResult.resolve(uri)
            return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
        }

        internal fun fromRustCoreResolutionResult(rustCoreResolutionResult: RustCoreResolutionResult): ResolutionResult {
            val data = rustCoreResolutionResult.getData()
            return ResolutionResult(data.document, data.documentMetadata, data.resolutionMetadata)
        }
    }
}