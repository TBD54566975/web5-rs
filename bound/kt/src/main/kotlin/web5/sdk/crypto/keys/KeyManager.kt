package web5.sdk.crypto.keys

import web5.sdk.rust.KeyManagerInterface as RustCoreKeyManagerInterface


typealias KeyManagerInterface = RustCoreKeyManagerInterface
//
///**
// * An interface representing a key manager for cryptographic operations.
// */
//interface KeyManager {
//    /**
//     * Generates new key material and returns the public key represented as a Jwk.
//     *
//     * @return Jwk The generated public key.
//     */
//    fun generateKeyMaterial(): Jwk
//
//    /**
//     * Returns the signer for the given public key.
//     *
//     * @param publicKey The public key represented as a Jwk.
//     * @return Signer The signer for the given public key.
//     */
//    fun getSigner(publicKey: Jwk): Signer
//
//    /**
//     * Imports a key which may be stored somewhere such as environment variables.
//     * Returns the public key for the given private key.
//     *
//     * @param privateKey The private key represented as a Jwk.
//     * @return Jwk The public key for the given private key.
//     */
//    fun importKey(privateKey: Jwk): Jwk
//}
