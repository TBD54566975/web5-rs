# Web5 API Design (APID) <!-- omit in toc -->

**Last Updated** May 30, 2024

**Version** 2.1.0

**[Custom DSL](./CUSTOM_DSL.md) Version**: 0.2.0

- [Credentials](#credentials)
  - [Verifiable Credentials (VCs)](#verifiable-credentials-vcs)
    - [Data Model 1.1](#data-model-11)
      - [`VerifiableCredential`](#verifiablecredential)
        - [`CredentialSubject`](#credentialsubject)
        - [`Issuer`](#issuer)
        - [`CreateOptions`](#createoptions)
  - [Presentation Exchange (PEX)](#presentation-exchange-pex)
    - [`PresentationDefinition`](#presentationdefinition)
    - [`InputDescriptor`](#inputdescriptor)
    - [`Constraints`](#constraints)
    - [`Field`](#field)
    - [`Optionality`](#optionality)
    - [`Filter`](#filter)
- [Crypto](#crypto)
  - [`Jwk`](#jwk)
  - [Key Managers](#key-managers)
    - [`KeyManager`](#keymanager)
    - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [Digital Signature Algorithms (DSA)](#digital-signature-algorithms-dsa)
    - [`Dsa`](#dsa)
    - [`Signer`](#signer)
    - [`Verifier`](#verifier)
    - [`Ed25519Generator`](#ed25519generator)
    - [`Ed25519Signer`](#ed25519signer)
    - [`Ed25519Verifier`](#ed25519verifier)
- [Decentralized Identifier's (DIDs)](#decentralized-identifiers-dids)
  - [`Did`](#did)
    - [Example: Instantiate from a `did:dht`](#example-instantiate-from-a-diddht)
  - [Data Model](#data-model)
    - [`Document`](#document)
    - [`VerificationMethod`](#verificationmethod)
    - [`Service`](#service)
  - [Resolution](#resolution)
    - [`ResolutionResult`](#resolutionresult)
    - [`ResolutionMetadataError`](#resolutionmetadataerror)
    - [`ResolutionMetadata`](#resolutionmetadata)
    - [`DocumentMetadata`](#documentmetadata)
  - [Methods](#methods)
    - [`DidJwk`](#didjwk)
      - [Example: Create a `did:jwk`](#example-create-a-didjwk)
      - [Example: Instantiate an existing `did:jwk`](#example-instantiate-an-existing-didjwk)
      - [Example: Resolve a `did:jwk`](#example-resolve-a-didjwk)
    - [`DidWeb`](#didweb)
      - [Example: Instantiate an existing `did:web`](#example-instantiate-an-existing-didweb)
      - [Example: Resolve a `did:web`](#example-resolve-a-didweb)
    - [`DidDht`](#diddht)
      - [Example: Create \& publish a `did:dht`](#example-create--publish-a-diddht)
      - [Example: Create a `did:dht`, add to the Core Properties \& publish](#example-create-a-diddht-add-to-the-core-properties--publish)
      - [Example: Instantiate an existing `did:dht`](#example-instantiate-an-existing-diddht)
      - [Example: Update a `did:dht`](#example-update-a-diddht)
      - [Example: Resolve a `did:dht`](#example-resolve-a-diddht)
  - [`BearerDid`](#bearerdid)
    - [Example: Instantiate from a `PortableDid`](#example-instantiate-from-a-portabledid)
  - [`PortableDid`](#portabledid)
    - [Example: Create a `PortableDid` via the `web5` CLI](#example-create-a-portabledid-via-the-web5-cli)

> [!NOTE]
> Refer to the [Custom DSL](./CUSTOM_DSL.md) for below syntax definitions.

# Credentials

## Verifiable Credentials (VCs)

### Data Model 1.1

> [!WARNING]
> We are currently missing `credentialStatus`, `credentialSchema` and `evidence`

#### `VerifiableCredential`

```pseudocode!
CLASS VerifiableCredential
  PUBLIC DATA @context: []string
  PUBLIC DATA id: string
  PUBLIC DATA type: []string
  PUBLIC DATA issuer: Issuer
  PUBLIC DATA issuance_date: datetime
  PUBLIC DATA expiration_date: datetime?
  PUBLIC DATA credentialSubject: CredentialSubject

  CONSTRUCTOR create(issuer: Issuer, credential_subject: CredentialSubject, options: CreateOptions)

  CONSTRUCTOR(vcjwt: string)
  CONSTRUCTOR(vcjwt: string, verifier: Verifier)
  METHOD sign(bearer_did: BearerDid): string
  METHOD sign_with_signer(key_id: string, signer: Signer): string
```

##### `CredentialSubject`

`Object` with at least a non-empty `id: string` data member.

##### `Issuer`

`Object` or `string`, and if `Object` then at least non-empty `id: string` and `name: string` data members.

##### `CreateOptions`

```psuedocode!
CLASS CreateOptions
  PUBLIC DATA id: string?
  PUBLIC DATA context: []string?
  PUBLIC DATA type: []string?
  PUBLIC DATA issuance_date: datetime?
  PUBLIC DATA expiration_date: datetime?
```

## Presentation Exchange (PEX)

### `PresentationDefinition` 

```pseudocode!
CLASS PresentationDefinition
  PUBLIC DATA id: string
  PUBLIC DATA name: string?
  PUBLIC DATA purpose: string?
  PUBLIC DATA input_descriptors: []InputDescriptor
  METHOD select_credentials(vc_jwts: []string): []string
```

### `InputDescriptor` 

```pseudocode!
CLASS InputDescriptor
  PUBLIC DATA id: string
  PUBLIC DATA name: string?
  PUBLIC DATA purpose: string?
  PUBLIC DATA constraints: Constraints
```

### `Constraints`

```pseudocode!
CLASS Constraints
  PUBLIC DATA fields: []Field
```

### `Field`

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

### `Optionality`

```pseudocode!
ENUM Optionality
  Required
  Preferred
```

### `Filter`

```pseudocode!
CLASS Filter
  PUBLIC DATA type: string?
  PUBLIC DATA pattern: string?
  PUBLIC DATA const_value: string?
  PUBLIC DATA contains: Filter?
```

# Crypto

## `Jwk`

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_jwk_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

```pseudocode!
/// Partial representation of a [JSON Web Key as per RFC7517](https://tools.ietf.org/html/rfc7517).
/// Note that this is a subset of the spec.
CLASS Jwk
  /// Identifies the algorithm intended for use with the key.
  PUBLIC DATA alg: string?

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

## Key Managers 

### `KeyManager`

```pseudocode!
INTERFACE KeyManager
  /// Returns the signer for the given public key.
  METHOD get_signer(public_jwk: Jwk): Signer
```

### `InMemoryKeyManager`

The `InMemoryKeyManager` manages private keys in working memory, and so therefore any production utilization of the instance may be exposed to memory safety vulnerabilities; the `InMemoryKeyManager` is primarily intended for development & testing environments. For cases wherein this is unacceptable, [`KeyManager`](#keymanager) & [`Signer`](#signer) are both polymorphic bases classes which can be implemented and utilized in the dependent areas.

```pseudocode!
/// An encapsulation of key material stored in-memory.
CLASS InMemoryKeyManager IMPLEMENTS KeyManager
  CONSTRUCTOR(private_jwks: []Jwk)

  /// Returns the signer for the given public key.
  METHOD get_signer(public_jwk: Jwk): Signer

  /// For importing keys which may be stored somewhere such as environment variables. Return Jwk is the public key for the given private key.
  METHOD import_private_jwk(private_jwk: Jwk): Jwk
```

## Digital Signature Algorithms (DSA)

### `Dsa`

```pseudocode!
/// The set of Digital Signature Algorithms natively supported within this SDK.
ENUM Dsa
  Ed25519
```

> We must add support for `X25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

### `Signer`

```pseudocode!
/// Set of functionality required to implement to be a compatible DSA signer.
INTERFACE Signer
  /// Signs the given payload by using the encapsulated private key material.
  METHOD sign(payload: []byte): []byte
```

### `Verifier`

```pseudocode!
/// Set of functionality required to implement to be a compatible DSA verifier.
INTERFACE Verifier
  /// Execute the verification of the signature against the payload by using the encapsulated public key material.
  METHOD verify(payload: []byte, signature: []byte): bool
```

### `Ed25519Generator`

```pseudocode!
/// Generates private key material for Ed25519.
CLASS Ed25519Generator
  /// Generate the private key material; return Jwk includes private key material.
  STATIC METHOD generate(): Jwk
```

### `Ed25519Signer`

```pseudocode!
/// Implementation of [`Signer`](#signer) for Ed25519.
CLASS Ed25519Signer IMPLEMENTS Signer
  CONSTRUCTOR(private_jwk: Jwk)

  /// Implementation of Signer's sign instance method for Ed25519.
  METHOD sign(payload: []byte): []byte
```

### `Ed25519Verifier`

```pseudocode!
/// Implementation of [`Verifier`](#verifier) for Ed25519.
CLASS Ed25519Verifier IMPLEMENTS Verifier
  CONSTRUCTOR(public_jwk: Jwk)

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

  CONSTRUCTOR parse(uri: string)
```

### Example: Instantiate from a `did:dht`

```pseudocode!
uri = "did:dht:i9xkp8ddcbcg8jwq54ox699wuzxyifsqx4jru45zodqu453ksz6y"
did = new Did(uri)
```

## Data Model

### `Document`

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

  /// Return the Jwk from the Verification Method with the matching key ID.
  METHOD find_public_jwk_jwk(key_id: string): Jwk
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

## Resolution

### `ResolutionResult`

```pseudocode!
/// Representation of the result of a DID (Decentralized Identifier) resolution.
CLASS ResolutionResult
  /// The resolved DID document, if available.
  PUBLIC DATA document: Document

  /// The metadata associated with the DID document.
  PUBLIC DATA document_metadata: DocumentMetadata

  /// The metadata associated with the DID resolution process.
  PUBLIC DATA resolution_metadata: ResolutionMetadata

  /// Resolve via a DID URI.
  CONSTRUCTOR(uri: string)
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
  CONSTRUCTOR(public_jwk: Jwk)
  CONSTRUCTOR(uri: string)
  STATIC METHOD resolve(uri: string): ResolutionResult
```

#### Example: Create a `did:jwk`

```pseudocode!
key_manager = new InMemoryKeyManager()
public_jwk = key_manager.import_private_jwk(Ed25519Generator::generate())
did_jwk = new DidJwk(public_jwk)
```

#### Example: Instantiate an existing `did:jwk`

```pseudocode!
uri = "did:jwk:eyJrdHkiOiJFQyIsInVzZSI6InNpZyIsImNydiI6InNlY3AyNTZrMSIsImtpZCI6ImkzU1BSQnRKS292SEZzQmFxTTkydGk2eFFDSkxYM0U3WUNld2lIVjJDU2ciLCJ4IjoidmRyYnoyRU96dmJMRFZfLWtMNGVKdDdWS04VEZaTm1BOVlnV3p2aGg3VSIsInkiOiJWTEZxUU1aUF9Bc3B1Y1hvV1gyLWJHWHBBTzFmUTVMbjE5VjVSQXhyZZVIiwiYWxnIjoiRVMyNTZLIn0"
did_jwk = new DidJwk(uri)
```

#### Example: Resolve a `did:jwk`

```pseudocode!
uri = "did:jwk:eyJrdHkiOiJFQyIsInVzZSI6InNpZyIsImNydiI6InNlY3AyNTZrMSIsImtpZCI6ImkzU1BSQnRKS292SEZzQmFxTTkydGk2eFFDSkxYM0U3WUNld2lIVjJDU2ciLCJ4IjoidmRyYnoyRU96dmJMRFZfLWtMNGVKdDdWS04VEZaTm1BOVlnV3p2aGg3VSIsInkiOiJWTEZxUU1aUF9Bc3B1Y1hvV1gyLWJHWHBBTzFmUTVMbjE5VjVSQXhyZZVIiwiYWxnIjoiRVMyNTZLIn0"
resolution_result = DidJwk.resolve(uri)
```

### `DidWeb`

> [!NOTE]
>
> The `CONSTRUCTOR(domain: string, public_jwk: Jwk)` does not publish the DID Document to a host, but merely creates the instance of the `did:web` in the local scope. 

```pseudocode!
CLASS DidWeb
  CONSTRUCTOR(domain: string, public_jwk: Jwk)
  CONSTRUCTOR(uri: string)
  STATIC METHOD resolve(uri: string): ResolutionResult
```

#### Example: Instantiate an existing `did:web`

```pseudocode!
uri = "did:web:example.com"
did_web = new DidWeb(uri)
```

#### Example: Resolve a `did:web`

```pseudocode!
uri = "did:web:example.com"
resolution_result = DidWeb.resolve(uri)
```

### `DidDht`

```pseudocode!
CLASS DidDht
  PUBLIC DATA did: Did
  PUBLIC DATA document: Document
  CONSTRUCTOR(identity_key: Jwk)
  CONSTRUCTOR(uri: string)
  METHOD publish(signer: Signer)
  METHOD deactivate(signer: Signer)
  STATIC METHOD resolve(uri: string): ResolutionResult
```

> [!NOTE]
> `resolve()` makes use of [`Ed25519Verifier`](#ed25519verifier) internally for DNS packet verification.

#### Example: Create & publish a `did:dht`

```pseudocode!
key_manager = new InMemoryKeyManager()
identity_key = key_manager.import_private_jwk(Ed25519Generator::generate())
did_dht = new DidDht(identity_key)
signer = key_manager.get_signer(identity_key)
did_dht.publish(signer)
```

#### Example: Create a `did:dht`, add to the Core Properties & publish

> [!NOTE]
> The call to the `new DidDht()` constructor only adds the minimum requirements to the DID Document.
> If additional [Core Properties](https://www.w3.org/TR/did-core/#core-properties) are required, update the `document` data member prior-to the call to `publish()`.

```pseudocode!
key_manager = new InMemoryKeyManager()
identity_key = key_manager.import_private_jwk(Ed25519Generator::generate())
did_dht = new DidDht(identity_key)

/// Set the alsoKnownAs
did_dht.document.alsoKnownAs = "did:example:efgh"
/// Note: you could also add a verification method, set the controller etc.

signer = key_manager.get_signer(identity_key)
did_dht.publish(signer)
```

#### Example: Instantiate an existing `did:dht`

```pseudocode!
uri = "did:dht:i9xkp8ddcbcg8jwq54ox699wuzxyifsqx4jru45zodqu453ksz6y"
did_dht = new DidDht(uri)
```

#### Example: Update a `did:dht`

```pseudocode!
uri = "did:dht:i9xkp8ddcbcg8jwq54ox699wuzxyifsqx4jru45zodqu453ksz6y"
did_dht = new DidDht(uri)

/// Set the alsoKnownAs
did_dht.document.alsoKnownAs = "did:example:efgh"
/// Note: you could also add a verification method, set the controller etc.

key_manager = new InMemoryKeyManager()
public_jwk = key_manager.import_private_jwk(private_jwk) /// assume private_jwk pre-exists, eg. read from env var
signer = key_manager.get_signer(public_jwk)

did_dht.publish(signer)
```

#### Example: Resolve a `did:dht`

```pseudocode!
uri = "did:dht:i9xkp8ddcbcg8jwq54ox699wuzxyifsqx4jru45zodqu453ksz6y"
resolution_result = DidDht.resolve(uri)
```

## `BearerDid`

```pseudocode!
CLASS BearerDid
  PUBLIC DATA did: Did
  PUBLIC DATA document: Document
  PUBLIC DATA key_manager: KeyManager
  CONSTRUCTOR(uri: string, key_manager: KeyManager)
  CONSTRUCTOR(portable_did: PortableDid)
  METHOD get_signer(): Signer
```

> [!WARNING]
>
> We'll need to add support for the developer to select a VM other than defaulting to the first in the `verification_method` array; add `METHOD get_signer_by_kid(key_id: string): Signer`.

### Example: Instantiate from a [`PortableDid`](#portabledid)

```pseudocode!
portable_did = new PortableDid(env.get("PORTABLE_DID_JSON"))
bearer_did = new BearerDid(portable_did)
```

## `PortableDid`

The `PortableDid` is a JSON serialized representation of a `BearerDid`.

> [!WARNING]
>
> The `PortableDid` contains **serialized private key material** and it is therefore **NOT SAFE** for production environments; the `PortableDid` is primarily intended for usage in development & testing environments.

```pseudocode!
CLASS PortableDid
  DATA MEMBER uri: string
  DATA MEMBER private_jwks: []Jwk
  DATA MEMBER document: Document
  CONSTRUCTOR(json: string)
```

### Example: Create a [`PortableDid`](#portabledid) via the `web5` CLI

> [!NOTE]
>
> Notice the `--no-indent` and `--json-escape` options for ease of use copy & paste.

```shell
#/bin/bash

web5 did create dht
{
  "uri": "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry",
  "document": {
    "id": "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry",
    "verificationMethod": [
      {
        "id": "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry#0",
        "type": "JsonWebKey",
        "controller": "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry",
        "publicKeyJwk": {
          "alg": "Ed25519",
          "kty": "OKP",
          "crv": "Ed25519",
          "x": "0JmDpHt23UJCgXyQUleKwzw8CT4rVcIQRODqWrpfeUg"
        }
      }
    ],
    "authentication": [
      "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry#0"
    ],
    "assertionMethod": [
      "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry#0"
    ],
    "capabilityInvocation": [
      "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry#0"
    ],
    "capabilityDelegation": [
      "did:dht:4nca8jd5q5qwrowbx1efrihkac6danj6fpkhrrnrhdifiq19xfry#0"
    ]
  },
  "privateKeys": [
    {
      "alg": "Ed25519",
      "kty": "OKP",
      "crv": "Ed25519",
      "d": "Kxk8IvhpNgBl965xtCm7l0ZHpSc4f02IhrOOPdG1jBY",
      "x": "0JmDpHt23UJCgXyQUleKwzw8CT4rVcIQRODqWrpfeUg"
    }
  ]
}
```