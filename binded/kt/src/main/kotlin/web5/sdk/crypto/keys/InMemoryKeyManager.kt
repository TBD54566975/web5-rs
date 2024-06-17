package web5.sdk.crypto.keys

import web5.sdk.InMemoryKeyManager as RcbInMemoryKeyManager

class InMemoryKeyManager : KeyManager {
    private val rcbKeyManager = RcbInMemoryKeyManager()

    override fun generateKeyMaterial(): Jwk {
        val jwkData = rcbKeyManager.generateKeyMaterial();
        return Jwk.fromBinding(jwkData);
    }

    override fun getSigner(publicKey: Jwk): Ed25519Signer {
        return Ed25519Signer(publicKey)
    }

    override fun importKey(privateKey: Jwk): Jwk {
        val rcbJwk =  rcbKeyManager.importKey(privateKey.toBinding())
        return Jwk.fromBinding(rcbJwk)
    }
}
