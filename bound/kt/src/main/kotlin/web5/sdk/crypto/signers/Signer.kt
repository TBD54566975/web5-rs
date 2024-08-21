package web5.sdk.crypto.signers

import web5.sdk.rust.Signer as RustCoreSigner

/**
 * Set of functionality required to implement to be a compatible DSA signer.
 */
interface Signer {
    /**
     * Signs the given payload by using the encapsulated private key material.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    fun sign(payload: ByteArray): ByteArray
}

internal class ToOuterSigner(private val rustCoreSigner: RustCoreSigner) : Signer {
    override fun sign(payload: ByteArray): ByteArray {
        return rustCoreSigner.sign(payload)
    }
}

internal class ToInnerSigner(private val signer: Signer) : RustCoreSigner {
    override fun sign(payload: ByteArray): ByteArray {
        return signer.sign(payload)
    }
}