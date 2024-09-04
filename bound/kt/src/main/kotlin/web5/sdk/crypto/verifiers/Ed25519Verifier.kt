package web5.sdk.crypto.verifiers

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Verifier as RustCoreEd25519Verifier
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Implementation of Verifier for Ed25519.
 */
class Ed25519Verifier(publicJwk: Jwk) : Verifier {
    private val rustCoreVerifier = RustCoreEd25519Verifier(publicJwk.rustCoreJwkData)

    /**
     * Implementation of Signer's verify instance method for Ed25519.
     *
     * @param message the data to be verified.
     * @param signature the signature to be verified.
     * @throws Web5Exception in the case of a failed verification
     */
    override fun verify(message: ByteArray, signature: ByteArray) {
        try {
            rustCoreVerifier.verify(message, signature)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}