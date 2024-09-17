package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.ToOuterSigner
import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.signers.ToInnerSigner
import web5.sdk.rust.JwkData
import web5.sdk.rust.JwkData as RustCoreJwkData
import web5.sdk.rust.KeyManager as RustCoreKeyManager
import web5.sdk.rust.Signer as RustCoreSigner

/**
 * Interface representing a key management system.
 *
 * The `KeyManager` interface defines methods for importing private JSON Web Keys (JWKs) and retrieving signers for public JWKs.
 */
interface KeyManager {
    /**
     * Imports a private JWK and returns the corresponding public JWK.
     *
     * @param privateJwk The private JWK to import.
     * @return The public JWK after import.
     */
    fun importPrivateJwk(privateJwk: Jwk): Jwk

    /**
     * Retrieves a signer for a given public JWK.
     *
     * @param publicJwk The public JWK for which to retrieve the signer.
     * @return The signer associated with the public JWK.
     */
    fun getSigner(publicJwk: Jwk): Signer
}

/**
 * Adapter class to convert a Rust core `KeyManager` to a Kotlin `KeyManager`.
 *
 * This class provides a bridge between Rust core key management logic and Kotlin key management functionality.
 */
internal class ToOuterKeyManager(private val rustCoreKeyManager: RustCoreKeyManager) : KeyManager {
    /**
     * Imports a private JWK using Rust core and returns the corresponding public JWK.
     *
     * @param privateJwk The private JWK to import.
     * @return The public JWK after import.
     */
    override fun importPrivateJwk(privateJwk: Jwk): Jwk {
        val rustCoreJwkData = rustCoreKeyManager.importPrivateJwk(privateJwk.rustCoreJwkData)
        return Jwk.fromRustCoreJwkData(rustCoreJwkData)
    }

    /**
     * Retrieves a signer for a given public JWK using Rust core.
     *
     * @param publicJwk The public JWK for which to retrieve the signer.
     * @return The signer associated with the public JWK.
     */
    override fun getSigner(publicJwk: Jwk): Signer {
        val rustCoreSigner = rustCoreKeyManager.getSigner(publicJwk.rustCoreJwkData)
        return ToOuterSigner(rustCoreSigner)
    }
}

/**
 * Adapter class to convert a Kotlin `KeyManager` to a Rust core `KeyManager`.
 *
 * This class provides a bridge to adapt Kotlin key management logic for use in Rust core.
 */
internal class ToInnerKeyManager(private val keyManager: KeyManager) : RustCoreKeyManager {
    /**
     * Imports a private JWK using Kotlin logic and converts it to Rust core format.
     *
     * @param privateJwk The private JWK in Rust core format.
     * @return The public JWK in Rust core format.
     */
    override fun importPrivateJwk(privateJwk: JwkData): JwkData {
        val rustCoreJwkData = Jwk.fromRustCoreJwkData(privateJwk)
        val jwk = keyManager.importPrivateJwk(rustCoreJwkData)
        return jwk.rustCoreJwkData
    }

    /**
     * Retrieves a signer for a given public JWK using Kotlin logic and converts it to Rust core format.
     *
     * @param publicJwk The public JWK in Rust core format.
     * @return The signer in Rust core format.
     */
    override fun getSigner(publicJwk: RustCoreJwkData): RustCoreSigner {
        val jwk = Jwk.fromRustCoreJwkData(publicJwk)
        val signer = keyManager.getSigner(jwk)
        val innerSigner = ToInnerSigner(signer)
        return innerSigner
    }
}
