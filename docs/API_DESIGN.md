> [!WARNING]
> This is currently a WIP

🚧 Under Construction 🚧
- Can we make `jws_signer` work?
- No circular dependencies
- No peer dependencies (unsure about how this would work in practice, at a minimum DX cannot include determining version compatibility)
- No optional parameters, create new functions if multiple functionalities are needed
- No bleeding abstractions from transitive dependencies
- Terminology
- Packages
- Package dependencies
- Error cases
- What about generics or extending data models? (ex. JWT registered claims + `vc` or `vp` claim)
- "Bring your own key manager"
- There's no way around exposing a jose for two reasons: 
  - JWK is part of the DID Core spec
  - KCC needs decoded jose headers & JWT claims in order to operate
  - We cannot bleed abstractions from underlying dependencies
- consider moving entirely away from `key_alias` to `key_id`
- consider `PrivateJwk` and `PublicJwk`
- why split `KeySigner` & `KeyImporter` away from `KeyManager`?
- can we in some way incorporate namespacing here, as a means of constraining scope? for example, would be nice to just have `Jwt` and have it under a `dids` namespace, which would indicate that's a JWT concept constrained to within the concept of a DID.
- a bunch of examples
- monad pattern?
- cryptographic digest's for tbdex

# Web5 API Design <!-- omit in toc -->

