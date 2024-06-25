package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Signer
import web5.sdk.rust.KeyManager as RustCoreKeyManager

/**
 * An interface representing a key manager for cryptographic operations.
 */
interface KeyManager {

    /**
     * Returns the signer for the given public key.
     *
     * @param publicKey The public key represented as a Jwk.
     * @return Signer The signer for the given public key.
     */
    fun getSigner(publicJwk: Jwk): Signer

    /**
     * Returns the RustCoreKeyManager
     *
     * @return RustCoreKeyManager The rust core key manager
     */
    fun getRustCoreKeyManager(): RustCoreKeyManager
}
