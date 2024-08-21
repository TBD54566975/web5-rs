package web5.sdk.crypto.verifiers

import web5.sdk.rust.Verifier as RustCoreVerifier
import web5.sdk.rust.Web5Exception

/**
 * Set of functionality required to implement to be a compatible DSA verifier.
 */
interface Verifier {
    /**
     * Execute the verification of the signature against the payload by using the encapsulated public key material.
     *
     * @param message the data which was signed over.
     * @param signature the signature over the message.
     * @throws Web5Exception in the case of a failed verification.
     */
    fun verify(message: ByteArray, signature: ByteArray)
}

internal class ToOuterVerifier(private val rustCoreVerifier: RustCoreVerifier) : Verifier {
    override fun verify(message: ByteArray, signature: ByteArray) {
        rustCoreVerifier.verify(message, signature)
    }
}

internal class ToInnerVerifier(private val verifier: Verifier) : RustCoreVerifier {
    override fun verify(message: ByteArray, signature: ByteArray) {
        verifier.verify(message, signature)
    }
}