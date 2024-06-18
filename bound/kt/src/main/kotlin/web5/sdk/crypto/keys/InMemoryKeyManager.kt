package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer

/**
 * A class for managing cryptographic keys in-memory.
 */
class InMemoryKeyManager {
    private val rustCoreKeyManager = RustCoreInMemoryKeyManager()

    /**
     * Generates new key material and returns the public key represented as a Jwk.
     *
     * @return Jwk the public key represented as a Jwk.
     */
    fun generateKeyMaterial(): Jwk {
        val jwkData = rustCoreKeyManager.generateKeyMaterial()
        return Jwk.fromRustCore(jwkData)
    }

    /**
     * Returns the Ed25519Signer for the given public key.
     *
     * @param publicKey the public key represented as a Jwk.
     * @return Ed25519Signer the signer for the given public key.
     */
    fun getSigner(publicKey: Jwk): Ed25519Signer {
        val rustCorePublicKey = publicKey.toRustCore()

        // TODO: This cast will never work
        val rustCoreSigner = rustCoreKeyManager.getSigner(rustCorePublicKey) as RustCoreEd25519Signer
        return Ed25519Signer.fromRustCore(rustCoreSigner)
    }

    /**
     * For importing keys which may be stored somewhere such as environment variables.
     * Returns Jwk which is the public key for the given private key.
     *
     * @param privateKey the private key represented as a Jwk.
     * @return Jwk the public key for the given private key.
     */
    fun importKey(privateKey: Jwk): Jwk {
        val rustCoreJwk = rustCoreKeyManager.importKey(privateKey.toRustCore())
        return Jwk.fromRustCore(rustCoreJwk)
    }
}
