package web5.sdk.crypto

import web5.sdk.rust.Dsa as RustCoreDsa

enum class Dsa {
    ED25519,
    SECP256K1;
}

internal fun dsaFromRustCore(rustCore: RustCoreDsa): Dsa {
    return when (rustCore) {
        RustCoreDsa.ED25519 -> Dsa.ED25519
        RustCoreDsa.SECP256K1 -> Dsa.SECP256K1
    }
}

internal fun dsaToRustCore(dsa: Dsa): RustCoreDsa {
    return when(dsa) {
        Dsa.ED25519 -> RustCoreDsa.ED25519
        Dsa.SECP256K1 -> RustCoreDsa.SECP256K1
    }
}