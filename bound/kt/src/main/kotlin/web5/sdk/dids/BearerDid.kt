package web5.sdk.dids

import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.keys.ToOuterKeyManager
import web5.sdk.crypto.signers.ToOuterSigner
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

    internal val rustCoreBearerDid: RustCoreBearerDid

    internal constructor(rustCoreBearerDid: RustCoreBearerDid) {
        this.rustCoreBearerDid = rustCoreBearerDid
        this.did = Did.fromRustCoreDidData(this.rustCoreBearerDid.getData().did)
        this.document = this.rustCoreBearerDid.getData().document
        this.keyManager = ToOuterKeyManager(this.rustCoreBearerDid.getData().keyManager)
    }

    /**
     * Constructs a BearerDid instance using a DID URI and a key manager.
     *
     * @param uri The DID URI.
     * @param keyManager The key manager to handle keys.
     */
    constructor(uri: String, keyManager: KeyManager) {
        val innerKeyManager = ToInnerKeyManager(keyManager)
        this.rustCoreBearerDid = RustCoreBearerDid(uri, innerKeyManager)

        this.did = Did.fromRustCoreDidData(this.rustCoreBearerDid.getData().did)
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
        this.did = Did.fromRustCoreDidData(data.did)
        this.document = data.document
        this.keyManager = ToOuterKeyManager(data.keyManager)
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): Signer {
        val keyId = this.document.verificationMethod.first().id
        return ToOuterSigner(this.rustCoreBearerDid.getSigner(keyId))
    }
}
