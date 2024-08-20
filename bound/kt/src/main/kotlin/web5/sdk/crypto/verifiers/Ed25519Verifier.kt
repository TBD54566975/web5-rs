package web5.sdk.crypto.verifiers

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Verifier as RustCoreEd25519Verifier

class Ed25519Verifier : Verifier {
    private val rustCoreVerifier: RustCoreEd25519Verifier

    constructor(privateKey: Jwk) {
        this.rustCoreVerifier = RustCoreEd25519Verifier(privateKey)
    }

    private constructor(rustCoreVerifier: RustCoreEd25519Verifier) {
        this.rustCoreVerifier = rustCoreVerifier
    }

    /**
     * Implementation of Signer's verify instance method for Ed25519.
     *
     * @param message the data to be verified.
     * @param signature the signature to be verified.
     * @return ByteArray the signature.
     */
    override fun verify(message: ByteArray, signature: ByteArray): Boolean {
        return rustCoreVerifier.verify(message, signature);
    }
}