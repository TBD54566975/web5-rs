package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.PortableDid as RustCorePortableDid
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents a portable DID (Decentralized Identifier) and its associated data, such as the DID document and private keys.
 *
 * A `PortableDid` allows the DID, its document, and associated private keys to be transported, saved, or shared
 * across systems in a portable format.
 *
 * @property didUri The DID URI associated with the portable DID.
 * @property document The DID document associated with the portable DID.
 * @property privateKeys A list of private keys (JWK format) associated with the DID.
 */
data class PortableDid private constructor(
    val didUri: String,
    val document: Document,
    val privateKeys: List<Jwk>,
    internal val rustCorePortableDid: RustCorePortableDid
) {
    /**
     * Constructs a `PortableDid` instance from a DID URI, document, and list of private keys.
     *
     * @param didUri The DID URI associated with the portable DID.
     * @param document The DID document associated with the portable DID.
     * @param privateKeys A list of private keys (JWK format) associated with the DID.
     * @throws Web5Exception If there is an error constructing the RustCorePortableDid object.
     *
     * @example
     * ```
     * val portableDid = PortableDid(didUri, document, privateKeys)
     * println(portableDid.didUri)  // Output: The DID URI
     * ```
     */
    constructor(didUri: String, document: Document, privateKeys: List<Jwk>) : this(
        didUri,
        document,
        privateKeys,
        try {
            RustCorePortableDid(
                didUri,
                document.toRustCore(),
                privateKeys.map { it.rustCoreJwkData }
            )
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    )

    companion object {
        /**
         * Constructs a `PortableDid` from a JSON string.
         *
         * This method parses a JSON string and constructs a `PortableDid` instance using Rust core functionality.
         *
         * @param json The JSON string representing the portable DID.
         * @return A `PortableDid` object.
         * @throws Web5Exception If there is an error parsing the JSON string.
         *
         * @example
         * ```
         * val jsonString = """{ "didUri": "did:example:123", "document": {...}, "privateKeys": [...] }"""
         * val portableDid = PortableDid.fromJsonString(jsonString)
         * println(portableDid.didUri)  // Output: The DID URI
         * ```
         */
        fun fromJsonString(json: String): PortableDid {
            try {
                val rustCorePortableDid = RustCorePortableDid.fromJsonString(json)
                return fromRustCorePortableDid(rustCorePortableDid)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        internal fun fromRustCorePortableDid(rustCorePortableDid: RustCorePortableDid): PortableDid {
            val data = rustCorePortableDid.getData()
            return PortableDid(
                data.didUri,
                Document.fromRustCore(data.document),
                data.privateJwks.map { Jwk.fromRustCoreJwkData(it) },
                rustCorePortableDid
            )
        }
    }

    /**
     * Serializes the `PortableDid` to a JSON string.
     *
     * This method converts the `PortableDid` instance into a JSON string using Rust core functionality.
     *
     * @return A JSON string representing the portable DID.
     * @throws Web5Exception If serialization to JSON fails.
     *
     * @example
     * ```
     * val portableDid = PortableDid(didUri, document, privateKeys)
     * val jsonString = portableDid.toJsonString()
     * println(jsonString)  // Output: JSON representation of the portable DID
     * ```
     */
    fun toJsonString(): String {
        try {
            return rustCorePortableDid.toJsonString()
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
