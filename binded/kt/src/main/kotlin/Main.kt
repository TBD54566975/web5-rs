import web5.sdk.DidJwk
import web5.sdk.InMemoryKeyManager

fun main() {
    println("Hello, World Start!")

    val keyManager = InMemoryKeyManager()
    val jwkData = keyManager.generateKeyMaterial()
    val didJwk = DidJwk.fromPublicKey(jwkData)

    println(didJwk.getData().did)
    println("Hello, World End!")
}