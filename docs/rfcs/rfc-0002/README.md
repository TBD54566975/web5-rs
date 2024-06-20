# RFC-0001 Standard Web5 API Design (APID) Document v0.1.0 <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
- [Detailed Design](#detailed-design)
  - [Known Limitations](#known-limitations)
- [Drawbacks](#drawbacks)
- [Alternatives](#alternatives)
- [Prior Art](#prior-art)

# Summary

🚧

# Motivation

🚧

We need Bearer DIDs for tbDEX API.

# Detailed Design

🚧

Semantic version (semver) breaking change induced by:

1. Removal of `InMemoryKeyManager`'s `METHOD generate_key_material()`: this is nonsensical since the key manager implementation stores keys in memory and does not expose the functionality to export keys. The only use case would be for short-lived ephermal use cases, in which case, the developer can use the `Ed25519Generator` to generate the keys and call `METHOD import_jwk()` or the new `CONSTRUCTOR(private_jwks: []Jwk)`.
2. Changed the return type of `METHOD get_signer()` to be a `Signer` to support polymorphism.
3. Renamed `METHOD import_key()` to `import_jwk()` as to be consistent with naming elsewhere.

## Known Limitations

- `BearerDid` missing `METHOD get_signer_by_kid(key_id: string): Signer`.
- `VerifiableCredential` missing `sign_with_bearer_did(bearer_did: BearerDid, key_id?: string)`

# Drawbacks

🚧

# Alternatives

🚧

# Prior Art

🚧