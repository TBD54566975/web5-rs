package com.example

import web5.sdk.Curve
import web5.sdk.LocalKeyManager

fun main(args: Array<String>) {
    val keyManager = LocalKeyManager.newInMemory()
    val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)
    val payload = "hello world".toByteArray().map { it.toUByte() }
    val signature = keyManager.sign(keyAlias, payload)
    val publicKey = keyManager.getPublicKey(keyAlias)
    publicKey.verify(payload, signature.map { it.toUByte() })
    println("Success!")
}