- [Language-Agnostic Concepts](#language-agnostic-concepts)
- [Examples](#examples)
  - [Create a DID and Sign a JWT](#create-a-did-and-sign-a-jwt)
  - [Instantiate an Existing DID, Create a VC, and Sign it](#instantiate-an-existing-did-create-a-vc-and-sign-it)
  - [Verify a VC and Inspect the Credential Subject](#verify-a-vc-and-inspect-the-credential-subject)
  - [Bring-Your-Own Key Manager \& Cryptography, Sign a JWT, and Verify it](#bring-your-own-key-manager--cryptography-sign-a-jwt-and-verify-it)
- [API Reference](#api-reference)
  - [JOSE](#jose)
    - [JWK](#jwk)
      - [`PublicJwk` (Interface)](#publicjwk-interface)
      - [`PrivateJwk` (Interface)](#privatejwk-interface)
      - [`EdDSAPublicJwk`](#eddsapublicjwk)
      - [`EdDSAPrivateJwk`](#eddsaprivatejwk)
      - [`ES256KPublicJwk`](#es256kpublicjwk)
      - [`ES256KPrivateJwk`](#es256kprivatejwk)
    - [JWS](#jws)
      - [`JwsHeader`](#jwsheader)
      - [`Jws`](#jws-1)
      - [`JwsSigner` (Interface)](#jwssigner-interface)
      - [`JwsVerifier` (Interface)](#jwsverifier-interface)
      - [`Ed25519JwsSigner`](#ed25519jwssigner)
      - [`Secp256k1JwsSigner`](#secp256k1jwssigner)
      - [`Ed25519JwsVerifier`](#ed25519jwsverifier)
      - [`Secp256k1JwsVerifier`](#secp256k1jwsverifier)
    - [JWT](#jwt)
      - [`JwtClaims`](#jwtclaims)
      - [`Jwt`](#jwt-1)
  - [Key Management](#key-management)
      - [`PrivateJwkGenerator` (Interface)](#privatejwkgenerator-interface)
      - [`Ed25519PrivateJwkGenerator`](#ed25519privatejwkgenerator)
      - [`Secp256k1PrivateJwkGenerator`](#secp256k1privatejwkgenerator)
      - [`KeyManager` (Interface)](#keymanager-interface)
      - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [DIDs](#dids)
      - [`Identifier`](#identifier)
    - [Data Model](#data-model)
    - [`Document`](#document)
    - [`VerificationMethod`](#verificationmethod)
    - [`Service`](#service)
    - [Resolution](#resolution)
      - [`Resolution`](#resolution-1)
      - [`DocumentMetadata`](#documentmetadata)
      - [`ResolutionMetadata`](#resolutionmetadata)
      - [`BearerDid`](#bearerdid)
    - [Methods](#methods)
      - [`DidJwk`](#didjwk)
      - [`DidWeb`](#didweb)
      - [`DidDht`](#diddht)
  - [Credentials](#credentials)
      - [`VerifiableCredential`](#verifiablecredential)

# Language-Agnostic Concepts

TODO

# Examples

## Create a DID and Sign a JWT

```rust
let bearer_did = DidJwk::create(InMemoryKeyManager::new(), Ed25519PrivateJwkGenerator::new());
let jwt = Jwt::sign(
  bearer_did.get_default_jws_signer(), 
  JwtClaims { iss: bearer_did.identifier.uri }
);
println!(jwt.jws.compact_serialized);
```

## Instantiate an Existing DID, Create a VC, and Sign it

```rust
// existing DID URI & private key material
let did_uri = "did:dht:...";
let private_jwk = serde_json::from_str::<EdDSAPrivateJwk>("{...}")?;

let bearer_did = BearerDid::new(did_uri, InMemoryKeyManager::new(vec![private_jwk])).await?;

let verifiable_credential = VerifiableCredential{
  // todo consider default's for convenience
};
let vcjwt = verifiable_credential.sign_vcjwt(bearer_did.get_default_jws_signer());
// TODO consider default sign() functions return compact_serialized
println!(vcjwt.jws.compact_serialized);
```

## Verify a VC and Inspect the Credential Subject

```rust
println!("todo");
```

## Bring-Your-Own Key Manager & Cryptography, Sign a JWT, and Verify it

```rust
println!("todo");
```

# API Reference

## JOSE

### JWK

#### `PublicJwk` (Interface)

| Instance Method                  | Notes                                                           |
| -------------------------------- | --------------------------------------------------------------- |
| `compute_thumbprint(): string`   | [Specification Reference](https://tools.ietf.org/html/rfc7638). |
| `get_public_key_bytes(): []byte` |                                                                 |

#### `PrivateJwk` (Interface)

| Instance Method                   | Notes |
| --------------------------------- | ----- |
| `get_private_key_bytes(): []byte` |       |
| `to_public_jwk(): PublicJwk`      |       |

#### `EdDSAPublicJwk`

Implementation of `PublicJwk` for EdDSA.

| Property      | Notes |
| ------------- | ----- |
| `alg: string` |       |
| `kty: string` |       |
| `crv: string` |       |
| `x: string`   |       |

#### `EdDSAPrivateJwk`

Implementation of `PrivateJwk` for EdDSA.

| Property      | Notes |
| ------------- | ----- |
| `alg: string` |       |
| `kty: string` |       |
| `crv: string` |       |
| `x: string`   |       |
| `d: string`   |       |

#### `ES256KPublicJwk`

Implementation of `PublicJwk` for ES256K.

| Property      | Notes |
| ------------- | ----- |
| `alg: string` |       |
| `kty: string` |       |
| `crv: string` |       |
| `x: string`   |       |
| `y: string`   |       |

#### `ES256KPrivateJwk`

Implementation of `PrivateJwk` for ES256K.

| Property      | Notes |
| ------------- | ----- |
| `alg: string` |       |
| `kty: string` |       |
| `crv: string` |       |
| `x: string`   |       |
| `y: string`   |       |
| `d: string`   |       |

### JWS

#### `JwsHeader` 

🚧 Consider adding constraints to `web5-spec` 🚧

| Property      | Notes                                                                                                                                                      |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `alg: string` |                                                                                                                                                            |
| `kid: string` | Must be a valid Verification Method `id` per [`web5-spec`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model). |
| `typ: string` |                                                                                                                                                            |

#### `Jws`

| Property                              | Notes |
| ------------------------------------- | ----- |
| `header: JwsHeader`                   |       |
| `payload: []byte`                     |       |
| `signature: string`                   |       |
| `compact_serialized: string`          |       |
| `compact_serialized_detached: string` |       |

| Static Method                                                           | Notes                                                                             |
| ----------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `sign(jws_signer: JwsSigner, payload: []byte): Jws`                     |                                                                                   |
| `sign_compact_detached(jws_signer: JwsSigner, payload: []byte): string` |                                                                                   |
| `verify(jws_verifiers: []JwsVerifier, compact_serialized: string): Jws` | Reference `JwsHeader`'s `crv`    property to match with associated `JwsVerifier`. |

#### `JwsSigner` (Interface)

| Instance Method                 | Notes |
| ------------------------------- | ----- |
| `get_header(): JwsHeader`       |       |
| `sign(payload: []byte): string` |       |

#### `JwsVerifier` (Interface)

| Instance Method                                                           | Notes |
| ------------------------------------------------------------------------- | ----- |
| `get_public_key(jws_header: JwsHeader): PublicJwk`                        |       |
| `verify(public_key: PublicJwk, payload: []byte, signature: string): bool` |       |

#### `Ed25519JwsSigner`

Implementation of `JwsSigner` for Ed25519.

#### `Secp256k1JwsSigner`

Implementation of `JwsSigner` for Secp256k1.

#### `Ed25519JwsVerifier`

Implementation of `JwsVerifier` for Ed25519.

#### `Secp256k1JwsVerifier`

Implementation of `JwsVerifier` for Secp256k1.

### JWT

#### `JwtClaims` 

🚧 Consider adding constraints to `web5-spec` 🚧

| Property                   | Notes |
| -------------------------- | ----- |
| `iss: string`              |       |
| `sub: string`              |       |
| `aud: string`              |       |
| `exp: number`              |       |
| `nbf: number`              |       |
| `iat: number`              |       |
| `jti: string`              |       |
| `vc: VerifiableCredential` |       |

#### `Jwt`

| Property            | Notes |
| ------------------- | ----- |
| `claims: JwtClaims` |       |
| `jws: Jws`          |       |

| Static Method                                                           | Notes                         |
| ----------------------------------------------------------------------- | ----------------------------- |
| `sign(jws_signer: JwsSigner, claims: JwtClaims): string`                | Return string is Compact JWS. |
| `sign_jws(jws_signer: JwsSigner, claims: JwtClaims): Jwt`               |                               |
| `verify(jws_verifiers: []JwsVerifier, compact_serialized: string): Jwt` |                               |

## Key Management

#### `PrivateJwkGenerator` (Interface)

| Instance Method          | Notes |
| ------------------------ | ----- |
| `generate(): PrivateJwk` |       |

#### `Ed25519PrivateJwkGenerator`

Implementation of `PrivateJwkGenerator` for Ed25519.

#### `Secp256k1PrivateJwkGenerator`

Implementation of `PrivateJwkGenerator` for Secp256k1.

#### `KeyManager` (Interface)

| Instance Method                                                               | Notes |
| ----------------------------------------------------------------------------- | ----- |
| `generate_private_key(private_jwk_generator: PrivateJwkGenerator): PublicJwk` |       |
| `get_jws_signer(public_jwk: PublicJwk): JwsSigner`                            |       |

#### `InMemoryKeyManager`

Implementation of `KeyManager` which stores key material in-memory.

| Constructor                               | Notes                 |
| ----------------------------------------- | --------------------- |
| `constructor()`                           |                       |
| `constructor(private_keys: []PrivateJwk)` | For import use cases. |

## DIDs

#### `Identifier`

| Property                      | Notes |
| ----------------------------- | ----- |
| `uri: string`                 |       |
| `url: string`                 |       |
| `method: string`              |       |
| `id: string`                  |       |
| `params: map<string, string>` |       |
| `path: string`                |       |
| `query: string`               |       |
| `fragment: string`            |       |

| Constructor                | Notes |
| -------------------------- | ----- |
| `constructor(uri: string)` |       |

### Data Model

### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

### Resolution

#### `Resolution`

| Property                                  | Notes |
| ----------------------------------------- | ----- |
| `document: Document`                      |       |
| `document_metadata: DocumentMetadata`     |       |
| `resolution_metadata: ResolutionMetadata` |       |

| Static Method                      | Notes                                                                              |
| ---------------------------------- | ---------------------------------------------------------------------------------- |
| `resolve(uri: string): Resolution` | Resolution may require networked invocation, and if should should be asynchronous. |

#### `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

#### `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the we5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

#### `BearerDid`

| Property                  | Notes |
| ------------------------- | ----- |
| `identifier: Identifier`  |       |
| `document: Document`      |       |
| `key_manager: KeyManager` |       |

| Instance Method                       | Notes                                                                |
| ------------------------------------- | -------------------------------------------------------------------- |
| `get_default_jws_signer(): JwsSigner` | Returns the `JwsSigner` associated to the first Verification Method. |

### Methods

#### `DidJwk`

| Static Method                                                                            | Notes |
| ---------------------------------------------------------------------------------------- | ----- |
| `create(key_manager: KeyManager, private_jwk_generator: PrivateJwkGenerator): BearerDid` |       |
| `resolve(uri): Resolution`                                                               |       |

#### `DidWeb`

| Static Method              | Notes |
| -------------------------- | ----- |
| `resolve(uri): Resolution` |       |

#### `DidDht`

| Function                                     | Notes                                   |
| -------------------------------------------- | --------------------------------------- |
| `create(key_manager: KeyManager): BearerDid` | TODO need to enable more for the inputs |
| `update()`                                   | TODO                                    |
| `resolve(uri): Resolution`                   |                                         |

## Credentials

#### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                          | Notes |
| ---------------------------------------- | ----- |
| `sign(jws_signer: JwsSigner): string`    |       |
| `sign_vcjwt(jws_signer: JwsSigner): Jwt` |       |

| Static Method                               | Notes |
| ------------------------------------------- | ----- |
| `verify(jwt: string): VerifiableCredential` |       |