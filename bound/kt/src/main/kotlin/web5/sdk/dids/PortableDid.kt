package web5.sdk.dids

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.SystemTarget
import web5.sdk.rust.PortableDid as RustCorePortableDid

class PortableDid {
    init {
        SystemTarget.set() // ensure the sys arch is set for first-time loading
    }

    val didUri: String
    val document: Document
    val privateKeys: List<Jwk>

    internal val rustCorePortableDid: RustCorePortableDid

    /**
     * Constructs a PortableDid from a JSON string.
     *
     * @param json The JSON string.
     */
    constructor(json: String) {
        this.rustCorePortableDid = RustCorePortableDid(json)

        this.didUri = rustCorePortableDid.getData().didUri
        this.document = rustCorePortableDid.getData().document
        this.privateKeys = rustCorePortableDid.getData().privateJwks
    }
}