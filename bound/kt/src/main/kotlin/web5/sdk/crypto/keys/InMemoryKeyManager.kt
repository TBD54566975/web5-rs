package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.ToOuterSigner
import web5.sdk.crypto.signers.Signer
import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager

/**
 * A class for managing cryptographic keys in-memory.
 *
 * @param privateJwks A list of private keys represented as JWKs (JSON Web Keys).
 */
class InMemoryKeyManager (privateJwks: List<Jwk>) : KeyManager, KeyExporter {
    private val rustCoreInMemoryKeyManager = RustCoreInMemoryKeyManager()

    init {
        privateJwks.forEach {
            this.rustCoreInMemoryKeyManager.importPrivateJwk(it.rustCoreJwkData)
        }
    }

    /**
     * Imports a private key which may be stored somewhere such as environment variables.
     *
     * @param privateJwk The private key represented as a JWK.
     * @return Jwk The public key represented as a JWK.
     */
    override fun importPrivateJwk(privateJwk: Jwk): Jwk {
        val rustCoreJwkData = this.rustCoreInMemoryKeyManager.importPrivateJwk(privateJwk.rustCoreJwkData)
        return Jwk.fromRustCoreJwkData(rustCoreJwkData)
    }

    /**
     * Returns the Signer for the given public key.
     *
     * @param publicJwk The public key represented as a JWK.
     * @return Signer The signer for the given public key.
     */
    override fun getSigner(publicJwk: Jwk): Signer {
        val rustCoreSigner = this.rustCoreInMemoryKeyManager.getSigner(publicJwk.rustCoreJwkData)
        return ToOuterSigner(rustCoreSigner)
    }

    override fun exportPrivateJwks(): List<Jwk> {
        val rustCorePrivateJwksData = this.rustCoreInMemoryKeyManager.exportPrivateJwks()
        return rustCorePrivateJwksData.map { Jwk.fromRustCoreJwkData(it) }
    }
}