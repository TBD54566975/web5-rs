package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Signer

import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

/**
 * A class for managing cryptographic keys in-memory.
 */
class InMemoryKeyManager {
    private val rustCoreKeyManager = RustCoreInMemoryKeyManager()

    /**
     * Returns the Ed25519Signer for the given public key.
     *
     * @param publicKey the public key represented as a Jwk.
     * @return Ed25519Signer the signer for the given public key.
     */
    fun getSigner(publicKey: Jwk): Signer {
        return rustCoreKeyManager.getSigner(publicKey)
    }

    /**
     * For importing private keys which may be stored somewhere such as environment variables.
     * Returns Jwk which is the public key for the given private key.
     *
     * @param privateKey the private key represented as a Jwk.
     * @return Jwk the public key for the given private key.
     */
    fun importPrivateKey(privateKey: Jwk): Jwk {
        return rustCoreKeyManager.importPrivateJwk(privateKey)
    }
}
