package web5.sdk

import org.junit.jupiter.api.Test
import web5.sdk.Jwk

class JwkTests {
    @Test
    fun jwkTest() {
        val jwk = Jwk("", "EC", "secp256k1", null, "IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY", "UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA")
        val thumbprint = jwk.computeThumbprint()
        println(thumbprint)
    }
}