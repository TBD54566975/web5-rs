> [!WARNING]
> 🚧 🚧 🚧 Under Construction 🚧 🚧 🚧
>
> 🚧 Documentation comments
> 
> 🚧 Test vectors

# Standard Web5 API Design (APID) Document <!-- omit in toc -->

**Last Updated:** May 30, 2024

**Version:** 0.1.0

- [Custom DSL](#custom-dsl)
  - [Limitations](#limitations)
  - [Primitive Concepts](#primitive-concepts)
  - [High-Level Concepts](#high-level-concepts)
    - [Polymorphic Base Class](#polymorphic-base-class)
    - [Class](#class)
    - [Enumeration](#enumeration)
- [Key Material](#key-material)
  - [`Jwk`](#jwk)
  - [`LocalKeyManager`](#localkeymanager)
- [Digital Signature Algorithm's (DSA)](#digital-signature-algorithms-dsa)
  - [`Dsa`](#dsa)
  - [`DsaSigner`](#dsasigner)
  - [`DsaVerifier`](#dsaverifier)
  - [`JwsSigner`](#jwssigner)
  - [`JwsVerifier`](#jwsverifier)
  - [`Ed25519Generator`](#ed25519generator)
  - [`Ed25519Signer`](#ed25519signer)
  - [`Ed25519Verifier`](#ed25519verifier)
- [Decentralized Identifier's (DIDs)](#decentralized-identifiers-dids)
  - [`Did`](#did)
  - [`Document`](#document)
    - [`VerificationMethod`](#verificationmethod)
    - [`Service`](#service)
  - [`Resolution`](#resolution)
    - [`ResolutionMetadataError`](#resolutionmetadataerror)
    - [`ResolutionMetadata`](#resolutionmetadata)
    - [`DocumentMetadata`](#documentmetadata)
  - [Methods](#methods)
    - [`DidJwk`](#didjwk)
    - [`DidWeb`](#didweb)
    - [`DidDht`](#diddht)
      - [`DidDhtCreateOptions`](#diddhtcreateoptions)
- [Verifiable Credential's (VCs)](#verifiable-credentials-vcs)
  - [`NamedIssuer`](#namedissuer)
  - [`VerifiableCredential`](#verifiablecredential)
- [Presentation Exchange (PEX)](#presentation-exchange-pex)
  - [`PresentationDefinition`](#presentationdefinition)
  - [`InputDescriptor`](#inputdescriptor)
  - [`Constraints`](#constraints)
  - [`Field`](#field)
  - [`Optionality`](#optionality)
  - [`Filter`](#filter)

# Custom DSL

The design definitions within this design document are intended to span any programming language, so long as the given programming language supports the [High-Level Concepts](#high-level-concepts) and [Primitive Concepts](#primitive-concepts) in one form or another. The instantiations of these concepts will be unique to the given idioms of the target programming language.

## Limitations

In order to achieve the goal of defining concrete design definitions which span multiple languages, we must make some sacrifices in our design. Namely, this design excludes ***generics*** and ***variadic function parameters***, because both lack broad support & consistency across target programming languages. Implementations may choose to utilize these concepts in their internals, but the publicly accessible Web5 API must exclude these concepts.

The APID does not assert requirements as to the artifact makeup (i.e. npm packages, rust crates, go modules, etc.) of the Web5 API. It is recommended to implement the entirety of Web5 in a single artifact, but each implementation may choose to create multiple artifacts. However, the APID makes no regards for the matter of circular dependencies, and so it may become unviable to implement the APID in it's completeness across multiple artifacts.

## Primitive Concepts

| Type              | Representation                          | Description |
| ----------------- | --------------------------------------- | ----------- |
| string            | `string`                                | 🚧           |
| byte              | `byte`                                  | 🚧           |
| boolean           | `bool`                                  | 🚧           |
| integer           | `int`                                   | 🚧           |
| array             | `[]T`                                   | 🚧           |
| optional/nullable | `T?`                                    | 🚧           |
| hash map          | `Map<T1, T2>`                           | 🚧           |
| function          | `func_name(param1: T1, param2: T2): T3` | 🚧           |
| mixed type        | `T1 \| T2`                              | 🚧           |

## High-Level Concepts

### Polymorphic Base Class

- `INTERFACE InterfaceName`: Defines a a polymorphic base class.
- `METHOD methodName(param: T1): T2`: Defines an instance method that any class implementing the interface must implement.

**Example:**

```psuedocode
INTERFACE Shape
  METHOD area(): int
  METHOD perimeter(): int
```

> [!NOTE]
> Polymorphic base class definitions may have a `CONSTRUCTOR` to indicate assumptions of encapsulation for implementations; given a target language does not support constructor's on the polymorphic base class, then the feature can be disregarded.

### Class

- `CLASS ClassName`: Defines a class.
- `IMPLEMENTS InterfaceName`: Defines a class implementation of a polymorphic base class.
- `PUBLIC DATA memberName: T`: Type: Defines a public data member.
- `CONSTRUCTOR(param: T1)`: Defines a constructor for a class.
- `METHOD methodName(param: T1): T2`: Defines an instance method on the class.
- `STATIC METHOD methodName(param: T1): T2`: Defines an instance method on the class.

**Example:**

```psuedocode
CLASS Circle IMPLEMENTS Shape
  PUBLIC DATA radius: int
  CONSTRUCTOR(radius: int)
  METHOD area(): int
  METHOD perimeter(): int
  STATIC METHOD unit_circle(): Circle
```

> [!NOTE]
> `STATIC METHOD`'s may be implemented on a `CLASS` given the implementation language supports the feature, but else can be a function (not associated with a `CLASS`).

### Enumeration

- `ENUM EnumName`: Defines an enumeration.

**Example:**

```psuedocode
ENUM Color
  RED
  GREEN
  BLUE
```

# Key Material

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

## `Jwk`

Partial representation of a [JSON Web Key](https://datatracker.ietf.org/doc/html/rfc7517).

```pseudocode!
CLASS Jwk
  PUBLIC DATA alg: string
  PUBLIC DATA kty: string
  PUBLIC DATA crv: string
  PUBLIC DATA x: string
  PUBLIC DATA y: string?
  PUBLIC DATA d: string?
```

## `LocalKeyManager`

An encapsulation of key material stored in-memory.

```pseudocode!
CLASS LocalKeyManager
  METHOD generate_key_material(): Jwk
  METHOD get_signer(public_key: Jwk): Ed25519Signer
  METHOD import_key(private_key: Jwk)
```

# Digital Signature Algorithm's (DSA)

## `Dsa`

The set of Digital Signature Algorithm's natively supported within this SDK.

```pseudocode!
ENUM Dsa
  Ed25519
```

> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

## `DsaSigner`

Set of functionality required to implement to be a compatible DSA signer.

```pseudocode!
INTERFACE DsaSigner
  CONSTRUCTOR(private_key: Jwk)
  METHOD dsa_sign(payload: []byte): []byte
```

## `DsaVerifier`

Set of functionality required to implement to be a compatible DSA verifier.

```pseudocode!
INTERFACE DsaVerifier
  CONSTRUCTOR(public_key: Jwk)
  METHOD dsa_verify(message: []byte, signature: []byte): bool
```

## `JwsSigner`

Set of functionality required to implement to be a compatible JWS signer.

```pseudocode!
INTERFACE JwsSigner
  CONSTRUCTOR(private_key: Jwk)
  METHOD jws_sign(payload: []byte): []byte
```

## `JwsVerifier`

Set of functionality required to implement to be a compatible JWS verifier.

```pseudocode!
INTERFACE JwsVerifier
  CONSTRUCTOR(public_key: Jwk)
  METHOD jws_verify(message: []byte, signature: []byte): bool
```

## `Ed25519Generator`

Generates private key material for Ed25519.

```pseudocode!
CLASS Ed25519Generator
  STATIC METHOD generate(): Jwk
```

## `Ed25519Signer`

Implementation of [`DsaSigner`](#dsasigner) and [`JwsSigner`](#jwssigner) for Ed25519.

```pseudocode!
CLASS Ed25519Signer IMPLEMENTS DsaSigner, JwsSigner
  CONSTRUCTOR(private_key: Jwk)
  METHOD dsa_sign(payload: []byte): []byte
  METHOD jws_sign(payload: []byte): []byte
```

## `Ed25519Verifier`

Implementation of [`DsaVerifier`](#dsaverifier) and [`JwsVerifier`](#jwsverifier) for Ed25519.

```pseudocode!
CLASS Ed25519Verifier IMPLEMENTS DsaVerifier, JwsVerifier
  CONSTRUCTOR(public_key: Jwk)
  METHOD dsa_verify(payload: []byte): bool
  METHOD jws_verify(payload: []byte): bool
```

# Decentralized Identifier's (DIDs)

## `Did`

Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).

```pseudocode!
CLASS Did
  PUBLIC DATA uri: string
  PUBLIC DATA url: string
  PUBLIC DATA method: string
  PUBLIC DATA id: string
  PUBLIC DATA params: Map<string, string>?
  PUBLIC DATA path: string?
  PUBLIC DATA query: string?
  PUBLIC DATA fragment: string?
  CONSTRUCTOR(uri: string)
```

## `Document`

```pseudocode!
CLASS Document
  PUBLIC DATA id: string
  PUBLIC DATA @context: []string?
  PUBLIC DATA controller: []string?
  PUBLIC DATA alsoKnownAs: []string?
  PUBLIC DATA verificationMethod: []VerificationMethod
  PUBLIC DATA authentication: []string?
  PUBLIC DATA assertionMethod: []string?
  PUBLIC DATA keyAgreement: []string?
  PUBLIC DATA capabilityInvocation: []string?
  PUBLIC DATA capabilityDelegation: []string?
  PUBLIC DATA service: []Service?
```

### `VerificationMethod`

```pseudocode!
CLASS VerificationMethod
  PUBLIC DATA id: string
  PUBLIC DATA type: string
  PUBLIC DATA controller: string
  PUBLIC DATA publicKeyJwk: Jwk
```

### `Service`

```pseudocode!
CLASS Service
  PUBLIC DATA id: string
  PUBLIC DATA type: string
  PUBLIC DATA serviceEndpoint: []string
```

## `Resolution`

```pseudocode!
CLASS Resolution
  PUBLIC DATA document: Document
  PUBLIC DATA document_metadata: DocumentMetadata
  PUBLIC DATA resolution_metadata: ResolutionMetadata
  STATIC METHOD resolve(uri: string): Resolution
```

### `ResolutionMetadataError`

```pseudocode!
ENUM ResolutionMetadataError
  invalidDid
  notFound
  representationNotSupported
  methodNotSupported
  invalidDidDocument
  invalidDidDocumentLength
  internalError
```

### `ResolutionMetadata`

```pseudocode!
CLASS ResolutionMetadata
  PUBLIC DATA error: ResolutionMetadataError?
```

### `DocumentMetadata`

```pseudocode!
CLASS DocumentMetadata
  PUBLIC DATA created: string?
  PUBLIC DATA updated: string?
  PUBLIC DATA deactivated: bool?
  PUBLIC DATA nextUpdate: string?
  PUBLIC DATA versionId: string?
  PUBLIC DATA nextVersionId: string?
  PUBLIC DATA equivalentId: []string?
  PUBLIC DATA canonicalId: string?
```

## Methods

### `DidJwk`

```pseudocode!
CLASS DidJwk
  PUBLIC DATA did: Did
  PUBLIC DATA document: Document
  STATIC METHOD create(public_key: Jwk): DidJwk
  STATIC METHOD resolve(uri: string): Resolution
```

### `DidWeb`

```pseudocode!
CLASS DidWeb
  STATIC METHOD resolve(uri: string): Resolution
```

### `DidDht`

```pseudocode!
CLASS DidDht
  PUBLIC DATA did: Did
  PUBLIC DATA document: Document
  STATIC METHOD create(signer: DsaSigner, identity_key: Jwk, options: DidDhtCreateOptions): DidDht
  STATIC METHOD resolve(uri: string): Resolution
  STATIC METHOD update() 🚧 incomplete 🚧
  STATIC METHOD deactivate() 🚧 incomplete 🚧
```

#### `DidDhtCreateOptions`

```pseudocode!
CLASS DidDhtCreateOptions
  PUBLIC DATA publish: bool
  PUBLIC DATA also_known_as: []string?
  PUBLIC DATA controller: []string?
  PUBLIC DATA service: []Service?
  PUBLIC DATA registered_type: []RegisteredDidType?
  PUBLIC DATA verification_methods: []VerificationMethod?
```

# Verifiable Credential's (VCs)

## `NamedIssuer`

```pseudocode!
CLASS NamedIssuer
  PUBLIC DATA id: string
  PUBLIC DATA name: string
```

## `VerifiableCredential`

```pseudocode!
CLASS VerifiableCredential
  PUBLIC DATA @context: []string
  PUBLIC DATA id: string
  PUBLIC DATA type: []string
  PUBLIC DATA issuer: string | NamedIssuer
  PUBLIC DATA issuanceDate: string
  PUBLIC DATA expirationDate: string?
  PUBLIC DATA credentialSubject: Object  # 🚧 `Object` not supported 🚧
  METHOD sign(jws_signer: JwsSigner): string
  STATIC METHOD verify(vcjwt: string): VerifiableCredential
  STATIC METHOD verify_with_verifier(vcjwt: string, jws_verifier: JwsVerifier): VerifiableCredential
```

> [!NOTE]
>
> `verify` and `verify_with_verifier` assume `vcjwt` is a compact serialized JWS wherein the `kid` JOSE Header is equal to a DID URI which can be dereferenced to fetch the [`publicKeyJwk`](./did.md#data-models).

# Presentation Exchange (PEX)

## `PresentationDefinition` 

```pseudocode!
CLASS PresentationDefinition
  PUBLIC DATA id: string
  PUBLIC DATA name: string?
  PUBLIC DATA purpose: string?
  PUBLIC DATA input_descriptors: []InputDescriptor
  METHOD select_credentials(vc_jwts: []string): []string
```

## `InputDescriptor` 

```pseudocode!
CLASS InputDescriptor
  PUBLIC DATA id: string
  PUBLIC DATA name: string?
  PUBLIC DATA purpose: string?
  PUBLIC DATA constraints: Constraints
```

## `Constraints`

```pseudocode!
CLASS Constraints
  PUBLIC DATA fields: []Field
```

## `Field`

```pseudocode!
CLASS Field
  PUBLIC DATA id: string?
  PUBLIC DATA name: string?
  PUBLIC DATA path: []string
  PUBLIC DATA purpose: string?
  PUBLIC DATA filter: Filter?
  PUBLIC DATA optional: bool?
  PUBLIC DATA predicate: Optionality?
```

## `Optionality`

```pseudocode!
ENUM Optionality
  Required
  Preferred
```

## `Filter`

```pseudocode!
CLASS Filter
  PUBLIC DATA type: string?
  PUBLIC DATA pattern: string?
  PUBLIC DATA const_value: string?
  PUBLIC DATA contains: Filter?
```
