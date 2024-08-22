package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.ToOuterSigner
import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.signers.ToInnerSigner
import web5.sdk.rust.JwkData
import web5.sdk.rust.JwkData as RustCoreJwkData
import web5.sdk.rust.KeyManager as RustCoreKeyManager
import web5.sdk.rust.Signer as RustCoreSigner

interface KeyManager {
    fun importPrivateJwk(privateJwk: Jwk): Jwk
    fun getSigner(publicJwk: Jwk): Signer
}

internal class ToOuterKeyManager(private val rustCoreKeyManager: RustCoreKeyManager) : KeyManager {
    override fun importPrivateJwk(privateJwk: Jwk): Jwk {
        val rustCoreJwkData = rustCoreKeyManager.importPrivateJwk(privateJwk.rustCoreJwkData)
        return Jwk.fromRustCoreJwkData(rustCoreJwkData)
    }

    override fun getSigner(publicJwk: Jwk): Signer {
        val rustCoreSigner = rustCoreKeyManager.getSigner(publicJwk.rustCoreJwkData)
        return ToOuterSigner(rustCoreSigner)
    }
}

internal class ToInnerKeyManager(private val keyManager: KeyManager) : RustCoreKeyManager {
    override fun importPrivateJwk(privateJwk: JwkData): JwkData {
        val rustCoreJwkData = Jwk.fromRustCoreJwkData(privateJwk)
        val jwk = keyManager.importPrivateJwk(rustCoreJwkData)
        return jwk.rustCoreJwkData
    }

    override fun getSigner(publicJwk: RustCoreJwkData): RustCoreSigner {
        val jwk = Jwk.fromRustCoreJwkData(publicJwk)
        val signer = keyManager.getSigner(jwk)
        val innerSigner = ToInnerSigner(signer)
        return innerSigner
    }
}
