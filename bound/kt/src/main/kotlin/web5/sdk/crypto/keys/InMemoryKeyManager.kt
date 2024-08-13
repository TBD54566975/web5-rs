package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Signer
import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

/**
 * A class for managing cryptographic keys in-memory.
 */
class InMemoryKeyManager : KeyManager {
    private val rustCoreInMemoryKeyManager = RustCoreInMemoryKeyManager()

    /**
     * Constructs an InMemoryKeyManager with the given private keys.
     *
     * @param privateJwks A list of private keys represented as JWKs (JSON Web Keys).
     */
    constructor(privateJwks: List<Jwk>) {
        privateJwks.forEach {
            this.rustCoreInMemoryKeyManager.importPrivateJwk(it)
        }
    }

    /**
     * Returns the Signer for the given public key.
     *
     * @param publicJwk The public key represented as a JWK.
     * @return Signer The signer for the given public key.
     */
    override fun getSigner(publicJwk: Jwk): Signer {
        return this.rustCoreInMemoryKeyManager.getSigner(publicJwk)
    }

    /**
     * Imports a private key which may be stored somewhere such as environment variables.
     *
     * @param privateJwk The private key represented as a JWK.
     * @return Jwk The public key represented as a JWK.
     */
    fun importPrivateJwk(privateJwk: Jwk): Jwk {
        return this.rustCoreInMemoryKeyManager.importPrivateJwk(privateJwk)
    }
}