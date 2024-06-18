package web5.sdk.crypto.signers

import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import org.junit.jupiter.api.Assertions.assertNotNull
import web5.sdk.crypto.signers.Ed25519Signer

import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer
import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate
import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

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
        val keyManager = InMemoryKeyManager()
        val publicJwk = keyManager.generateKeyMaterial()

        val ed25519Signer = keyManager.getSigner(publicJwk)
        val payload = ed25519Signer.sign("abc".toByteArray())

        assertNotNull(payload)
    }
}