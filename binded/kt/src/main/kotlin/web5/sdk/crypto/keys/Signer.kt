package web5.sdk.crypto.keys

interface Signer {
    fun sign(payload: ByteArray): ByteArray
}