package web5.sdk.dids

import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.rust.BearerDid as RustCoreBearerDid

/**
 * Represents a Decentralized Identifier (DID) along with its DID document, key manager, metadata,
 * and convenience functions.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class BearerDid {
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
        this.rustCoreBearerDid = RustCoreBearerDid(uri, keyManager)

        this.did = this.rustCoreBearerDid.getData().did
        this.document = this.rustCoreBearerDid.getData().document
        this.keyManager = keyManager
    }

    /**
     * Constructs a BearerDid instance from a PortableDid.
     *
     * @param portableDid The PortableDid.
     */
    constructor(portableDid: PortableDid) {
        this.rustCoreBearerDid = RustCoreBearerDid.fromPortableDid(portableDid.rustCorePortableDid)

        val data = this.rustCoreBearerDid.getData()
        this.did = data.did
        this.document = data.document
        this.keyManager = data.keyManager
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): Signer {
        val keyId = this.document.verificationMethod.first().id
        return this.rustCoreBearerDid.getSigner(keyId)
    }
}
