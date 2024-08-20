package web5.sdk.crypto.signers

import web5.sdk.rust.Signer as RustCoreSigner

interface Signer {
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