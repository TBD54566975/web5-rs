package web5.sdk.crypto.signers

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer

/**
 * Implementation of [Signer] for Ed25519.
 */
class Ed25519Signer {
    private val rustCoreEd25519Signer: RustCoreEd25519Signer

    constructor(privateKey: Jwk) {
        this.rustCoreEd25519Signer = RustCoreEd25519Signer(privateKey.toRustCore())
    }

    constructor(rustCoreEd25519Signer: RustCoreEd25519Signer) {
        this.rustCoreEd25519Signer = rustCoreEd25519Signer
    }


    /**
     * Implementation of Signer's sign instance method for Ed25519.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    fun sign(payload: ByteArray): ByteArray {
        val uByteList = payload.toUByteArray().toList()
        return rustCoreEd25519Signer.sign(uByteList)
    }

    /**
     * Converts this Ed25519Signer instance to a RustCoreEd25519Signer object.
     */
    fun toRustCoreEd25519Signer(): RustCoreEd25519Signer {
        return this.rustCoreEd25519Signer
    }

    companion object {
        /**
         * Creates an instance of Ed25519Signer from a RustCoreEd25519Signer object.
         */
        fun fromRustCore(rustCoreSigner: RustCoreEd25519Signer): Ed25519Signer {
            return Ed25519Signer(rustCoreSigner)
        }
    }
}
