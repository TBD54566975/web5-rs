package web5.sdk.crypto.signers

import web5.sdk.rust.Signer as RustCoreSigner

interface Signer {
    fun sign(payload: ByteArray): ByteArray
}

class OuterSigner: Signer {
    private val rustCoreSigner: RustCoreSigner

    constructor(rustCoreSigner: RustCoreSigner) {
        this.rustCoreSigner = rustCoreSigner
    }

    override fun sign(payload: ByteArray): ByteArray {
        return this.rustCoreSigner.sign(payload)
    }
}