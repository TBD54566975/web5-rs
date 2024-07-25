package web5.sdk.dids

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.SystemTarget
import web5.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid private constructor(
    val didUri: String,
    val document: Document,
    val privateKeys: List<Jwk>,
    internal val rustCorePortableDid: RustCorePortableDid
) {
    init {
        SystemTarget.set() // ensure the sys arch is set for first-time loading
    }

    companion object {
        /**
         * Constructs a PortableDid instance from a JSON string representation.
         *
         * @param json The JSON string.
         */
        fun fromJsonString(json: String): PortableDid {
            val rustCorePortableDid = RustCorePortableDid.fromJsonString(json)
            val data = rustCorePortableDid.getData()

            return PortableDid(data.didUri, data.document, data.privateJwks, rustCorePortableDid)
        }
    }
}