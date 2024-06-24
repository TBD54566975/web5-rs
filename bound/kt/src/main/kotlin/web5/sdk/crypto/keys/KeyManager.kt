package web5.sdk.crypto.keys

import web5.sdk.crypto.signers.Signer
import web5.sdk.rust.KeyManager as RustCoreKeyManager

interface KeyManager {
    fun getSigner(publicJwk: Jwk): Signer
    fun getRustCoreKeyManager(): RustCoreKeyManager
}
