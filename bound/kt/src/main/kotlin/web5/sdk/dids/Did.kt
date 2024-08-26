package web5.sdk.dids

import web5.sdk.rust.Did as RustCoreDid
import web5.sdk.rust.DidData as RustCoreDidData

/**
 * Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).
 */
data class Did (
    val uri: String,
    val url: String,
    val method: String,
    val id: String,
    val params: Map<String, String>? = null,
    val path: String? = null,
    val query: String? = null,
    val fragment: String? = null
) {
    companion object {
        fun parse(uri: String): Did {
            val rustCoreDid = RustCoreDid(uri)
            val data = rustCoreDid.getData()
            return fromRustCoreDidData(data)
        }

        internal fun fromRustCoreDidData(data: RustCoreDidData): Did {
            return Did(
                data.uri,
                data.url,
                data.method,
                data.id,
                data.params,
                data.path,
                data.query,
                data.fragment,
            )
        }
    }

    internal fun toRustCoreDidData(): RustCoreDidData {
        return RustCoreDidData(uri, url, method, id, params, path, query, fragment)
    }
}