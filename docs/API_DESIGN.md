# Web5 API Design (APID) <!-- omit in toc -->

**Last Updated** May 30, 2024

**Version** 1.0.0

**[Custom DSL](./CUSTOM_DSL.md) Version**: 0.1.0

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
- [Verifiable Credentials (VCs)](#verifiable-credentials-vcs)
  - [Data Model 1.1](#data-model-11)
    - [`NamedIssuer`](#namedissuer)
    - [`VerifiableCredential`](#verifiablecredential)
      - [Example: Create a VC \& sign](#example-create-a-vc--sign)
      - [Example: Verify a VC-JWT](#example-verify-a-vc-jwt)
- [Presentation Exchange (PEX)](#presentation-exchange-pex)
  - [`PresentationDefinition`](#presentationdefinition)
  - [`InputDescriptor`](#inputdescriptor)
  - [`Constraints`](#constraints)
  - [`Field`](#field)
  - [`Optionality`](#optionality)
  - [`Filter`](#filter)

> [!NOTE]
> Refer to the [Custom DSL](./CUSTOM_DSL.md) for below syntax definitions.

# Crypto

## `Jwk`

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

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

## Key Managers 

### `KeyManager`

```pseudocode!
INTERFACE KeyManager
  /// Returns the signer for the given public key.
  METHOD get_signer(public_jwk: Jwk): Signer
```

### `InMemoryKeyManager`

```pseudocode!
/// An encapsulation of key material stored in-memory.
CLASS InMemoryKeyManager IMPLEMENTS KeyManager
  CONSTRUCTOR(private_jwks: []Jwk)

  /// Returns the signer for the given public key.
  METHOD get_signer(public_jwk: Jwk): Signer

  /// For importing keys which may be stored somewhere such as environment variables. Return Jwk is the public key for the given private key.
  METHOD import_jwk(private_jwk: Jwk): Jwk
```

## Digital Signature Algorithms (DSA)

### `Dsa`

```pseudocode!
/// The set of Digital Signature Algorithms natively supported within this SDK.
ENUM Dsa
  Ed25519
```

> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

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
  CONSTRUCTOR(private_key: Jwk)

  /// Implementation of Signer's sign instance method for Ed25519.
  METHOD sign(payload: []byte): []byte
```

### `Ed25519Verifier`

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
  METHOD find_public_key_jwk(key_id: string): Jwk
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
  CONSTRUCTOR resolve(uri: string)
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
  CONSTRUCTOR(public_key: Jwk)
  CONSTRUCTOR(uri: string)
  STATIC METHOD resolve(uri: string): ResolutionResult
```

#### Example: Create a `did:jwk`

```pseudocode!
key_manager = new InMemoryKeyManager()
public_key = key_manager.generate_key_material()
did_jwk = new DidJwk(public_key)
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

```pseudocode!
CLASS DidWeb
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
public_key = key_manager.generate_key_material()
did_dht = new DidDht(public_key)
signer = key_manager.get_signer(public_key)
did_dht.publish(signer)
```

#### Example: Create a `did:dht`, add to the Core Properties & publish

> [!NOTE]
> The call to the `new DidDht()` constructor only adds the minimum requirements to the DID Document.
> If additional [Core Properties](https://www.w3.org/TR/did-core/#core-properties) are required, update the `document` data member prior-to the call to `publish()`.

```pseudocode!
key_manager = new InMemoryKeyManager()
public_key = key_manager.generate_key_material()
did_dht = new DidDht(public_key)

/// Set the alsoKnownAs
did_dht.document.alsoKnownAs = "did:example:efgh"
/// Note: you could also add a verification method, set the controller etc.

signer = key_manager.get_signer(public_key)
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
public_key = key_manager.import_key(private_key) /// assume private_key pre-exists, eg. read from env var
signer = key_manager.get_signer(public_key)

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
  CONSTRUCTOR(did: Did, key_manager: KeyManager)
  METHOD get_signer(): Signer
```

> [!WARNING]
>
> We'll need to add support for the developer to select a VM other than defaulting to the first in the `verification_method` array; add `METHOD get_signer_by_kid(key_id: string): Signer`.

# Verifiable Credentials (VCs)

## Data Model 1.1

### `NamedIssuer`

```pseudocode!
CLASS NamedIssuer
  PUBLIC DATA id: string
  PUBLIC DATA name: string
```

### `VerifiableCredential`

> [!WARNING]
> The following is incomplete in that an `Object` is not currently supported in the Custom DSL; the matter of the `Object` below is a placeholder and expected to be completed in a subsequent version.

> [!WARNING]
> We need to consider default behaviors such as always including the base `@context` and `type`

```pseudocode!
CLASS VerifiableCredential
  PUBLIC DATA @context: []string
  PUBLIC DATA id: string
  PUBLIC DATA type: []string
  PUBLIC DATA issuer: string | NamedIssuer
  PUBLIC DATA issuanceDate: string
  PUBLIC DATA expirationDate: string?
  PUBLIC DATA credentialSubject: Object  # 🚧 `Object` not supported 🚧
  CONSTRUCTOR(context: []string, id: string, type: []string, issuer: string | NamedIssuer, issuanceDate: string, expirationDate: string?)
  CONSTRUCTOR(vcjwt: string)
  CONSTRUCTOR(vcjwt: string, verifier: Verifier)
  METHOD sign(signer: Signer): string
```

> [!NOTE]
>
> `CONSTRUCTOR(vcjwt: string)` and `CONSTRUCTOR(vcjwt: string, verifier: Verifier)` both execute cryprographic verification and assume `vcjwt` is a compact serialized JWS wherein the `kid` JOSE Header is equal to a DID URI which can be dereferenced to fetch the [`publicKeyJwk`](./did.md#data-models).

#### Example: Create a VC & sign

```pseudocode!
key_manager = new InMemoryKeyManager()
public_key = key_manager.generate_key_material()

did_jwk = new DidJwk(public_key)

context = ["https://www.w3.org/2018/credentials/v1"]
id = "urn:vc:uuid:123456"
type = ["VerifiableCredential"]
issuer = did_jwk.did.uri
issuance_date = DateTime.now()
vc = new VerifiableCredential(context, id, type, issuer, issuance_date, null)

signer = key_manager.get_signer(public_key)
vcjwt = vc.sign(signer)
```

#### Example: Verify a VC-JWT

```pseudocode!
vcjwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOmUzMDc0OWVhLTg4YjctNDkwMi05ZTRlLWYwYjk1MTRjZmU1OSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTlc5Q1pGaE1OM1JEV0MxaVdtd3dOblU1V1dSU1dqQmFhbEpMTFVweFZXWmhaa1YzWjAwdFRHSm1heUo5IiwiaXNzdWFuY2VEYXRlIjoxNzE2MzEyNDU3LCJleHBpcmF0aW9uRGF0ZSI6MjM0NzQ2NDQ1NywiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lOVzlDWkZoTU4zUkRXQzFpV213d05uVTVXV1JTV2pCYWFsSkxMVXB4VldaaFprVjNaMDB0VEdKbWF5SjkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSIsImV4cCI6MjM0NzQ2NDQ1NywibmJmIjoxNzE2MzEyNDU3LCJqdGkiOiJ1cm46dmM6dXVpZDplMzA3NDllYS04OGI3LTQ5MDItOWU0ZS1mMGI5NTE0Y2ZlNTkifQ.a8ciqXyNgqttWPKl76CFwDTRvEoJEq5nndfM1UMkClvzhPOUWSUtE0wNHOxQFwUBBSbwozScBNe-dc-mWQFqAQ"

vc = VerifiableCredential.verify(vcjwt)
```

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
