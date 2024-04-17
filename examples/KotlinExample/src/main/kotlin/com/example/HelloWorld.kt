package com.example

import web5.sdk.Jwk
import web5.sdk.SomeTrait
import web5.sdk.SomeTraitA
import web5.sdk.SomeTraitB

fun main(args: Array<String>) {
    val jwk = Jwk(
        "ES256K",
        "EC",
        "secp256k1",
        "P3hRuve79GaggsVdQG_w-JpdM6dHXG33-1nwZ8Jw07g",
        "vA8umEbOhhQjFfk1-byvVxtJNRtwQSEE0UMVmxSN9K4",
        "A1qGUBx-wpznzVI0DLu8kEhDZ77ou533NKSCw90R33Q")
    val thumbprint = jwk.computeThumbprint()
    println("Computed thumbprint: $thumbprint")

    val someTraitA = SomeTraitA()
    println(someTraitA.someFunc())
}