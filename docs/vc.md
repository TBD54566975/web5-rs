# Verifiable Credential <!-- omit in toc -->

### `VerifiableCredential`

Data models conformant to *W3C Verifiable Credentials v1.1* [within the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                              | Notes                                                 |
| :------------------------------------------- | :---------------------------------------------------- |
| `sign(jws_signer: &dyn JwsSigner) -> String` | See [`JwsSigner`](#jwssigner-polymorphic-base-class). |

| Static Method                                                                 | Notes                                                                                                |
| :---------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| `verify_with_defaults(vcjwt: &str) -> VerifiableCredential`                   | Where the natively supported [`Dsa`](#dsa-enumeration)'s are applied for cryptographic verification. |
| `verify(vcjwt: &str, jws_verifier: &dyn JwsVerifier) -> VerifiableCredential` | See [`JwsVerifier`](#jwsverifier-polymorphic-base-class).                                            |

> [!NOTE]
>
> Verification methods assume `vcjwt` is a compact serialized JWS wherein the `kid` JOSE Header is equal to a DID URI which can be dereferenced to fetch the [`publicKeyJwk`](./did.md#data-models).