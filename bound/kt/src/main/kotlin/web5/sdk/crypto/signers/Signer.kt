package web5.sdk.crypto.signers

/**
 * An interface defining the contract for signing signatures on payloads.
 */
interface Signer {
    /**
     * Signs the provided payload.
     *
     * @param payload The data to be signed.
     * @return ByteArray The signature.
     */
    fun sign(payload: ByteArray): ByteArray
}
