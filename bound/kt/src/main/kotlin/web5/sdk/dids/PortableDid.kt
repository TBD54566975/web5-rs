package web5.sdk.dids

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid private constructor(
    val didUri: String,
    val document: Document,
    val privateKeys: List<Jwk>,
    internal val rustCorePortableDid: RustCorePortableDid
) {
    constructor(didUri: String, document: Document, privateKeys: List<Jwk>) : this(
        didUri,
        document,
        privateKeys,
        RustCorePortableDid(
            didUri,
            document.toRustCore(),
            privateKeys.map { it.rustCoreJwkData }
        )
    )

    companion object {
        /**
         * Constructs a PortableDid from a JSON string.
         *
         * @param json The JSON string.
         */
        fun fromJsonString(json: String): PortableDid {
            val rustCorePortableDid = RustCorePortableDid.fromJsonString(json)
            return PortableDid.fromRustCorePortableDid(rustCorePortableDid)
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
     * Serializes a PortableDid to a JSON string.
     */
    fun toJsonString(): String {
        return rustCorePortableDid.toJsonString()
    }
}