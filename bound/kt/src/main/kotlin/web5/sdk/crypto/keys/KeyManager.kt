package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.ToOuterSigner
import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.signers.ToInnerSigner
import web5.sdk.rust.JwkData as RustCoreJwkData
import web5.sdk.rust.KeyManager as RustCoreKeyManager
import web5.sdk.rust.Signer as RustCoreSigner

interface KeyManager {
    fun getSigner(publicJwk: Jwk): Signer
}

internal class ToOuterKeyManager(private val rustCoreKeyManager: RustCoreKeyManager) : KeyManager {
    override fun getSigner(publicJwk: Jwk): Signer {
        val rustCoreSigner = rustCoreKeyManager.getSigner(publicJwk.rustCoreJwkData)
        return ToOuterSigner(rustCoreSigner)
    }
}

internal class ToInnerKeyManager(private val keyManager: KeyManager) : RustCoreKeyManager {
    override fun getSigner(publicJwk: RustCoreJwkData): RustCoreSigner {
        val jwk = Jwk.fromRustCoreJwkData(publicJwk)
        val signer = keyManager.getSigner(jwk)
        val innerSigner = ToInnerSigner(signer)
        return innerSigner
    }
}
