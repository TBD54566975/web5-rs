package web5.sdk.crypto.signers

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.SystemTarget
import web5.sdk.rust.Ed25519Signer as RustCoreEd25519Signer
import web5.sdk.rust.Signer as RustCoreSigner

class Ed25519Signer : RustCoreSigner {
    init {
        SystemTarget.set() // ensure the sys arch is set for first-time loading
    }

    private val rustCoreSigner: RustCoreEd25519Signer

    constructor(privateKey: Jwk) {
        this.rustCoreSigner = RustCoreEd25519Signer(privateKey)
    }

    /**
     * Implementation of Signer's sign instance method for Ed25519.
     *
     * @param payload the data to be signed.
     * @return ByteArray the signature.
     */
    override fun sign(payload: ByteArray): ByteArray {
        return rustCoreSigner.sign(payload)
    }
}
