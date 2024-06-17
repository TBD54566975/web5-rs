package web5.sdk.crypto.keys

interface KeyManager {
    fun generateKeyMaterial(): Jwk
    fun getSigner(publicKey: Jwk): Ed25519Signer
    fun importKey(privateKey: Jwk): Jwk
}