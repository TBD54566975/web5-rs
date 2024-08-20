package web5.sdk.dids

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid private constructor(
    val didUri: String,
    val document: Document,
    val privateKeys: List<Jwk>,
    internal val rustCorePortableDid: RustCorePortableDid
) {
    /**
     * Constructs a PortableDid from a JSON string.
     *
     * @param json The JSON string.
     */
    companion object {
        fun fromJsonString(json: String): PortableDid {
            val rustCorePortableDid = RustCorePortableDid.fromJsonString(json)
            val data = rustCorePortableDid.getData()
            return PortableDid(
                data.didUri,
                data.document,
                data.privateJwks.map { Jwk.fromRustCoreJwkData(it) },
                rustCorePortableDid
            )
        }
    }

    /**
     * Serializes a PortableDid to a JSON string.
     */
    fun toJsonString(): String {
        return rustCorePortableDid.toJsonString()
    }
}