package web5.sdk.crypto.signers

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer

/**
 * Implementation of [Signer] for Ed25519.
 */
class Ed25519Signer(privateKey: Jwk) : Signer {
    private val rustCoreEd25519Signer = RustCoreEd25519Signer(privateKey.toBinding())

    /**
     * Implementation of Signer's sign instance method for Ed25519.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    override fun sign(payload: ByteArray): ByteArray {
        val uByteList = payload.toUByteArray().toList()
        return rustCoreEd25519Signer.sign(uByteList)
    }
}
