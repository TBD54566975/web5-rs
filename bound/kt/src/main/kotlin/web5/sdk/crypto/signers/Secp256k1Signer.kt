package web5.sdk.crypto.signers

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Secp256k1Signer as RustCoreSecp256k1Signer
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Implementation of Signer for secp256k1.
 */
class Secp256k1Signer(privateJwk: Jwk) : Signer {
    private val rustCoreSigner = RustCoreSecp256k1Signer(privateJwk.rustCoreJwkData)

    /**
     * Implementation of Signer's sign instance method for secp256k1.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    override fun sign(payload: ByteArray): ByteArray {
        try {
            return rustCoreSigner.sign(payload)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
