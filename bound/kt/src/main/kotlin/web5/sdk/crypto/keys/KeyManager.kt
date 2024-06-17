package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Ed25519Signer

interface KeyManager {
    fun generateKeyMaterial(): Jwk
    fun getSigner(publicKey: Jwk): Ed25519Signer
    fun importKey(privateKey: Jwk): Jwk
}