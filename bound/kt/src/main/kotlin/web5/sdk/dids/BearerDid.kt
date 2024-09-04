package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.*
import web5.sdk.crypto.keys.ToInnerKeyExporter
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.keys.ToOuterKeyManager
import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.signers.ToOuterSigner
import web5.sdk.rust.BearerDid as RustCoreBearerDid
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents a Decentralized Identifier (DID) along with its DID document, key manager, metadata,
 * and convenience functions.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 * @property keyManager The KeyManager associated with this instance.
 */
data class BearerDid private constructor(
    val did: Did,
    val document: Document,
    val keyManager: KeyManager,
    internal val rustCoreBearerDid: RustCoreBearerDid
) {
    constructor(did: Did, document: Document, keyManager: KeyManager) : this(
        did,
        document,
        keyManager,
        try {
            RustCoreBearerDid(
                did.toRustCoreDidData(),
                document.toRustCore(),
                ToInnerKeyManager(keyManager)
            )
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    )

    companion object {
        /**
         * Constructs a BearerDid instance from a PortableDid.
         *
         * @param portableDid The PortableDid.
         */
        fun fromPortableDid(portableDid: PortableDid): BearerDid {
            try {
                val rustCoreBearerDid = RustCoreBearerDid.fromPortableDid(portableDid.rustCorePortableDid)
                return fromRustCoreBearerDid(rustCoreBearerDid)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        internal fun fromRustCoreBearerDid(rustCoreBearerDid: RustCoreBearerDid): BearerDid {
            val rustCoreBearerDidData = rustCoreBearerDid.getData()
            return BearerDid(
                Did.fromRustCoreDidData(rustCoreBearerDidData.did),
                Document.fromRustCore(rustCoreBearerDidData.document),
                ToOuterKeyManager(rustCoreBearerDidData.keyManager)
            )
        }
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(verificationMethodId: String): Signer {
        try {
            val rustCoreSigner = rustCoreBearerDid.getSigner(verificationMethodId)
            return ToOuterSigner(rustCoreSigner)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }

    /**
     * Returns the BearerDid represented as a PortableDid
     */
    fun toPortableDid(keyExporter: KeyExporter): PortableDid {
        try {
            val innerKeyExporter = ToInnerKeyExporter(keyExporter)
            val rustCorePortableDid = rustCoreBearerDid.toPortableDid(innerKeyExporter)
            return PortableDid.fromRustCorePortableDid(rustCorePortableDid)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
