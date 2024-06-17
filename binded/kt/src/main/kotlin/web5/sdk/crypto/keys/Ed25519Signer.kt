package web5.sdk.crypto.keys

import web5.sdk.Ed25519Signer as RcbEd25519Signer

class Ed25519Signer(private val privateKey: Jwk) : Signer {
    private val rcbEd25519Signer = RcbEd25519Signer(privateKey.toBinding())

    @OptIn(ExperimentalUnsignedTypes::class)
    override fun sign(payload: ByteArray): ByteArray {
        val uByteList = payload.toUByteArray().toList()
        return rcbEd25519Signer.sign(uByteList)
    }
}