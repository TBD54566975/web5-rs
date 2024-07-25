package web5.sdk.dids

import web5.sdk.rust.SystemTarget
import web5.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.rust.KeyManager as RustCoreKeyManager
import web5.sdk.rust.Signer as RustCoreSigner

/**
 * Represents a Decentralized Identifier (DID) along with its DID document, key manager, metadata,
 * and convenience functions.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class BearerDid {
    init {
        SystemTarget.set() // ensure the sys arch is set for first-time loading
    }

    val did: Did
    val document: Document
    val keyManager: RustCoreKeyManager

    private val rustCoreBearerDid: RustCoreBearerDid

    /**
     * Constructs a BearerDid instance using a DID URI and a key manager.
     *
     * @param uri The DID URI.
     * @param keyManager The key manager to handle keys.
     */
    constructor(uri: String, keyManager: RustCoreKeyManager): this(RustCoreBearerDid(uri, keyManager))

    private constructor(rustCoreBearerDid: RustCoreBearerDid) {
        this.rustCoreBearerDid = rustCoreBearerDid

        val data = this.rustCoreBearerDid.getData()
        this.did = data.did
        this.document = data.document
        this.keyManager = data.keyManager
    }

    companion object {
        fun fromPortableDid(portableDid: PortableDid): BearerDid {
            val rustCoreBearerDid = RustCoreBearerDid.fromPortableDid(portableDid.rustCorePortableDid)
            return BearerDid(rustCoreBearerDid)
        }
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): RustCoreSigner {
        val keyId = this.document.verificationMethod.first().id
        return this.rustCoreBearerDid.getSigner(keyId)
    }
}
