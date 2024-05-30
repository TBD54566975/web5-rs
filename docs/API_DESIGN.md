> [!WARNING]
> 🚧 🚧 🚧 Under Construction 🚧 🚧 🚧
>
> 🚧 Documentation comments
> 
> 🚧 Examples
> 
> 🚧 Test vectors

# Standard Web5 API Design (APID) Document <!-- omit in toc -->

**Last Updated:** May 30, 2024

**Version:** 0.0.1

- [Language-Independent Programmatic Concepts](#language-independent-programmatic-concepts)
  - [High-Level Programming](#high-level-programming)
  - [Primitive Types](#primitive-types)
- [Key Material](#key-material)
  - [`Jwk`](#jwk)
  - [`InMemoryKeyManager`](#inmemorykeymanager)
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
  - [`ResolutionMetadataError`](#resolutionmetadataerror)
  - [`ResolutionMetadata`](#resolutionmetadata)
  - [`DocumentMetadata`](#documentmetadata)
  - [`Resolution`](#resolution)
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

# Language-Independent Programmatic Concepts

The design definitions within this design document are intended to span any programming language, so long as the given programming language supports the [High-Level Programming](#high-level-programming) and [Primitive Types](#primitive-types) concepts below. The instantiations of these concepts will be unique to the given idioms for each target programming language.

## High-Level Programming

- **Polymorphic Base Class:** A base class designed to be extended by other classes, allowing them to override methods to provide specific implementations. This enables different classes to be treated through a common interface, facilitating polymorphism.
- **Object Oriented Class:** A blueprint for creating objects (instances), encapsulating data for the object and methods to manipulate that data. It represents the core structure in object-oriented programming, promoting code reuse and modularity.
- **Public Data Members:** Public attributes or fields within a class that store the state or data of an object. These properties are directly accessible from outside the class, allowing other classes and functions to read and modify their values.
- **Constructor:** A special method in a class that is called when an object is instantiated. The constructor initializes the object's properties and performs any setup required. Constructors can be overloaded to provide multiple ways to create an object.
- **Instance Method:** A function defined in a class that operates on instances of the class. Instance methods can access and modify the object's properties and often define the behavior of the objects created from the class.
- **Static Method:** A method defined in a class that does not operate on instances of the class but on the class itself. Static methods can be called without creating an instance of the class and typically provide utility functions or perform actions related to the class as a whole.
- **Enumeration:** A distinct data type consisting of a set of named values called elements or members. Enumerations are used to define variables that can only take one out of a small set of possible values, improving code clarity and reducing errors by limiting the range of acceptable values.

## Primitive Types

| Type       | Representation                          | Description |
| ---------- | --------------------------------------- | ----------- |
| string     | `String`                                | 🚧           |
| byte       | `Byte`                                  | 🚧           |
| boolean    | `Bool`                                  | 🚧           |
| array      | `[]T`                                   | 🚧           |
| optional   | `T?`                                    | 🚧           |
| hash map   | `Map<T1, T2>`                           | 🚧           |
| function   | `func_name(param1: T1, param2: T2): T3` | 🚧           |
| mixed type | `T1 \| T2`                              | 🚧           |

# Key Material

> [!NOTE]
> We strictly represent public & private *key material* as [Jwk](#jwk-object-oriented-class), but we may consider disintermediating at some point by introducing polymorphic base classes for `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()` instance method for JWK representations.

## `Jwk`

| Member        | Description |
| ------------- | ----------- |
| `alg: String` |             |
| `kty: String` |             |
| `crv: String` |             |
| `x: String`   |             |
| `y: String?`  |             |
| `d: String?`  |             |

## `InMemoryKeyManager`

🚧 Strictly Ed25519 🚧

| Instance Method                              | Notes |
| :------------------------------------------- | :---- |
| `generate_key_material(): Jwk`               |       |
| `get_signer(public_key: Jwk): Ed25519Signer` |       |
| `import_key(private_key: Jwk)`               |       |

# Digital Signature Algorithm's (DSA)

## `Dsa`

The set of Digital Signature Algorithm's natively supported within this SDK.

| Enumeration |
| :---------- |
| `Ed22519`   |

> [!NOTE]
> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

## `DsaSigner`

`DsaSigner` is a Polymorphic Base Class.

Private key material is assumed to be encapsulated.

| Instance Method                     | Notes |
| :---------------------------------- | :---- |
| `dsa_sign(payload: []Byte): []Byte` |       |

## `DsaVerifier`

`DsaVerifier` is a Polymorphic Base Class.

Public key material is assumed to be encapsulated.

| Instance Method                                          | Notes |
| :------------------------------------------------------- | :---- |
| `dsa_verify(message: []Byte, signature: []Byte): []Byte` |       |

## `JwsSigner`

`JwsSigner` is a Polymorphic Base Class.

Private key material is assumed to be encapsulated.

| Instance Method                     | Notes |
| :---------------------------------- | :---- |
| `jws_sign(payload: []Byte): []Byte` |       |

## `JwsVerifier`

`JwsVerifier` is a Polymorphic Base Class.

Public key material is assumed to be encapsulated.

| Instance Method                                          | Notes |
| :------------------------------------------------------- | :---- |
| `jws_verify(message: []Byte, signature: []Byte): []Byte` |       |

## `Ed25519Generator`

| Static Method                          | Notes |
| :------------------------------------- | :---- |
| `generate_private_key_material(): Jwk` |       |

## `Ed25519Signer`

Implements [`DsaSigner`](#dsasigner) and [`JwsSigner`](#jwssigner) for Ed25519.

| Constructor                     | Notes |
| :------------------------------ | :---- |
| `constructor(private_key: Jwk)` |       |

## `Ed25519Verifier`

Implements [`DsaVerifier`](#dsaverifier) and [`JwsVerifier`](#jwsverifier) for Ed25519.

| Constructor                    | Notes |
| :----------------------------- | :---- |
| `constructor(public_key: Jwk)` |       |

# Decentralized Identifier's (DIDs)

## `Did`

| Property                       | Notes |
| :----------------------------- | :---- |
| `uri: String`                  |       |
| `url: String`                  |       |
| `method: String`               |       |
| `id: String`                   |       |
| `params: Map<String, String>?` |       |
| `path: String?`                |       |
| `query: String?`               |       |
| `fragment: String?`            |       |

| Static Method             | Notes |
| :------------------------ | :---- |
| `parse(uri: String): Did` |       |

## `Document`

| Property                                   | Notes |
| ------------------------------------------ | ----- |
| `id: String`                               |       |
| `@context: []String?`                      |       |
| `controller: []String?`                    |       |
| `alsoKnownAs: []String?`                   |       |
| `verificationMethod: []VerificationMethod` |       |
| `authentication: []String?`                |       |
| `assertionMethod: []String?`               |       |
| `keyAgreement: []String?`                  |       |
| `capabilityInvocation: []String?`          |       |
| `capabilityDelegation: []String?`          |       |
| `service: []Service?`                      |       |

## `VerificationMethod`

| Property             | Notes |
| -------------------- | ----- |
| `id: String`         |       |
| `type: String`       |       |
| `controller: String` |       |
| `publicKeyJwk: Jwk`  |       |

## `Service`

| Property                    | Notes |
| --------------------------- | ----- |
| `id: String`                |       |
| `type: String`              |       |
| `serviceEndpoint: []String` |       |

## `ResolutionMetadataError`

| Enumeration                  |
| ---------------------------- |
| `invalidDid`                 |
| `notFound`                   |
| `representationNotSupported` |
| `methodNotSupported`         |
| `invalidDidDocument`         |
| `invalidDidDocumentLength`   |
| `internalError`              |

## `ResolutionMetadata`

| Property                          | Notes |
| --------------------------------- | ----- |
| `error: ResolutionMetadataError?` |       |

## `DocumentMetadata`

| Property                  | Notes |
| ------------------------- | ----- |
| `created: String?`        |       |
| `updated: String?`        |       |
| `deactivated: Bool?`      |       |
| `nextUpdate: String?`     |       |
| `versionId: String?`      |       |
| `nextVersionId: String?`  |       |
| `equivalentId: []String?` |       |
| `canonicalId: String?`    |       |

## `Resolution`

| Property                                  | Notes |
| :---------------------------------------- | :---- |
| `document: Document`                      |       |
| `document_metadata: DocumentMetadata`     |       |
| `resolution_metadata: ResolutionMetadata` |       |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: String): Resolution` |       |

## Methods

### `DidJwk`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `create(public_key: Jwk): DidJwk`  |       |
| `resolve(uri: String): Resolution` |       |

### `DidWeb`

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `resolve(uri: String): Resolution` |       |

### `DidDht`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                                                                        | Notes      |
| ------------------------------------------------------------------------------------ | ---------- |
| `create(signer: DsaSigner, identity_key: Jwk, options: DidDhtCreateOptions): DidDht` |            |
| `resolve(uri: String): Resolution`                                                   |            |
| `update(...todo)`                                                                    | 🚧 params 🚧 |
| `deactivate(...todo)`                                                                | 🚧 params 🚧 |

#### `DidDhtCreateOptions`

| Property                                      | Notes                 |
| --------------------------------------------- | --------------------- |
| `publish: Bool`                               |                       |
| `also_known_as: []String?`                    |                       |
| `controller: []String?`                       |                       |
| `service: []Service?`                         |                       |
| `registered_type: []RegisteredDidType?`       | 🚧 `RegisteredDidType` |
| `verification_methods: []VerificationMethod?` |                       |

# Verifiable Credential's (VCs)

## `NamedIssuer`

| Property       | Notes |
| -------------- | ----- |
| `id: String`   |       |
| `name: String` |       |

## `VerifiableCredential`

| Property                        | Notes                      |
| ------------------------------- | -------------------------- |
| `@context: []String`            |                            |
| `id: String`                    |                            |
| `type: []String`                |                            |
| `issuer: String \| NamedIssuer` |                            |
| `issuanceDate: String`          |                            |
| `expirationDate: String?`       |                            |
| `credentialSubject: Object`     | 🚧 `Object` not supported 🚧 |

| Instance Method                       | Notes |
| :------------------------------------ | :---- |
| `sign(jws_signer: JwsSigner): String` |       |

| Static Method                                                                          | Notes                                                                                    |
| :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `verify(vcjwt: String): VerifiableCredential`                                          | Where the natively supported [`Dsa`](#dsa)'s are applied for cryptographic verification. |
| `verify_with_verifier(vcjwt: String, jws_verifier: JwsVerifier): VerifiableCredential` |                                                                                          |

> [!NOTE]
>
> `verify` and `verify_with_verifier` assume `vcjwt` is a compact serialized JWS wherein the `kid` JOSE Header is equal to a DID URI which can be dereferenced to fetch the [`publicKeyJwk`](./did.md#data-models).

# Presentation Exchange (PEX)

## `PresentationDefinition` 

| Property                               | Notes |
| -------------------------------------- | ----- |
| `id: String`                           |       |
| `name: String?`                        |       |
| `purpose: String?`                     |       |
| `input_descriptors: []InputDescriptor` |       |

| Instance Method                                   | Notes |
| ------------------------------------------------- | ----- |
| `select_credentials(vc_jwts: []String): []String` |       |

## `InputDescriptor` 

| Property                   | Notes |
| -------------------------- | ----- |
| `id: String`               |       |
| `name: String?`            |       |
| `purpose: String?`         |       |
| `constraints: Constraints` |       |

## `Constraints`

| Property          | Notes |
| ----------------- | ----- |
| `fields: []Field` |       |

## `Field`

| Property                  | Notes |
| ------------------------- | ----- |
| `id: String?`             |       |
| `name: String?`           |       |
| `path: []String`          |       |
| `purpose: String?`        |       |
| `filter: Filter?`         |       |
| `optional: Bool?`         |       |
| `predicate: Optionality?` |       |

## `Optionality`

| Enumeration |
| ----------- |
| `Required`  |
| `Preferred` |

## `Filter`

| Property               | Notes |
| ---------------------- | ----- |
| `type: String?`        |       |
| `pattern: String?`     |       |
| `const_value: String?` |       |
| `contains: Filter?`    |       |
