package web5.sdk.crypto.signers

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Implementation of Signer for Ed25519.
 */
class Ed25519Signer(privateJwk: Jwk) : Signer {
    private val rustCoreSigner = RustCoreEd25519Signer(privateJwk.rustCoreJwkData)

    /**
     * Implementation of Signer's sign instance method for Ed25519.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    override fun sign(payload: ByteArray): ByteArray {
        try {
            return rustCoreSigner.sign(payload)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
