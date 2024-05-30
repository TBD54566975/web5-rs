> [!WARNING]
> 🚧 Under Construction 🚧

# Standard Web5 API Design (APID) Document <!-- omit in toc -->

**Last Updated:** May 30, 2024

**Version:** 0.0.1

- [Language-Independent Programmatic Concepts](#language-independent-programmatic-concepts)
  - [High-Level Programming](#high-level-programming)
  - [Primitive Types](#primitive-types)
- [Key Material](#key-material)
  - [`Jwk`](#jwk)
    - [Public Data Members](#public-data-members)
      - [\`alg](#alg)
  - [`InMemoryKeyManager`](#inmemorykeymanager)
- [Digital Signature Algorithm's (DSA)](#digital-signature-algorithms-dsa)
- [Decentralized Identifier's (DIDs)](#decentralized-identifiers-dids)
- [Verifiable Credential's (VCs)](#verifiable-credentials-vcs)
- [Presentation Exchange (PEX)](#presentation-exchange-pex)


# Language-Independent Programmatic Concepts

The design definitions within this design document are intended to span any programming language, so long as the given programming language supports object oriented & polymorphic programming. The instantiations of these concepts will be unique to the given idioms for each target programming language.

## High-Level Programming

- **Polymorphic Base Class:** A base class designed to be extended by other classes, allowing them to override methods to provide specific implementations. This enables different classes to be treated through a common interface, facilitating polymorphism.
- **Object Oriented Class:** A blueprint for creating objects (instances), encapsulating data for the object and methods to manipulate that data. It represents the core structure in object-oriented programming, promoting code reuse and modularity.
- **Public Data Members:** Public attributes or fields within a class that store the state or data of an object. These properties are directly accessible from outside the class, allowing other classes and functions to read and modify their values.
- **Constructor:** A special method in a class that is called when an object is instantiated. The constructor initializes the object's properties and performs any setup required. Constructors can be overloaded to provide multiple ways to create an object.
- **Instance Method:** A function defined in a class that operates on instances of the class. Instance methods can access and modify the object's properties and often define the behavior of the objects created from the class.
- **Static Method:** A method defined in a class that does not operate on instances of the class but on the class itself. Static methods can be called without creating an instance of the class and typically provide utility functions or perform actions related to the class as a whole.
- **Enumeration:** A distinct data type consisting of a set of named values called elements or members. Enumerations are used to define variables that can only take one out of a small set of possible values, improving code clarity and reducing errors by limiting the range of acceptable values.

## Primitive Types

- string
- array
- byte
- boolean
- optional/nullable

# Key Material

## `Jwk`

The `Jwk` is an Object Oriented Class representing a [JSON Web Key](https://datatracker.ietf.org/doc/html/rfc7517).

### Public Data Members

#### `alg

| Property            | Notes |
| :------------------ | :---- |
| `alg: String`       |       |
| `kty: String`       |       |
| `crv: String`       |       |
| `x: String`         |       |
| `y: Option<String>` |       |
| `d: Option<String>` |       |

## `InMemoryKeyManager` 

The `InMemoryKeyManager` is Object Oriented Class which encapsulates Ed25519 key material.

| Instance Method                                 | Notes                                                                           |
| :---------------------------------------------- | :------------------------------------------------------------------------------ |
| `generate_key_material(): Jwk`                  | Return [`Jwk`](#jwk) is a public key and does not contain private key material. |
| `get_signer(public_key: &Jwk) -> Ed25519Signer` | See [`Ed25519Signer`](#ed25519signer).                                          |
| `import_key(private_key: &Jwk)`                 |                                                                                 |



# Digital Signature Algorithm's (DSA)

...

# Decentralized Identifier's (DIDs)

...

# Verifiable Credential's (VCs)

...

# Presentation Exchange (PEX)

...