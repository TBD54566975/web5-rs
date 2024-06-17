package web5.sdk.crypto.signers

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer

class Ed25519Signer(privateKey: Jwk) : Signer {
    private val rustCoreEd25519Signer = RustCoreEd25519Signer(privateKey.toBinding())

    @OptIn(ExperimentalUnsignedTypes::class)
    override fun sign(payload: ByteArray): ByteArray {
        val uByteList = payload.toUByteArray().toList()
        return rustCoreEd25519Signer.sign(uByteList)
    }
}