package web5.sdk.vc

import web5.sdk.Json
import web5.sdk.Web5Exception
import web5.sdk.rust.Web5Exception.Exception as RustCoreException
import web5.sdk.rust.StatusListCredential as RustCoreStatusListCredential

/**
 * Represents a Status List Credential, which is used to maintain the revocation or suspension status of multiple Verifiable Credentials.
 * A Status List Credential is a special type of Verifiable Credential that tracks the status of other credentials.
 *
 * @property base The base Verifiable Credential associated with the Status List.
 */
data class StatusListCredential(
    val base: VerifiableCredential,
    internal val rustCoreStatusListCredential: RustCoreStatusListCredential
) {
    companion object {
        /**
         * Creates a new Status List Credential with the specified issuer, status purpose,
         * and the list of disabled credentials.
         *
         * @param issuer The entity issuing the Status List Credential.
         * @param statusPurpose The purpose of the status (e.g., "revocation").
         * @param credentialsToDisable A list of Verifiable Credentials that are disabled (revoked or suspended).
         * @return A new [StatusListCredential] instance.
         * @throws Web5Exception if there is an error in creating the Status List Credential.
         *
         * Example usage:
         * ```
         * val issuerBearerDid = DidJwk.create(null)
         * val subjectDidUri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy"
         * val verifiableCredential = VerifiableCredential.create(
         *     Issuer.StringIssuer(issuerBearerDid.did.uri),
         *     CredentialSubject(id = subjectDidUri),
         *     VerifiableCredentialCreateOptions(
         *         credentialStatus = CredentialStatus(
         *             id = "https://example.com/status/1",
         *             type = "StatusList2021Entry",
         *             statusPurpose = "revocation",
         *             statusListIndex = "3",
         *             statusListCredential = "https://example.com/status/1"
         *         )
         *     )
         * )
         * val statusListCredential = StatusListCredential.create(
         *     Issuer.StringIssuer(issuerBearerDid.did.uri),
         *     "revocation",
         *     listOf(verifiableCredential)
         * )
         * ```
         */
        fun create(
            issuer: Issuer,
            statusPurpose: String,
            credentialsToDisable: List<VerifiableCredential>? = null
        ): StatusListCredential {
            try {
                val jsonSerializedIssuer = Json.stringify(issuer)
                val rustCoreCredentials = credentialsToDisable?.map { it.rustCoreVerifiableCredential }

                val rustCoreStatusListCredential = RustCoreStatusListCredential.create(jsonSerializedIssuer, statusPurpose, rustCoreCredentials)

                val baseVerifiableCredential = VerifiableCredential.fromRustCore(rustCoreStatusListCredential.getBase())

                return StatusListCredential(baseVerifiableCredential, rustCoreStatusListCredential)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }

    /**
     * Checks if a given credential is disabled according to this Status List Credential.
     *
     * @param credential The [VerifiableCredential] to check.
     * @return `true` if the credential is disabled, `false` otherwise.
     * @throws Web5Exception if there is an error while checking the status of the credential.
     *
     * Example usage:
     * ```
     * val issuerBearerDid = DidJwk.create(null)
     * val subjectDidUri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy"
     *
     * val verifiableCredential = VerifiableCredential.create(
     *     Issuer.StringIssuer(issuerBearerDid.did.uri),
     *     CredentialSubject(id = subjectDidUri),
     *     VerifiableCredentialCreateOptions(
     *         credentialStatus = CredentialStatus(
     *             id = "https://example.com/status/1",
     *             type = "StatusList2021Entry",
     *             statusPurpose = "revocation",
     *             statusListIndex = "3",
     *             statusListCredential = "https://example.com/status/1"
     *         )
     *     )
     * )
     *
     * val statusListCredential = StatusListCredential.create(
     *     Issuer.StringIssuer(issuerBearerDid.did.uri),
     *     "revocation",
     *     listOf(verifiableCredential)
     * )
     *
     * val isDisabled = statusListCredential.isDisabled(verifiableCredential)
     * ```
     */
    fun isDisabled(credential: VerifiableCredential): Boolean {
        try {
            return rustCoreStatusListCredential.isDisabled(credential.rustCoreVerifiableCredential)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}