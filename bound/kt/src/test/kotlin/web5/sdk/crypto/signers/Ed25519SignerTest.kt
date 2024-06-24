package web5.sdk.crypto.signers

import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import org.junit.jupiter.api.Assertions.assertNotNull

import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate

class Ed25519SignerTest {

    @Test
    fun `test signer`() {
        val rustCorePrivateJwk = rustCoreEd25519GeneratorGenerate()
        val ed25519Signer = Ed25519Signer(rustCorePrivateJwk)

        val payload = ed25519Signer.sign("abc".toByteArray())

        assertNotNull(payload)
    }

    @Test
    fun `test signer with key manager`() {
        val privateJwk = rustCoreEd25519GeneratorGenerate()

        val keyManager = InMemoryKeyManager(listOf())
        val publicJwk = keyManager.importPrivateJwk(privateJwk)

        val ed25519Signer = keyManager.getSigner(publicJwk)
        val payload = ed25519Signer.sign("abc".toByteArray())

        assertNotNull(payload)
    }
}