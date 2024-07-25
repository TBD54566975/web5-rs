package web5.sdk.dids

import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.signers.OuterSigner
import web5.sdk.rust.SystemTarget

import web5.sdk.rust.BearerDid as RustCoreBearerDid

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
    val keyManager: KeyManager

    private val rustCoreBearerDid: RustCoreBearerDid

    /**
     * Constructs a BearerDid instance using a DID URI and a key manager.
     *
     * @param uri The DID URI.
     * @param keyManager The key manager to handle keys.
     */
    constructor(uri: String, keyManager: KeyManager) {
        this.rustCoreBearerDid = RustCoreBearerDid(uri, keyManager.getRustCoreKeyManager())

        this.did = this.rustCoreBearerDid.getData().did
        this.document = this.rustCoreBearerDid.getData().document
        this.keyManager = keyManager
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): Signer {
        val keyId = this.document.verificationMethod.first().id
        val innerSigner = this.rustCoreBearerDid.getSigner(keyId)
        return OuterSigner(innerSigner)
    }
}
