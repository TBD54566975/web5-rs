package web5.sdk.crypto.keys

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.Ed25519Generator
import web5.sdk.Web5Exception

class InMemoryKeyManagerTest {
  @Nested
  @TestInstance(TestInstance.Lifecycle.PER_CLASS)
  inner class ImportPrivateJwk {

    @Test
    fun test_must_be_private_jwk() {

      val keyManager = InMemoryKeyManager(listOf())
      val privateJwk = Ed25519Generator.generate()
      val publicJwk = privateJwk.copy(d = null)

      val exception = assertThrows<Web5Exception> {
        keyManager.importPrivateJwk(publicJwk)
      }

      assertEquals("parameter error private_jwk must be a private key", exception.message)
      assertEquals("Parameter", exception.variant)
    }

    @Test
    fun test_successfully_imports_and_returns_public_jwk() {

      val keyManager = InMemoryKeyManager(listOf())
      val privateJwk = Ed25519Generator.generate()

      val publicJwk = keyManager.importPrivateJwk(privateJwk)

      assertEquals(publicJwk, privateJwk.copy(d = null))
    }
  }

  @Nested
  @TestInstance(TestInstance.Lifecycle.PER_CLASS)
  inner class GetSigner {
    @Test
    fun test_must_be_public_key() {

      val privateJwk = Ed25519Generator.generate()
      val keyManager = InMemoryKeyManager(listOf(privateJwk))

      val exception = assertThrows<Web5Exception> {
        keyManager.getSigner(privateJwk)
      }

      assertEquals("parameter error public_jwk must be a public key", exception.message)
      assertEquals("Parameter", exception.variant)
    }

    @Test
    fun test_not_found() {

      val keyManager = InMemoryKeyManager(listOf())
      val privateJwk = Ed25519Generator.generate()
      val publicJwk = privateJwk.copy(d = null)

      val exception = assertThrows<Web5Exception> {
        keyManager.getSigner(publicJwk)
      }

      assertEquals("not found error signer not found for public_jwk with thumbprint ${publicJwk.computeThumbprint()}", exception.message)
      assertEquals("NotFound", exception.variant)
    }

    @Test
    fun test_found() {
      
      val privateJwk = Ed25519Generator.generate()
      val publicJwk = privateJwk.copy(d = null)
      val keyManager = InMemoryKeyManager(listOf(privateJwk))

      assertDoesNotThrow {
        keyManager.getSigner(publicJwk)
      }
    }
  }

  @Nested
  @TestInstance(TestInstance.Lifecycle.PER_CLASS)
  inner class ExportPrivateJwks {

    @Test
    fun test_export_empty_list() {

      val keyManager = InMemoryKeyManager(listOf())
      val privateJwks = keyManager.exportPrivateJwks()

      assertTrue(privateJwks.isEmpty())
    }

    @Test
    fun test_export_single_key() {

      val privateJwk = Ed25519Generator.generate()
      val keyManager = InMemoryKeyManager(listOf(privateJwk))
      val privateJwks = keyManager.exportPrivateJwks()

      assertEquals(1, privateJwks.size)
      assertEquals(privateJwk, privateJwks[0])
    }

    @Test
    fun test_export_multiple_keys() {

      val privateJwk1 = Ed25519Generator.generate()
      val privateJwk2 = Ed25519Generator.generate()
      val keyManager = InMemoryKeyManager(listOf(privateJwk1, privateJwk2))
      val privateJwks = keyManager.exportPrivateJwks()

      assertEquals(2, privateJwks.size)
      assertTrue(privateJwks.containsAll(listOf(privateJwk1, privateJwk2)))
    }
  }
}