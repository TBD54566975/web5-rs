package web5.sdk.crypto.signers

interface Signer {
    fun sign(payload: ByteArray): ByteArray
}