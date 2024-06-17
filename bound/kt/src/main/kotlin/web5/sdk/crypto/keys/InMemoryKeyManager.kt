package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

class InMemoryKeyManager : KeyManager {
    private val rustCoreKeyManager = RustCoreInMemoryKeyManager()

    override fun generateKeyMaterial(): Jwk {
        val jwkData = rustCoreKeyManager.generateKeyMaterial();
        return Jwk.fromBinding(jwkData);
    }

    override fun getSigner(publicKey: Jwk): Ed25519Signer {
        return Ed25519Signer(publicKey)
    }

    override fun importKey(privateKey: Jwk): Jwk {
        val rustCoreJwk =  rustCoreKeyManager.importKey(privateKey.toBinding())
        return Jwk.fromBinding(rustCoreJwk)
    }
}