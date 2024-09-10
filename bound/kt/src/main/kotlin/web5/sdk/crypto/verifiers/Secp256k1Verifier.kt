package web5.sdk.crypto.verifiers

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Secp256k1Verifier as RustCoreSecp256k1Verifier
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Implementation of Verifier for secp256k1.
 */
class Secp256k1Verifier(publicJwk: Jwk) : Verifier {
    private val rustCoreVerifier = RustCoreSecp256k1Verifier(publicJwk.rustCoreJwkData)

    /**
     * Implementation of Signer's verify instance method for secp256k1.
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