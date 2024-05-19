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

# Web5 Object-Oriented API Design <!-- omit in toc -->

This API Design assumes the implementing language supports object-oriented programming (OOP). Static methods which are not for the case of instantiation (a "constructor") are forbidden. Polymorphism is supported through the use of interfaces.

- [JWK](#jwk)
  - [`Jwk`](#jwk-1)
- [Key Management](#key-management)
  - [`KeyManager` (Interface)](#keymanager-interface)
  - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [`Signer`](#signer)
  - [`Curve`](#curve)
- [JWS](#jws)
  - [`Jws`](#jws-1)
  - [`JwsHeader`](#jwsheader)
- [JWT](#jwt)
  - [`Jwt` (class)](#jwt-class)
  - [`JwtClaims`](#jwtclaims)
- [DIDs](#dids)
  - [`Identifier`](#identifier)
  - [`Document`](#document)
  - [`DocumentMetadata`](#documentmetadata)
  - [`ResolutionMetadata`](#resolutionmetadata)
  - [`Resolution`](#resolution)
  - [`BearerDid`](#bearerdid)
  - [`DidJwk` TODO](#didjwk-todo)
  - [`DidWeb` (class)](#didweb-class)
  - [`DidDht` (class)](#diddht-class)
- [Credentials](#credentials)
  - [`VerifiableCredential` (class)](#verifiablecredential-class)

# JWK

## `Jwk`

Data properties conformant with [RFC7517](https://datatracker.ietf.org/doc/html/rfc7517).

| Instance Method                  | Notes                                  |
| -------------------------------- | -------------------------------------- |
| `compute_thumbprint() -> string` | RECOMMENDED to be used as a key alias. |

# Key Management

## `KeyManager` (Interface)

| Instance Method                                  | Notes                                    |
| ------------------------------------------------ | ---------------------------------------- |
| `generate_private_key(curve: Curve) -> string`   | Return string is equal to the key alias. |
| `get_public_key(alias: string) -> Jwk `          |                                          |
| `sign(alias: string, payload: []byte) -> []byte` |                                          |


## `InMemoryKeyManager`

Implementation of `KeyManager` which stores key material in-memory.

| Constructor                                      | Notes |
| ------------------------------------------------ | ----- |
| `new(private_keys: []Jwk) -> InMemoryKeyManager` |       |

## `Signer`

| Constructor                                             | Notes |
| ------------------------------------------------------- | ----- |
| `new(key_manager: KeyManager, alias: string) -> Signer` |       |

| Instance Method                   | Notes |
| --------------------------------- | ----- |
| `sign(payload: []byte) -> []byte` |       |

## `Curve`

Open Issue on naming [#38](https://github.com/TBD54566975/web5-rs/issues/38).

| Enumeration |
| ----------- |
| `Ed25519`   |
| `Secp256k1` |

# JWS

## `Jws`

| Property                     | Notes |
| ---------------------------- | ----- |
| `header: JwsHeader`          |       |
| `payload: []byte`            |       |
| `signature: string`          |       |
| `compact_serialized: string` |       |
| `json_serialized: string`    |       |

| Constructor                                                         | Notes                                                                                                                                            |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `create(header: JwsHeader, payload: []byte, signer: Signer) -> Jws` |                                                                                                                                                  |
| `from_compact_serialized(compact_serialized: string) -> Jws`        | This will perform cryptographic verification by accessing the public key via revolving the DID Document as specified in the `JwsHeader`'s `kid`. |

## `JwsHeader`

Data properties conformant with [Section 4. of RFC7515](https://datatracker.ietf.org/doc/html/rfc7515#section-4).

# JWT

## `Jwt` (class)

| Property            | Notes |
| ------------------- | ----- |
| `claims: JwtClaims` |       |
| `jws: Jws`          |       |

| Constructor                                                            | Notes                                                                                                                  |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `new(claims: JwtClaims, jws_header: JwsHeader, signer: Signer) -> Jwt` |                                                                                                                        |
| `from_jwt(jwt: string) -> Jwt`                                         | This will perform cryptographic verification by accessing the public key from the DID Document resolved via the `kid`. |

## `JwtClaims`

Data properties conformant to [RFC7519](https://datatracker.ietf.org/doc/html/rfc7519#section-4). 

| Property                   | Notes |
| -------------------------- | ----- |
| `vc: VerifiableCredential` |       |

# DIDs

## `Identifier`

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

| Constructor                          | Notes |
| ------------------------------------ | ----- |
| `new(did_uri: string) -> Identifier` |       |

## `Document`

Data properties conformant to [DID Document Data Model
 in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

## `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

## `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the we5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

## `Resolution`

| Property                                  | Notes |
| ----------------------------------------- | ----- |
| `document: Document`                      |       |
| `document_metadata: DocumentMetadata`     |       |
| `resolution_metadata: ResolutionMetadata` |       |

| Constructor                          | Notes |
| ------------------------------------ | ----- |
| `new(did_uri: string) -> Resolution` |       |

## `BearerDid`

| Property                  | Notes |
| ------------------------- | ----- |
| `identifier: Identifier`  |       |
| `document: Document`      |       |
| `key_manager: KeyManager` |       |

## `DidJwk` TODO

| Function                                                     | Notes |
| ------------------------------------------------------------ | ----- |
| `create(key_manager: KeyManager, curve: Curve) -> BearerDid` |       |
| `resolve(did_uri) -> Resolution`                             |       |

## `DidWeb` (class)

| Function                         | Notes |
| -------------------------------- | ----- |
| `resolve(did_uri) -> Resolution` |       |

## `DidDht` (class)

| Function                         | Notes |
| -------------------------------- | ----- |
| `resolve(did_uri) -> Resolution` |       |

# Credentials

## `VerifiableCredential` (class)

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Function                                                                            | Notes |
| ----------------------------------------------------------------------------------- | ----- |
| `sign_vcjwt(self, bearer_did: BearerDid, verification_method_id: string) -> string` |       |
| `verify_vcjwt(vcjwt: string) -> VerifiableCredential`                               |       |