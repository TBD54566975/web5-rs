package web5.sdk.crypto.signers

import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import org.junit.jupiter.api.Assertions.assertNotNull
import web5.sdk.crypto.signers.Ed25519Signer

class Ed25519SignerTest {
    @Test
    fun `test key manager`() {
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        // TODO: Fix ed25519Signer
        // val ed25519Signer: Ed25519Signer = keyManager.getSigner(jwk)


        // val payload = ed25519Signer.sign("abc".toByteArray())
        // assertNotNull(payload)
    }
}