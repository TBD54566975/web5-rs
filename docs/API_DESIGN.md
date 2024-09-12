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
        - [`Evidence`](#evidence)
        - [`CredentialStatus`](#credentialstatus)
        - [`VerifiableCredentialCreateOptions`](#verifiablecredentialcreateoptions)
  - [StatusListCredential](#statuslistcredential)
      - [`StatusListCredential`](#statuslistcredential-1)
  - [VerifiablePresentation](#verifiablepresentation)
      - [`VerifiablePresentation`](#verifiablepresentation-1)
      - [`VerifiablePresentationCreateOptions`](#verifiablepresentationcreateoptions)
  - [Presentation Exchange (PEX)](#presentation-exchange-pex)
    - [`PresentationDefinition`](#presentationdefinition)
    - [`InputDescriptor`](#inputdescriptor)
    - [`Constraints`](#constraints)
    - [`Field`](#field)
    - [`Optionality`](#optionality)
    - [`Filter`](#filter)
    - [`PresentationResult`](#presentationresult)
    - [`PresentationSubmission`](#presentationsubmission)
    - [`InputDescriptorMapping`](#inputdescriptormapping)
    - [`SubmissionRequirement`](#submissionrequirement)
    - [`SubmissionRequirementRule`](#submissionrequirementrule)
- [Crypto](#crypto)
  - [`Jwk`](#jwk)
  - [Key Managers](#key-managers)
    - [`KeyManager`](#keymanager)
    - [`KeyExporter`](#keyexporter)
    - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [Digital Signature Algorithms (DSA)](#digital-signature-algorithms-dsa)
    - [`Dsa`](#dsa)
    - [`Signer`](#signer)
    - [`Verifier`](#verifier)
    - [`Ed25519Generator`](#ed25519generator)
    - [`Ed25519Signer`](#ed25519signer)
    - [`Ed25519Verifier`](#ed25519verifier)
    - [`Secp256k1Generator`](#secp256k1generator)
    - [`Secp256k1Signer`](#secp256k1signer)
    - [`Secp256k1Verifier`](#secp256k1verifier)
    - [`X25519Generator`](#x25519generator)
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
      - [`DidJwkCreateOptions`](#didjwkcreateoptions)
    - [`DidWeb`](#didweb)
      - [`DidWebCreateOptions`](#didwebcreateoptions)
    - [`DidDht`](#diddht)
      - [`DidDhtCreateOptions`](#diddhtcreateoptions)
  - [`BearerDid`](#bearerdid)
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
/// Represents a Verifiable Credential according to the W3C Verifiable Credentials Data Model.
/// A Verifiable Credential is a tamper-evident credential that has authorship that can be cryptographically verified.
CLASS VerifiableCredential

  /// A list of contexts used to define the semantic meaning of the data contained in the Verifiable Credential.
  PUBLIC DATA @context: []string
  
  /// The unique identifier for the Verifiable Credential.
  PUBLIC DATA id: string
  
  /// The type(s) of the Verifiable Credential, typically includes "VerifiableCredential".
  PUBLIC DATA type: []string
  
  /// The entity (either a string or an object) that issued the credential.
  PUBLIC DATA issuer: Issuer
  
  /// The subject of the credential, containing claims about the entity being described by the credential.
  PUBLIC DATA credential_subject: CredentialSubject
  
  /// The date and time when the credential was issued.
  PUBLIC DATA issuance_date: datetime
  
  /// The optional expiration date and time after which the credential is no longer valid.
  PUBLIC DATA expiration_date: datetime?
  
  /// The credential status information, if applicable (e.g., revoked or suspended).
  PUBLIC DATA credential_status: CredentialStatus?
  
  /// The credential schema, used to validate the data structure of the credential.
  PUBLIC DATA credential_schema: CredentialSchema?
  
  /// An array of evidence supporting the claims made in the credential.
  PUBLIC DATA evidence: []Evidence?

  /// Creates a new Verifiable Credential with the specified issuer, subject, and optional creation options.
  /// 
  /// @param issuer The entity issuing the credential.
  /// @param credential_subject The subject of the credential containing claims.
  /// @param options Optional parameters for creating the credential, such as schema or status.
  CONSTRUCTOR create(issuer: Issuer, credential_subject: CredentialSubject, options: VerifiableCredentialCreateOptions?)
  
  /// Constructs a Verifiable Credential from a VC JWT (JSON Web Token).
  /// 
  /// @param vc_jwt The Verifiable Credential in JWT format.
  /// @param verify If true, verifies the integrity of the JWT before creating the credential.
  CONSTRUCTOR from_vc_jwt(vc_jwt: string, verify: bool)

  /// Signs the Verifiable Credential using the specified Bearer DID and optional verification method.
  /// 
  /// @param bearer_did The DID used to sign the credential.
  /// @param verification_method_id Optional identifier for the verification method.
  /// @returns A string representing the signed JWT of the Verifiable Credential.
  METHOD sign(bearer_did: BearerDid, verification_method_id: String?): string
```

##### `CredentialSubject`

`Object` with at least a non-empty `id: string` data member.

##### `Issuer`

`Object` or `string`, and if `Object` then at least non-empty `id: string` and `name: string` data members.

##### `Evidence`

`Object` with any data members.

##### `CredentialStatus`

```pseudocode!
/// Represents the status information of a Verifiable Credential.
/// CredentialStatus is used to indicate the revocation or suspension status of a credential.
CLASS CredentialStatus

  /// The unique identifier for the credential status.
  PUBLIC DATA id: string
  
  /// The type(s) of the credential status, typically includes "CredentialStatus".
  PUBLIC DATA type: []string
  
  /// The purpose of the status (e.g., "revocation" or "suspension").
  PUBLIC DATA status_purpose: string
  
  /// The index in the status list indicating the credential's position.
  PUBLIC DATA status_list_index: string
  
  /// The unique identifier for the Verifiable Credential that lists the status of the credential.
  PUBLIC DATA status_list_credential: string
```

##### `VerifiableCredentialCreateOptions`

```psuedocode!
/// Represents the options available when creating a Verifiable Credential.
/// These options allow customization of various attributes of the credential during its creation.
CLASS VerifiableCredentialCreateOptions

  /// The unique identifier for the Verifiable Credential. This is optional.
  PUBLIC DATA id: string?
  
  /// The context(s) for the Verifiable Credential, which define the meaning of terms within the credential.
  PUBLIC DATA context: []string?
  
  /// The type(s) of the Verifiable Credential. Typically includes "VerifiableCredential".
  PUBLIC DATA type: []string?
  
  /// The issuance date of the credential. If not provided, defaults to the current date and time.
  PUBLIC DATA issuance_date: datetime?
  
  /// The optional expiration date for the credential, after which it is no longer valid.
  PUBLIC DATA expiration_date: datetime?
  
  /// The optional credential status, which indicates revocation or suspension information.
  PUBLIC DATA credential_status: CredentialStatus?
  
  /// An optional array of evidence supporting the claims made in the credential.
  PUBLIC DATA evidence: []Evidence?
```

## StatusListCredential

#### `StatusListCredential`

```pseudocode!
/// Represents a Status List Credential, which is used to maintain the revocation or suspension status of multiple Verifiable Credentials.
/// A Status List Credential is a special type of Verifiable Credential that tracks the status of other credentials.
CLASS StatusListCredential

  /// The base Verifiable Credential associated with the Status List.
  PUBLIC DATA base: VerifiableCredential
  
  /// Creates a new Status List Credential with the specified issuer, status purpose, and the list of disabled credentials.
  /// 
  /// @param issuer The entity issuing the Status List Credential.
  /// @param status_purpose The purpose of the status (e.g., "revocation").
  /// @param disabled_credentials A list of Verifiable Credentials that are disabled (revoked or suspended).
  CONSTRUCTOR create(issuer: Issuer, status_purpose: string, disabled_credentials: []VerifiableCredential)
  
  /// Checks whether the specified Verifiable Credential is disabled in the Status List.
  /// 
  /// @param credential The Verifiable Credential to check.
  /// @returns A boolean value indicating whether the credential is disabled (true if disabled, false otherwise).
  METHOD is_disabled(credential VerifiableCredential): bool
```

## VerifiablePresentation

#### `VerifiablePresentation`

```pseudocode!
/// Represents a Verifiable Presentation according to the W3C Verifiable Credentials Data Model.
/// A Verifiable Presentation allows a holder to present one or more Verifiable Credentials to a verifier.
CLASS VerifiablePresentation

  /// A list of contexts used to define the semantic meaning of the data contained in the presentation.
  PUBLIC DATA @context: []string
  
  /// The unique identifier for the Verifiable Presentation.
  PUBLIC DATA id: string
  
  /// The type(s) of the Verifiable Presentation, typically includes "VerifiablePresentation".
  PUBLIC DATA type: []string
  
  /// The holder of the Verifiable Presentation, identified by a DID or other identifier.
  PUBLIC DATA holder: string
  
  /// The date and time when the presentation was issued.
  PUBLIC DATA issuance_date: datetime
  
  /// The optional expiration date and time after which the presentation is no longer valid.
  PUBLIC DATA expiration_date: datetime?
  
  /// A list of Verifiable Credentials contained within the presentation.
  PUBLIC DATA verifiable_credential: []string
  
  /// Additional data that may be included in the presentation, represented as a key-value map.
  PUBLIC DATA additional_data: Map<string, string>?

  /// Creates a new Verifiable Presentation with the specified holder, Verifiable Credential JWTs, and optional creation options.
  /// 
  /// @param holder The entity holding and presenting the Verifiable Presentation.
  /// @param vc_jwts A list of Verifiable Credential JWTs to include in the presentation.
  /// @param options Optional parameters for creating the presentation, such as context or expiration.
  CONSTRUCTOR create(holder: string, vc_jwts: []string, options: VerifiablePresentationCreateOptions?)
  
  /// Constructs a Verifiable Presentation from a VP JWT (JSON Web Token).
  /// 
  /// @param vp_jwt The Verifiable Presentation in JWT format.
  /// @param verify If true, verifies the integrity of the JWT before creating the presentation.
  CONSTRUCTOR from_vp_jwt(vp_jwt: string, verify: bool)

  /// Signs the Verifiable Presentation using the specified Bearer DID and optional verification method.
  /// 
  /// @param bearer_did The DID used to sign the presentation.
  /// @param verification_method_id Optional identifier for the verification method.
  /// @returns A string representing the signed JWT of the Verifiable Presentation.
  METHOD sign(bearer_did: BearerDid, verification_method_id: String?): string
```

#### `VerifiablePresentationCreateOptions`

```pseudocode!
/// Represents the options available when creating a Verifiable Presentation.
/// These options allow customization of various attributes of the presentation during its creation.
CLASS VerifiablePresentationCreateOptions

  /// The unique identifier for the Verifiable Presentation. This is optional.
  PUBLIC DATA id: string?
  
  /// The context(s) for the Verifiable Presentation, which define the meaning of terms within the presentation.
  PUBLIC DATA context: []string?
  
  /// The type(s) of the Verifiable Presentation. Typically includes "VerifiablePresentation".
  PUBLIC DATA type: []string?
  
  /// The issuance date of the presentation. If not provided, defaults to the current date and time.
  PUBLIC DATA issuance_date: datetime?
  
  /// The optional expiration date for the presentation, after which it is no longer valid.
  PUBLIC DATA expiration_date: datetime?
  
  /// Additional data that may be included in the presentation, represented as a key-value map.
  PUBLIC DATA additional_data: Map<string, string>?
```

## Presentation Exchange (PEX)

### `PresentationDefinition` 

```pseudocode!
/// Represents a Presentation Definition in the context of Presentation Exchange.
/// A Presentation Definition specifies the requirements for Verifiable Credentials to be presented.
CLASS PresentationDefinition

  /// The unique identifier for the Presentation Definition.
  PUBLIC DATA id: string
  
  /// The name of the Presentation Definition. This is optional.
  PUBLIC DATA name: string?
  
  /// The purpose of the Presentation Definition, explaining its intent. This is optional.
  PUBLIC DATA purpose: string?
  
  /// A list of input descriptors defining the Verifiable Credentials required for the presentation.
  PUBLIC DATA input_descriptors: []InputDescriptor
  
  /// Selects the Verifiable Credentials from the provided list of Verifiable Credential JWTs that satisfy the input descriptors.
  /// 
  /// @param vc_jwts A list of Verifiable Credential JWTs to select from.
  /// @returns A list of Verifiable Credential JWTs that match the input descriptors.
  METHOD select_credentials(vc_jwts: []string): []string
  
  /// Creates a Verifiable Presentation from the selected Verifiable Credentials based on the input descriptors.
  /// 
  /// @param vc_jwts A list of Verifiable Credential JWTs to include in the presentation.
  /// @returns A PresentationResult containing the created Verifiable Presentation.
  METHOD create_presentation_from_credentials(vc_jwts: []string): PresentationResult
```

### `InputDescriptor` 

```pseudocode!
/// Represents an Input Descriptor, which defines the required properties of Verifiable Credentials
/// for inclusion in a presentation as part of a Presentation Definition.
CLASS InputDescriptor

  /// The unique identifier for the Input Descriptor.
  PUBLIC DATA id: string
  
  /// The name of the Input Descriptor. This is optional.
  PUBLIC DATA name: string?
  
  /// The purpose of the Input Descriptor, explaining its intent. This is optional.
  PUBLIC DATA purpose: string?
  
  /// The constraints defining the required properties and structure of the Verifiable Credentials.
  PUBLIC DATA constraints: Constraints
```

### `Constraints`

```pseudocode!
/// Represents the constraints that define the required properties of Verifiable Credentials 
/// for inclusion in a presentation as part of an Input Descriptor.
CLASS Constraints

  /// A list of fields that define the required structure and values of the Verifiable Credentials.
  PUBLIC DATA fields: []Field
```

### `Field`

```pseudocode!
/// Represents a field within the constraints of an Input Descriptor.
/// Defines the required structure and values for a specific attribute of a Verifiable Credential.
CLASS Field

  /// The unique identifier for the field. This is optional.
  PUBLIC DATA id: string?
  
  /// The name of the field. This is optional.
  PUBLIC DATA name: string?
  
  /// The JSON paths to the data in the Verifiable Credential that must satisfy the field's constraints.
  PUBLIC DATA path: []string
  
  /// The purpose of the field, explaining its intent. This is optional.
  PUBLIC DATA purpose: string?
  
  /// An optional filter to apply additional constraints on the values of the field.
  PUBLIC DATA filter: Filter?
  
  /// Indicates whether the field is optional or required.
  PUBLIC DATA optional: bool?
  
  /// Specifies whether the field should be treated as a predicate (for comparison purposes).
  PUBLIC DATA predicate: Optionality?
```

### `Optionality`

```pseudocode!
/// Represents the optionality of a field in an Input Descriptor.
/// Defines whether a field is required or preferred.
ENUM Optionality

  /// The field is required.
  Required
  
  /// The field is preferred but not mandatory.
  Preferred
```

### `Filter`

```pseudocode!
/// Represents a filter applied to a field in an Input Descriptor.
/// Defines additional constraints that the field's values must satisfy.
CLASS Filter

  /// The type of the value expected for the field.
  PUBLIC DATA type: string?
  
  /// A regular expression pattern that the field's value must match. This is optional.
  PUBLIC DATA pattern: string?
  
  /// A constant value that the field's value must equal. This is optional.
  PUBLIC DATA const_value: string?
  
  /// A nested filter to apply further constraints on the field. This is optional.
  PUBLIC DATA contains: Filter?
```

### `PresentationResult`

```pseudocode!
/// Represents the result of creating a Verifiable Presentation based on a Presentation Definition.
/// Contains the presentation submission and the matched Verifiable Credentials.
CLASS PresentationResult

  /// The submission of the presentation, containing the mapping between input descriptors and Verifiable Credentials.
  PUBLIC DATA presentation_submission: PresentationSubmission
  
  /// A list of the Verifiable Credential JWTs that matched the input descriptors.
  PUBLIC DATA matched_vc_jwts: []string
```

### `PresentationSubmission`

```pseudocode!
/// Represents the submission of a Verifiable Presentation in response to a Presentation Definition.
/// Contains the mapping between input descriptors and the Verifiable Credentials used in the presentation.
CLASS PresentationSubmission

  /// The unique identifier for the presentation submission.
  PUBLIC DATA id: string
  
  /// The identifier of the Presentation Definition that this submission satisfies.
  PUBLIC DATA definition_id: string
  
  /// A list of mappings between input descriptors and the Verifiable Credentials provided.
  PUBLIC DATA descriptor_map: []InputDescriptorMapping
```

### `InputDescriptorMapping`

```pseudocode!
/// Represents a mapping between an Input Descriptor and a Verifiable Credential in a Presentation Submission.
/// Defines how the credentials satisfy the input descriptors in the Presentation Definition.
CLASS InputDescriptorMapping

  /// The unique identifier for the Input Descriptor being satisfied.
  PUBLIC DATA id: string
  
  /// The format of the Verifiable Credential (e.g., JWT or JSON-LD).
  PUBLIC DATA format: string
  
  /// The JSON path to the credential that satisfies the Input Descriptor.
  PUBLIC DATA path: string
  
  /// An optional nested mapping for further structuring of the credential.
  PUBLIC DATA path_nested: InputDescriptorMapping?
```

### `SubmissionRequirement`

```pseudocode!
/// Represents a submission requirement in a Presentation Definition.
/// Specifies rules for how Verifiable Credentials must be selected and combined in a presentation.
CLASS SubmissionRequirement

  /// The rule defining how the credentials must be selected (e.g., "all" or "pick").
  PUBLIC DATA rule: SubmissionRequirementRule
  
  /// The identifier of the source from which credentials should be selected. This is optional.
  PUBLIC DATA from: string?
  
  /// A list of nested submission requirements, allowing for more complex selection rules. This is optional.
  PUBLIC DATA from_nested: []SubmissionRequirement?
  
  /// The name of the submission requirement. This is optional.
  PUBLIC DATA name: string?
  
  /// The purpose of the submission requirement, explaining its intent. This is optional.
  PUBLIC DATA purpose: string?
  
  /// The exact number of credentials required to satisfy this submission requirement. This is optional.
  PUBLIC DATA count: uint32?
  
  /// The minimum number of credentials required to satisfy this submission requirement. This is optional.
  PUBLIC DATA min: uint32?
  
  /// The maximum number of credentials allowed to satisfy this submission requirement. This is optional.
  PUBLIC DATA max: uint32?
```

### `SubmissionRequirementRule`

```pseudocode!
/// Defines the rule for how Verifiable Credentials must be selected in a submission requirement.
ENUM SubmissionRequirementRule

  /// All credentials that match the requirement must be selected.
  All
  
  /// A subset of the credentials that match the requirement may be selected.
  Pick
```
# Crypto

## `Jwk`

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_jwk_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

```pseudocode!
/// Represents a JSON Web Key (JWK) as defined by RFC 7517.
/// A JWK is a JSON representation of a cryptographic key used for signing, verification, encryption, or decryption.
CLASS Jwk

  /// Identifies the algorithm intended for use with the key (e.g., "EdDSA", "ES256").
  PUBLIC DATA alg: string?
  
  /// The key type (e.g., "EC" for elliptic curve, "OKP" for Edwards curve).
  PUBLIC DATA kty: string
  
  /// The curve name for Elliptic Curve (EC) and Edwards Curve (OKP) keys (e.g., "secp256k1", "Ed25519").
  PUBLIC DATA crv: string
  
  /// The X coordinate for EC keys, or the public key for OKP.
  PUBLIC DATA x: string
  
  /// The Y coordinate for EC keys. This is optional and used for certain types of keys.
  PUBLIC DATA y: string?
  
  /// The private key component for EC or OKP keys. This is optional.
  PUBLIC DATA d: string?
```

## Key Managers 

### `KeyManager`

```pseudocode!
/// Represents a Key Manager interface for managing cryptographic keys.
/// Provides methods for retrieving signers and importing private keys.
INTERFACE KeyManager

  /// Returns the signer for the given public JWK.
  /// 
  /// @param public_jwk The public JWK used to retrieve the corresponding signer.
  /// @returns A Signer instance for the provided public JWK.
  METHOD get_signer(public_jwk: Jwk): Signer

  /// Imports a private JWK and returns the corresponding public JWK.
  /// 
  /// @param private_jwk The private JWK to be imported.
  /// @returns The public JWK corresponding to the imported private JWK.
  METHOD import_private_jwk(private_jwk: Jwk): Jwk
```

### `KeyExporter`

> [!WARNING]
>
> Exporting private key material is an unsafe practice and should be constrained to development settings.

```pseudocode!
/// Represents a Key Exporter interface for exporting private key material.
/// Provides methods to safely export private JWKs. Intended for development and testing environments only.
INTERFACE KeyExporter

  /// Exports the full set of private keys as JWKs.
  /// 
  /// @returns An array of JWKs representing the private keys.
  METHOD export_private_jwks(): []Jwk
```

### `InMemoryKeyManager`

The `InMemoryKeyManager` manages private keys in working memory, and so therefore any production utilization of the instance may be exposed to memory safety vulnerabilities; the `InMemoryKeyManager` is primarily intended for development & testing environments. For cases wherein this is unacceptable, [`KeyManager`](#keymanager) & [`Signer`](#signer) are both polymorphic bases classes which can be implemented and utilized in the dependent areas.

```pseudocode!
/// Represents an in-memory key manager that stores cryptographic key material in memory.
/// This class is primarily intended for development and testing environments due to memory safety concerns.
CLASS InMemoryKeyManager IMPLEMENTS KeyManager, KeyExporter

  /// Creates a new in-memory key manager with the specified private JWKs.
  /// 
  /// @param private_jwks A list of private JWKs to be managed in memory.
  CONSTRUCTOR(private_jwks: []Jwk)

  /// Returns the signer for the given public JWK.
  /// 
  /// @param public_jwk The public JWK used to retrieve the corresponding signer.
  /// @returns A Signer instance for the provided public JWK.
  METHOD get_signer(public_jwk: Jwk): Signer

  /// Imports a private JWK and returns the corresponding public JWK.
  /// 
  /// @param private_jwk The private JWK to be imported.
  /// @returns The public JWK corresponding to the imported private JWK.
  METHOD import_private_jwk(private_jwk: Jwk): Jwk

  /// Exports the full set of private keys as JWKs.
  /// 
  /// @returns An array of JWKs representing the private keys.
  METHOD export_private_jwks(): []Jwk
```

## Digital Signature Algorithms (DSA)

### `Dsa`

```pseudocode!
/// Represents the set of Digital Signature Algorithms (DSA) supported by the SDK.
/// Used to indicate the algorithm used for signing and verification.
ENUM Dsa

  /// Ed25519 digital signature algorithm.
  Ed25519
  
  /// Secp256k1 elliptic curve digital signature algorithm.
  Secp256k1
```

> We must add support for `X25519` and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

### `Signer`

```pseudocode!
/// Represents a signer interface for digital signatures using a private key.
/// Provides methods for signing data.
INTERFACE Signer

  /// Signs the given payload using the encapsulated private key material.
  /// 
  /// @param payload The data to be signed.
  /// @returns The signature as a byte array.
  METHOD sign(payload: []byte): []byte
```

### `Verifier`

```pseudocode!
/// Represents a verifier interface for verifying digital signatures using public key material.
/// Provides methods for verifying data against a signature.
INTERFACE Verifier

  /// Verifies the given signature against the payload using the encapsulated public key material.
  /// 
  /// @param payload The original data that was signed.
  /// @param signature The signature to be verified.
  /// @returns A boolean indicating whether the verification was successful.
  METHOD verify(payload: []byte, signature: []byte): bool
```

### `Ed25519Generator`

```pseudocode!
/// Represents a generator for creating Ed25519 key pairs.
/// Provides a method to generate private key material for the Ed25519 digital signature algorithm.
CLASS Ed25519Generator

  /// Generates a new Ed25519 key pair, returning the JWK containing both the private and public key material.
  /// 
  /// @returns A JWK representing the Ed25519 private and public keys.
  STATIC METHOD generate(): Jwk
```

### `Ed25519Signer`

```pseudocode!
/// Represents a signer implementation for the Ed25519 digital signature algorithm.
/// Provides methods for signing data using an Ed25519 private key.
CLASS Ed25519Signer IMPLEMENTS Signer

  /// Creates a new Ed25519Signer with the given private JWK.
  /// 
  /// @param private_jwk The private JWK used for signing.
  CONSTRUCTOR(private_jwk: Jwk)

  /// Signs the given payload using the Ed25519 private key.
  /// 
  /// @param payload The data to be signed.
  /// @returns The signature as a byte array.
  METHOD sign(payload: []byte): []byte
```

### `Ed25519Verifier`

```pseudocode!
/// Represents a verifier implementation for the Ed25519 digital signature algorithm.
/// Provides methods for verifying data using an Ed25519 public key.
CLASS Ed25519Verifier IMPLEMENTS Verifier

  /// Creates a new Ed25519Verifier with the given public JWK.
  /// 
  /// @param public_jwk The public JWK used for verification.
  CONSTRUCTOR(public_jwk: Jwk)

  /// Verifies the given signature against the payload using the Ed25519 public key.
  /// 
  /// @param payload The original data that was signed.
  /// @param signature The signature to be verified.
  /// @returns A boolean indicating whether the verification was successful.
  METHOD verify(payload: []byte): bool
```

### `Secp256k1Generator`

```pseudocode!
/// Represents a generator for creating Secp256k1 key pairs.
/// Provides a method to generate private key material for the Secp256k1 elliptic curve digital signature algorithm.
CLASS Secp256k1Generator

  /// Generates a new Secp256k1 key pair, returning the JWK containing both the private and public key material.
  /// 
  /// @returns A JWK representing the Secp256k1 private and public keys.
  STATIC METHOD generate(): Jwk
```

### `Secp256k1Signer`

```pseudocode!
/// Represents a signer implementation for the Secp256k1 elliptic curve digital signature algorithm.
/// Provides methods for signing data using a Secp256k1 private key.
CLASS Secp256k1Signer IMPLEMENTS Signer

  /// Creates a new Secp256k1Signer with the given private JWK.
  /// 
  /// @param private_jwk The private JWK used for signing.
  CONSTRUCTOR(private_jwk: Jwk)

  /// Signs the given payload using the Secp256k1 private key.
  /// 
  /// @param payload The data to be signed.
  /// @returns The signature as a byte array.
  METHOD sign(payload: []byte): []byte
```

### `Secp256k1Verifier`

```pseudocode!
/// Represents a verifier implementation for the Secp256k1 elliptic curve digital signature algorithm.
/// Provides methods for verifying data using a Secp256k1 public key.
CLASS Secp256k1Verifier IMPLEMENTS Verifier

  /// Creates a new Secp256k1Verifier with the given public JWK.
  /// 
  /// @param public_jwk The public JWK used for verification.
  CONSTRUCTOR(public_jwk: Jwk)

  /// Verifies the given signature against the payload using the Secp256k1 public key.
  /// 
  /// @param payload The original data that was signed.
  /// @param signature The signature to be verified.
  /// @returns A boolean indicating whether the verification was successful.
  METHOD verify(payload: []byte): bool
```

### `X25519Generator`

```pseudocode!
/// Represents a generator for creating X25519 key pairs.
/// Provides a method to generate private key material for the X25519 key exchange algorithm.
CLASS X25519Generator

  /// Generates a new X25519 key pair, returning the JWK containing both the private and public key material.
  /// 
  /// @returns A JWK representing the X25519 private and public keys.
  STATIC METHOD generate(): Jwk
```

# Decentralized Identifier's (DIDs)

## `Did`

```pseudocode!
/// Represents a Decentralized Identifier (DID) according to the W3C DID Core Specification.
/// A DID is a globally unique identifier that enables verifiable, decentralized digital identity.
CLASS Did

  /// The complete Decentralized Identifier (DID) URI.
  /// Specifies the DID syntax as per the W3C DID Core specification.
  PUBLIC DATA uri: string
  
  /// The DID URI plus a network location identifier for a specific resource.
  PUBLIC DATA url: string
  
  /// The method component of the DID URI, indicating the identifier scheme (e.g., "jwk", "dht").
  PUBLIC DATA method: string
  
  /// The method-specific identifier component in the DID URI.
  PUBLIC DATA id: string
  
  /// A map containing optional parameters present in the DID URI, which are method-specific.
  PUBLIC DATA params: Map<string, string>?
  
  /// An optional path component in the DID URI.
  PUBLIC DATA path: string?
  
  /// An optional query component in the DID URI, used to express a request for a specific representation or resource.
  PUBLIC DATA query: string?
  
  /// An optional fragment component in the DID URI, used to reference a specific part of a DID document.
  PUBLIC DATA fragment: string?
  
  /// Parses a DID URI and constructs a new Did object.
  /// 
  /// @param uri The DID URI to parse.
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
/// Represents a DID Document according to the W3C DID Core Specification.
/// A DID Document contains the public keys, services, and other metadata associated with a DID.
CLASS Document

  /// The DID URI for the subject of the DID Document.
  PUBLIC DATA id: string
  
  /// An optional array of contexts defining the semantic meaning of the properties within the document.
  PUBLIC DATA @context: []string?
  
  /// An optional array of DIDs that are authorized to control the DID Document.
  PUBLIC DATA controller: []string?
  
  /// An optional array of other identifiers associated with the DID subject.
  PUBLIC DATA alsoKnownAs: []string?
  
  /// An array of verification methods (e.g., public keys) associated with the DID subject.
  PUBLIC DATA verificationMethod: []VerificationMethod
  
  /// An optional array of methods for authenticating the DID subject.
  PUBLIC DATA authentication: []string?
  
  /// An optional array of methods for asserting claims on behalf of the DID subject.
  PUBLIC DATA assertionMethod: []string?
  
  /// An optional array of methods for establishing secure communication channels with the DID subject.
  PUBLIC DATA keyAgreement: []string?
  
  /// An optional array of methods for invoking cryptographic capabilities on behalf of the DID subject.
  PUBLIC DATA capabilityInvocation: []string?
  
  /// An optional array of methods for delegating cryptographic capabilities on behalf of the DID subject.
  PUBLIC DATA capabilityDelegation: []string?
  
  /// An optional array of services associated with the DID subject.
  PUBLIC DATA service: []Service?
  
  /// Constructs a DID Document from a JSON string.
  /// 
  /// @param json The JSON string representing the DID Document.
  CONSTRUCTOR from_json_string(json: string)

  /// Converts the DID Document to a JSON string.
  /// 
  /// @returns A JSON string representation of the DID Document.
  METHOD to_json_string(): string
```

### `VerificationMethod`

```pseudocode!
/// Represents a verification method in a DID Document.
/// A verification method is typically a cryptographic public key used for authenticating or authorizing actions.
CLASS VerificationMethod

  /// The unique identifier for the verification method.
  PUBLIC DATA id: string
  
  /// The type of the verification method (e.g., "JsonWebKey2020").
  PUBLIC DATA type: string
  
  /// The controller of the verification method, typically the DID subject or another authorized entity.
  PUBLIC DATA controller: string
  
  /// The public key in JWK format associated with the verification method.
  PUBLIC DATA publicKeyJwk: Jwk
```

### `Service`

```pseudocode!
/// Represents a service in a DID Document.
/// Services define ways to interact with the DID subject, such as communication endpoints or capabilities.
CLASS Service

  /// The unique identifier for the service.
  PUBLIC DATA id: string
  
  /// The type of the service (e.g., "MessagingService").
  PUBLIC DATA type: string
  
  /// The service endpoint, which can be a URL or other communication protocol.
  PUBLIC DATA serviceEndpoint: []string
```

## Resolution

### `ResolutionResult`

```pseudocode!
/// Represents the result of resolving a Decentralized Identifier (DID).
/// Contains the resolved DID Document and associated metadata.
CLASS ResolutionResult

  /// The resolved DID Document, if available.
  PUBLIC DATA document: Document?
  
  /// The metadata associated with the resolved DID Document.
  PUBLIC DATA document_metadata: DocumentMetadata?
  
  /// The metadata associated with the resolution process.
  PUBLIC DATA resolution_metadata: ResolutionMetadata
  
  /// Resolves a DID and returns the corresponding ResolutionResult.
  /// 
  /// @param uri The DID URI to resolve.
  /// @returns The result of the DID resolution process.
  CONSTRUCTOR resolve(uri: string)
```

### `ResolutionMetadataError`

```pseudocode!
/// Represents error codes from the DID resolution process.
/// Used to indicate specific issues encountered during resolution.
ENUM ResolutionMetadataError

  /// The DID provided was invalid and resolution could not proceed.
  invalidDid
  
  /// The DID was not found during resolution.
  notFound
  
  /// The requested representation of the DID payload is not supported.
  representationNotSupported
  
  /// The DID method used is not supported by the resolver.
  methodNotSupported
  
  /// The DID Document found was invalid or not conformant.
  invalidDidDocument
  
  /// The DID Document length exceeded acceptable limits.
  invalidDidDocumentLength
  
  /// An internal error occurred during the DID resolution process.
  internalError
```

### `ResolutionMetadata`

```pseudocode!
/// Represents the metadata associated with the resolution of a Decentralized Identifier (DID).
/// Provides information about the resolution process, including errors if any occurred.
CLASS ResolutionMetadata

  /// The error code from the resolution process, if an error occurred.
  PUBLIC DATA error: ResolutionMetadataError?
```

### `DocumentMetadata`

```pseudocode!
/// Represents the metadata associated with a resolved DID Document.
/// Contains information about the lifecycle and versioning of the DID Document.
CLASS DocumentMetadata

  /// The timestamp when the DID Document was created. This is optional.
  PUBLIC DATA created: string?
  
  /// The timestamp when the DID Document was last updated. This is optional.
  PUBLIC DATA updated: string?
  
  /// Indicates whether the DID has been deactivated. This is optional.
  PUBLIC DATA deactivated: bool?
  
  /// The timestamp for when the DID Document is scheduled for the next update. This is optional.
  PUBLIC DATA nextUpdate: string?
  
  /// The identifier for the version of the DID Document. This is optional.
  PUBLIC DATA versionId: string?
  
  /// The identifier for the next version of the DID Document. This is optional.
  PUBLIC DATA nextVersionId: string?
  
  /// A list of equivalent DIDs, indicating alternative identifiers for the DID subject. This is optional.
  PUBLIC DATA equivalentId: []string?
  
  /// The canonical identifier for the DID, representing its authoritative form. This is optional.
  PUBLIC DATA canonicalId: string?
```

## Methods

### `DidJwk`

```pseudocode!
/// Represents a DID using the JWK (JSON Web Key) method.
/// Provides methods for creating and resolving DIDs based on JWKs.
CLASS DidJwk

  /// Creates a new DID based on a JWK with the specified options.
  /// 
  /// @param options Optional parameters for creating the DID.
  /// @returns A BearerDid instance representing the created DID.
  STATIC METHOD create(options: DidJwkCreateOptions?): BearerDid
  
  /// Resolves a DID JWK URI and returns the corresponding ResolutionResult.
  /// 
  /// @param uri The DID JWK URI to resolve.
  /// @returns The result of the DID resolution process.
  STATIC METHOD resolve(uri: string): ResolutionResult
```

#### `DidJwkCreateOptions`

```pseudocode!
/// Represents the options available when creating a DID using the JWK method.
/// Allows customization of key management and signature algorithms.
CLASS DidJwkCreateOptions

  /// The key manager responsible for handling key material. This is optional.
  PUBLIC DATA key_manager: KeyManager?
  
  /// The digital signature algorithm to be used (e.g., "Ed25519", "Secp256k1"). This is optional.
  PUBLIC DATA dsa: Dsa?
```

### `DidWeb`

> [!NOTE]
>
> The `create` does not publish the DID Document to a host, but creates the instance of the `did:web` `BearerDid` in the local scope. 

```pseudocode!
/// Represents a DID using the Web method.
/// Provides methods for creating and resolving DIDs based on domain names.
CLASS DidWeb

  /// Creates a new DID based on a domain name with the specified options.
  /// 
  /// @param domain The domain name to associate with the DID.
  /// @param options Optional parameters for creating the DID.
  /// @returns A BearerDid instance representing the created DID.
  STATIC METHOD create(domain: string, options: DidWebCreateOptions?): BearerDid
  
  /// Resolves a DID Web URI and returns the corresponding ResolutionResult.
  /// 
  /// @param uri The DID Web URI to resolve.
  /// @returns The result of the DID resolution process.
  STATIC METHOD resolve(uri: string): ResolutionResult
```

#### `DidWebCreateOptions`

```pseudocode!
/// Represents the options available when creating a DID using the Web method.
/// Allows customization of the DID's services, controllers, and verification methods.
CLASS DidWebCreateOptions

  /// The key manager responsible for handling key material. This is optional.
  PUBLIC DATA key_manager: KeyManager?
  
  /// The digital signature algorithm to be used (e.g., "Ed25519", "Secp256k1"). This is optional.
  PUBLIC DATA dsa: Dsa?
  
  /// An optional array of services associated with the DID.
  PUBLIC DATA service: []Service?
  
  /// An optional array of controllers authorized to manage the DID.
  PUBLIC DATA controller: []string?
  
  /// An optional array of additional DIDs associated with the subject.
  PUBLIC DATA also_known_as: []string?
  
  /// An optional array of verification methods (e.g., public keys) associated with the DID.
  PUBLIC DATA verification_method: []VerificationMethod?
```

### `DidDht`

```pseudocode!
/// Represents a DID using the DHT (Distributed Hash Table) method.
/// Provides methods for creating, publishing, and resolving DIDs on a DHT network.
CLASS DidDht

  /// Creates a new DID using the DHT method with the specified options.
  /// 
  /// @param options Optional parameters for creating the DID.
  /// @returns A BearerDid instance representing the created DID.
  STATIC METHOD create(options: DidDhtCreateOptions?): BearerDid

  /// Publishes the DID Document to the specified DHT gateway.
  /// 
  /// @param bearer_did The DID to be published.
  /// @param gateway_url Optional URL of the DHT gateway where the DID Document will be published.
  STATIC METHOD publish(bearer_did: BearerDid, gateway_url: string?)

  /// Resolves a DID DHT URI and returns the corresponding ResolutionResult.
  /// 
  /// @param uri The DID DHT URI to resolve.
  /// @param gateway_url Optional URL of the DHT gateway to use for resolving the DID.
  /// @returns The result of the DID resolution process.
  STATIC METHOD resolve(uri: string, gateway_url: string?): ResolutionResult
```

> [!NOTE]
> `resolve()` makes use of [`Ed25519Verifier`](#ed25519verifier) internally for DNS packet verification.

#### `DidDhtCreateOptions`

```pseudocode!
/// Represents the options available when creating a DID using the DHT method.
/// Allows customization of the DID's services, controllers, and verification methods.
CLASS DidDhtCreateOptions

  /// The key manager responsible for handling key material. This is optional.
  PUBLIC DATA key_manager: KeyManager?
  
  /// An optional array of services associated with the DID.
  PUBLIC DATA service: []Service?
  
  /// An optional array of controllers authorized to manage the DID.
  PUBLIC DATA controller: []string?
  
  /// An optional array of additional DIDs associated with the subject.
  PUBLIC DATA also_known_as: []string?
  
  /// An optional array of verification methods (e.g., public keys) associated with the DID.
  PUBLIC DATA verification_method: []VerificationMethod?
  
  /// A flag indicating whether the DID should be published upon creation. Defaults to true.
  PUBLIC DATA publish: bool? = true
  
  /// The URL of the DHT gateway to use for publishing or resolving the DID. This is optional.
  PUBLIC DATA gateway_url: string?
```

## `BearerDid`

```pseudocode!
/// Represents a Bearer Decentralized Identifier (DID).
/// A BearerDid contains both the DID and the associated DID Document along with the key management logic.
CLASS BearerDid

  /// The DID associated with this BearerDid instance.
  PUBLIC DATA did: Did
  
  /// The DID Document associated with this BearerDid instance.
  PUBLIC DATA document: Document
  
  /// The key manager responsible for handling cryptographic operations for this DID.
  PUBLIC DATA key_manager: KeyManager

  /// Creates a new BearerDid from a PortableDid instance.
  /// 
  /// @param portable_did The PortableDid instance to convert into a BearerDid.
  CONSTRUCTOR from_portable_did(portable_did: PortableDid)

  /// Converts the BearerDid into a PortableDid by exporting the private key material.
  /// 
  /// @param key_exporter The KeyExporter instance used to export private key material.
  /// @returns A PortableDid instance representing the BearerDid.
  METHOD to_portable_did(key_exporter: KeyExporter): PortableDid
  
  /// Retrieves a signer for the specified verification method associated with this BearerDid.
  /// 
  /// @param verification_method_id The identifier of the verification method to retrieve the signer for.
  /// @returns A Signer instance for the specified verification method.
  METHOD get_signer(verification_method_id: string): Signer
```

## `PortableDid`

The `PortableDid` is a JSON serialized representation of a `BearerDid`.

> [!WARNING]
>
> The `PortableDid` contains **serialized private key material** and it is therefore **NOT SAFE** for production environments; the `PortableDid` is primarily intended for usage in development & testing environments.

```pseudocode!
/// Represents a Portable Decentralized Identifier (DID).
/// A PortableDid is a JSON-serialized representation of a BearerDid, including private key material.
/// This format is primarily intended for development and testing environments.
CLASS PortableDid

  /// The URI representing the PortableDid.
  PUBLIC DATA uri: string
  
  /// An array of private JWKs (JSON Web Keys) associated with the PortableDid.
  PUBLIC DATA private_jwks: []Jwk
  
  /// The DID Document associated with the PortableDid.
  PUBLIC DATA document: Document
  
  /// Creates a new PortableDid from a JSON string.
  /// 
  /// @param json The JSON string representing the PortableDid.
  CONSTRUCTOR from_json_string(json: string)

  /// Converts the PortableDid to a JSON string.
  /// 
  /// @returns A JSON string representation of the PortableDid.
  METHOD to_json_string(): string
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
