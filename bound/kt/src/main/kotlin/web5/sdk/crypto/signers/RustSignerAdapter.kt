package web5.sdk.crypto.signers

import web5.sdk.rust.SignerInterface as RustSignerInterface

// TODO: Figure out if we need this
class RustSignerAdapter(private val kotlinSigner: Signer) : RustSignerInterface {
    override fun sign(payload: List<UByte>): ByteArray {
        val byteArray = payload.map { it.toByte() }.toByteArray()
        return kotlinSigner.sign(byteArray)
    }
}