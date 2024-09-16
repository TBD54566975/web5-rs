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
 * A `BearerDid` encapsulates a DID and its associated DID document. It provides functionality for
 * working with the DID's keys, managing key signers, and exporting the DID as a PortableDid.
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
    /**
     * Constructs a new [BearerDid] from the provided DID, document, and key manager.
     *
     * @param did The DID.
     * @param document The DID document.
     * @param keyManager The key manager used for key storage and signing.
     * @throws Web5Exception If an error occurs during the creation process.
     */
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
         * Constructs a [BearerDid] instance from a [PortableDid].
         *
         * This method allows you to create a `BearerDid` from a portable representation of a DID, typically used for
         * importing or exporting DIDs across different systems or platforms.
         *
         * @param portableDid The [PortableDid] object.
         * @return A new instance of [BearerDid].
         * @throws Web5Exception If an error occurs during the creation process.
         */
        fun fromPortableDid(portableDid: PortableDid): BearerDid {
            try {
                val rustCoreBearerDid = RustCoreBearerDid.fromPortableDid(portableDid.rustCorePortableDid)
                return fromRustCoreBearerDid(rustCoreBearerDid)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        /**
         * Constructs a [BearerDid] from a RustCore `BearerDid`.
         *
         * @param rustCoreBearerDid The RustCore `BearerDid` object.
         * @return A new instance of [BearerDid].
         */
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
     * This method retrieves a signer associated with the specified verification method. The signer can be used
     * to cryptographically sign data using the DID's key material.
     *
     * @param verificationMethodId The ID of the verification method to use.
     * @return A [Signer] instance for signing data.
     * @throws Web5Exception If an error occurs during signer retrieval.
     *
     * @example
     * ```
     * val bearerDid = BearerDid(did, document, keyManager)
     * val signer = bearerDid.getSigner("did:example:123#key-1")
     * val signature = signer.sign(data)
     * ```
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
     * Returns the BearerDid represented as a [PortableDid].
     *
     * This method exports the `BearerDid` as a `PortableDid`, allowing for easy transport and storage
     * across different systems or platforms.
     *
     * @param keyExporter The key exporter to use for exporting the private keys.
     * @return A [PortableDid] object representing the exported DID.
     * @throws Web5Exception If an error occurs during the export process.
     *
     * @example
     * ```
     * val bearerDid = BearerDid(did, document, keyManager)
     * val portableDid = bearerDid.toPortableDid(keyExporter)
     * ```
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
