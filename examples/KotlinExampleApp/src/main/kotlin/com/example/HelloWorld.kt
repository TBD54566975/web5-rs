package com.example

import web5.sdk.Jwk
import web5.sdk.computeThumbprint

fun main(args: Array<String>) {
    println("Hello, World!")
    val jwk = Jwk("", "EC", "secp256k1", "", "IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY", "UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA")
    val thumbprint = computeThumbprint(jwk)
    println("Computed thumbprint: $thumbprint")
}