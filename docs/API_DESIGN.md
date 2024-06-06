> [!WARNING]
> 🚧 Under Construction 🚧
>
> - Incomplete doc comments
> - Missing examples
> - Missing test vectors
> - Incomplete did:dht design
> - VC `Object` not supported
> - custom DSL lacks errors
> - only a single key manager currently

# Web5 API Design (APID) <!-- omit in toc -->

**Last Updated** May 30, 2024

**Version** 0.1.0

**[Custom DSL](./CUSTOM_DSL.md) Version**: 0.1.0

- [Examples](#examples)
  - [Creating a did:dht](#creating-a-diddht)
  - [Creating a VC](#creating-a-vc)
  - [Verifying a VC](#verifying-a-vc)
- [Key Material](#key-material)
  - [`Jwk`](#jwk)
  - [`InMemoryKeyManager`](#inmemorykeymanager)
- [Digital Signature Algorithm's (DSA)](#digital-signature-algorithms-dsa)
  - [`Dsa`](#dsa)
  - [`Signer`](#signer)
  - [`Verifier`](#verifier)
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

> [!NOTE]
> Refer to the [Custom DSL](./CUSTOM_DSL.md) for below syntax definitions.

# Examples

...

## Creating a did:dht

...

## Creating a VC

...

## Verifying a VC

...

# Key Material

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

## `Jwk`

```pseudocode!
/// Partial representation of a [JSON Web Key as per RFC7517](https://tools.ietf.org/html/rfc7517).
/// Note that this is a subset of the spec.
CLASS Jwk
  /// Identifies the algorithm intended for use with the key.
  PUBLIC DATA alg: string

  /// Represents the key type. e.g. EC for elliptic curve, OKP for Edwards curve
  PUBLIC DATA kty: string

  /// curve name for Elliptic Curve (EC) and Edwards Curve (Ed) keys.
  /// e.g. secp256k1, Ed25519
  PUBLIC DATA crv: string

  /// X coordinate for EC keys, or the public key for OKP.
  PUBLIC DATA x: string

  /// Y coordinate for EC keys.
  PUBLIC DATA y: string?

  /// Private key component for EC or OKP keys.
  PUBLIC DATA d: string?
```

## `InMemoryKeyManager`

```pseudocode!
/// An encapsulation of key material stored in-memory.
CLASS InMemoryKeyManager
  /// Generates new key material and returns the public key represented as a Jwk.
  METHOD generate_key_material(): Jwk

  /// Returns the Ed25519Signer for the given public key.
  METHOD get_signer(public_key: Jwk): Ed25519Signer

  /// For importing keys which may be stored somewhere such as environment variables.
  METHOD import_key(private_key: Jwk)
```

# Digital Signature Algorithm's (DSA)

## `Dsa`

```pseudocode!
/// The set of Digital Signature Algorithm's natively supported within this SDK.
ENUM Dsa
  Ed25519
```

> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

## `Signer`

```pseudocode!
/// Set of functionality required to implement to be a compatible DSA signer.
INTERFACE Signer
  /// Signs the given payload by using the encapsulated private key material.
  METHOD sign(payload: []byte): []byte
```

## `Verifier`

```pseudocode!
/// Set of functionality required to implement to be a compatible DSA verifier.
INTERFACE Verifier
  /// Execute the verification of the signature against the message by using the encapsulated public key material.
  METHOD verify(message: []byte, signature: []byte): bool
```

## `Ed25519Generator`

```pseudocode!
/// Generates private key material for Ed25519.
CLASS Ed25519Generator
  /// Generate the private key material; return Jwk includes private key material.
  STATIC METHOD generate(): Jwk
```

## `Ed25519Signer`

```pseudocode!
/// Implementation of [`Signer`](#signer) for Ed25519.
CLASS Ed25519Signer IMPLEMENTS Signer
  CONSTRUCTOR(private_key: Jwk)

  /// Implementation of Signer's sign instance method for Ed25519.
  METHOD sign(payload: []byte): []byte
```

## `Ed25519Verifier`

```pseudocode!
/// Implementation of [`Verifier`](#verifier) for Ed25519.
CLASS Ed25519Verifier IMPLEMENTS Verifier
  CONSTRUCTOR(public_key: Jwk)

  /// Implementation of Verifier's dsa_verify instance method for Ed25519.
  METHOD verify(payload: []byte): bool
```

# Decentralized Identifier's (DIDs)

## `Did`

```pseudocode!
/// Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).
CLASS Did
  /// URI represents the complete Decentralized Identifier (DID) URI.
  /// Spec: https://www.w3.org/TR/did-core/#did-syntax.
  PUBLIC DATA uri: string

  /// URL represents the DID URI + A network location identifier for a specific resource.
  /// Spec: https://www.w3.org/TR/did-core/#did-url-syntax.
  PUBLIC DATA url: string

  /// Method specifies the DID method in the URI, which indicates the underlying method-specific identifier scheme (e.g., jwk, dht, key, etc.).
  /// Spec: https://www.w3.org/TR/did-core/#method-schemes.
  PUBLIC DATA method: string

  /// ID is the method-specific identifier in the DID URI.
  /// Spec: https://www.w3.org/TR/did-core/#method-specific-id.
  PUBLIC DATA id: string

  /// Params is a map containing optional parameters present in the DID URI. These parameters are method-specific.
  /// Spec: https://www.w3.org/TR/did-core/#did-parameters.
  PUBLIC DATA params: Map<string, string>?

  /// Path is an optional path component in the DID URI.
  /// Spec: https://www.w3.org/TR/did-core/#path.
  PUBLIC DATA path: string?

  /// Query is an optional query component in the DID URI, used to express a request for a specific representation or resource related to the DID.
  /// Spec: https://www.w3.org/TR/did-core/#query.
  PUBLIC DATA query: string?

  /// Fragment is an optional fragment component in the DID URI, used to reference a specific part of a DID document.
  /// Spec: https://www.w3.org/TR/did-core/#fragment.
  PUBLIC DATA fragment: string?

  CONSTRUCTOR(uri: string)
```

**Examples**

```pseudocode!
🚧
```

## `Document`

```pseudocode!
/// Representation of a [DID Document](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md)
CLASS Document
  /// The DID URI for a particular DID subject is expressed using the id property in the DID document.
  PUBLIC DATA id: string

  PUBLIC DATA @context: []string?

  /// A DID controller is an entity that is authorized to make changes to a
  /// DID document. The process of authorizing a DID controller is defined
  /// by the DID method.
  PUBLIC DATA controller: []string?

  /// A DID subject can have multiple identifiers for different purposes, or at
  /// different times. The assertion that two or more DIDs (or other types of URI)
  /// refer to the same DID subject can be made using the alsoKnownAs property.
  PUBLIC DATA alsoKnownAs: []string?

  /// Cryptographic public keys, which can be used to authenticate or authorize
  /// interactions with the DID subject or associated parties.
  /// [spec reference](https://www.w3.org/TR/did-core/#verification-methods)
  PUBLIC DATA verificationMethod: []VerificationMethod

  /// The authentication verification relationship is used to specify how the
  /// DID subject is expected to be authenticated, for purposes such as logging
  /// into a website or engaging in any sort of challenge-response protocol.
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#key-agreement)
  PUBLIC DATA authentication: []string?

  /// The assertionMethod verification relationship is used to specify how the
  /// DID subject is expected to express claims, such as for the purposes of
  /// issuing a Verifiable Credential
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#assertion)
  PUBLIC DATA assertionMethod: []string?

  /// The keyAgreement verification relationship is used to specify how an
  /// entity can generate encryption material in order to transmit confidential
  /// information intended for the DID subject, such as for the purposes of
  /// establishing a secure communication channel with the recipient
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#key-agreement)
  PUBLIC DATA keyAgreement: []string?

  /// The capabilityInvocation verification relationship is used to specify a
  /// verification method that might be used by the DID subject to invoke a
  /// cryptographic capability, such as the authorization to update the
  /// DID Document
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#capability-invocation)
  PUBLIC DATA capabilityInvocation: []string?

  /// The capabilityDelegation verification relationship is used to specify a
  /// mechanism that might be used by the DID subject to delegate a
  /// cryptographic capability to another party, such as delegating the
  /// authority to access a specific HTTP API to a subordinate.
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#capability-delegation)
  PUBLIC DATA capabilityDelegation: []string?

  /// Services are used in DID documents to express ways of communicating with
  /// the DID subject or associated entities.
  /// A service can be any type of service the DID subject wants to advertise.
  ///
  /// [Specification Reference](https://www.w3.org/TR/did-core/#services)
  PUBLIC DATA service: []Service?
```

### `VerificationMethod`

```pseudocode!
/// Representation of a DID Document's [Verification Method](https://www.w3.org/TR/did-core/#verification-methods).
CLASS VerificationMethod
  /// 🚧
  PUBLIC DATA id: string

  /// 🚧
  PUBLIC DATA type: string

  /// 🚧
  PUBLIC DATA controller: string

  /// 🚧
  PUBLIC DATA publicKeyJwk: Jwk
```

### `Service`

```pseudocode!
/// Representation of a DID Document's [Service](https://www.w3.org/TR/did-core/#service).
CLASS Service
  /// 🚧
  PUBLIC DATA id: string

  /// 🚧
  PUBLIC DATA type: string

  /// 🚧
  PUBLIC DATA serviceEndpoint: []string
```

## `Resolution`

```pseudocode!
/// Representation of the result of a DID (Decentralized Identifier) resolution.
CLASS Resolution
  /// The resolved DID document, if available.
  PUBLIC DATA document: Document

  /// The metadata associated with the DID document.
  PUBLIC DATA document_metadata: DocumentMetadata

  /// The metadata associated with the DID resolution process.
  PUBLIC DATA resolution_metadata: ResolutionMetadata

  /// Resolve via a DID URI.
  STATIC METHOD resolve(uri: string): Resolution
```

**Examples**

```pseudocode!
🚧
```

### `ResolutionMetadataError`

```pseudocode!
/// The error code from the resolution process.
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
/// Metadata about the given resolution.
CLASS ResolutionMetadata
  /// The error code from the resolution process.
  PUBLIC DATA error: ResolutionMetadataError?
```

### `DocumentMetadata`

```pseudocode!
/// Metadata about the DID Document.
CLASS DocumentMetadata
  /// 🚧
  PUBLIC DATA created: string?

  /// 🚧
  PUBLIC DATA updated: string?

  /// 🚧
  PUBLIC DATA deactivated: bool?

  /// 🚧
  PUBLIC DATA nextUpdate: string?

  /// 🚧
  PUBLIC DATA versionId: string?

  /// 🚧
  PUBLIC DATA nextVersionId: string?

  /// 🚧
  PUBLIC DATA equivalentId: []string?

  /// 🚧
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

> [!WARNING]
> The following is considered to be incomplete and must be completed in a subsequent version; the design definition below is intended to be a placeholder.

```pseudocode!
CLASS DidDht
  PUBLIC DATA did: Did
  PUBLIC DATA document: Document
  STATIC METHOD create(identity_key: Jwk, options: DidDhtCreateOptions): DidDht
  STATIC METHOD resolve(uri: string): Resolution
  STATIC METHOD update(signer: Signer) 🚧 incomplete 🚧
  STATIC METHOD deactivate(signer: Signer) 🚧 incomplete 🚧
```

> [!NOTE]
> `resolve()` makes use of [`Ed25519Verifier`](#ed25519verifier) internally for DNS packet verification.

#### `DidDhtCreateOptions`

```pseudocode!
CLASS DidDhtCreateOptions
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

> [!WARNING]
> The following is incomplete in that an `Object` is not currently supported in the Custom DSL; the matter of the `Object` below is a placeholder and expected to be completed in a subsequent version.

```pseudocode!
CLASS VerifiableCredential
  PUBLIC DATA @context: []string
  PUBLIC DATA id: string
  PUBLIC DATA type: []string
  PUBLIC DATA issuer: string | NamedIssuer
  PUBLIC DATA issuanceDate: string
  PUBLIC DATA expirationDate: string?
  PUBLIC DATA credentialSubject: Object  # 🚧 `Object` not supported 🚧
  METHOD sign(signer: Signer): string
  STATIC METHOD verify(vcjwt: string): VerifiableCredential
  STATIC METHOD verify_with_verifier(vcjwt: string, verifier: Verifier): VerifiableCredential
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
