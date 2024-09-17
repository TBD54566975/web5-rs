package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.rust.Did as RustCoreDid
import web5.sdk.rust.DidData as RustCoreDidData
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).
 *
 * A `Did` represents a Decentralized Identifier (DID), which uniquely identifies a subject (such as a person,
 * organization, device, etc.). This class allows the parsing of a DID URI, as well as the transformation
 * to and from Rust core data.
 *
 * @property uri The complete DID URI.
 * @property url The parsed URL associated with the DID.
 * @property method The DID method (e.g., `jwk`, `dht`, `web`), which defines how the DID is resolved.
 * @property id The method-specific identifier of the DID.
 * @property params Optional method-specific parameters present in the DID URI.
 * @property path Optional path component in the DID URI.
 * @property query Optional query component in the DID URI.
 * @property fragment Optional fragment component in the DID URI.
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
        /**
         * Parses a DID URI into a `Did` object.
         *
         * This method constructs a `Did` instance by using Rust core functions to parse the provided DID URI.
         * It retrieves the underlying data and converts it into a Kotlin `Did` object.
         *
         * @param uri The DID URI to parse.
         * @return A `Did` object representing the parsed DID URI.
         * @throws Web5Exception If parsing the DID URI fails.
         *
         * @example
         * ```
         * val did = Did.parse("did:example:123456")
         * println(did.method)  // Output: "example"
         * ```
         */
        fun parse(uri: String): Did {
            try {
                val rustCoreDid = RustCoreDid(uri)
                val data = rustCoreDid.getData()
                return fromRustCoreDidData(data)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
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
