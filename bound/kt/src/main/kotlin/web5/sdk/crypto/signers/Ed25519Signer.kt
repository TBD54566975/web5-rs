package web5.sdk.crypto.signers

import web5.sdk.crypto.keys.Jwk

import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer

class Ed25519Signer : Signer {
    private val rustCoreSigner: RustCoreEd25519Signer

    constructor(privateKey: Jwk) {
        this.rustCoreSigner = RustCoreEd25519Signer(privateKey)
    }

    private constructor(rustCoreSigner: RustCoreEd25519Signer) {
        this.rustCoreSigner = rustCoreSigner
    }

    /**
     * Implementation of Signer's sign instance method for Ed25519.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    override fun sign(payload: List<UByte>): ByteArray {
        val uByteList = payload.toUByteArray().toList()
        return rustCoreSigner.sign(uByteList)
    }
}
