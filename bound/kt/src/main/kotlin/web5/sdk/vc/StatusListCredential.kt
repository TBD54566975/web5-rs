package web5.sdk.vc

import web5.sdk.Json
import web5.sdk.rust.StatusListCredential as RustCoreStatusListCredential

data class StatusListCredential(
    val base: VerifiableCredential,
    internal val rustCoreStatusListCredential: RustCoreStatusListCredential
) {
    companion object {
        fun create(
            issuer: Issuer,
            statusPurpose: String,
            credentialsToDisable: List<VerifiableCredential>? = null
        ): StatusListCredential {
            val jsonSerializedIssuer = Json.stringify(issuer)
            val rustCoreCredentials = credentialsToDisable?.map { it.rustCoreVerifiableCredential }

            val rustCoreStatusListCredential = RustCoreStatusListCredential.create(jsonSerializedIssuer, statusPurpose, rustCoreCredentials)

            val baseVerifiableCredential = VerifiableCredential.fromRustCore(rustCoreStatusListCredential.getBase())

            return StatusListCredential(baseVerifiableCredential, rustCoreStatusListCredential)
        }
    }

    fun isDisabled(credential: VerifiableCredential): Boolean {
        return rustCoreStatusListCredential.isDisabled(credential.rustCoreVerifiableCredential)
    }
}