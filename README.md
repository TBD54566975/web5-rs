# Web5 SDK Mono Repo

This monorepo houses the core components of the Web5 platform containing the core Rust code with Kotlin bindings. It features libraries for building applications with decentralized identifiers (DIDs), verifiable credentials (VCs), presentation exchange (PEX), and much more.

## 🎉 Hacktoberfest 2024 🎉

`web5-rs` is a participating project in Hacktoberfest 2024! We’re so excited for your contributions, and have created a wide variety of issues so that anyone can contribute. Whether you're a seasoned developer or a first-time open source contributor, there's something for everyone.

### To get started:
1. Read the [contributing guide](https://github.com/taniashiba/web5-rs/blob/main/CONTRIBUTING.md).
2. Read the [code of conduct](https://github.com/taniashiba/web5-rs/blob/main/CODE_OF_CONDUCT.md).
3. Choose a task from this project's Hacktoberfest issues in our [Project Hub](https://github.com/TBD54566975/web5-rs/issues/322) and follow the instructions. Each issue has the 🏷️ `hacktoberfest` label.

Have questions? Connecting with us in our [Discord community](https://discord.gg/tbd) in the `#hacktoberfest` project channel.

---

## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
   - [Cloning](#cloning)
- [Development Prerequisites](#development-prerequisites)
   - [Hermit](#hermit)
- [Building and Testing](#building-and-testing)
- [Binding Process](#binding-process)
- [API Documentation](#api-documentation)
- [Basic Usage](#basic-usage)
   - [DidJwk Creation](#didjwk-creation)
   - [Verifiable Credential Creation & Signing](#verifiable-credential-creation--signing)
   - [Verifiable Presentation Creation & Signing](#verifiable-presentation-creation--signing)
   - [Presentation Exchange](#presentation-exchange)
- [Rust Examples](#rust-examples)
   - [Instantiate a new did:jwk](#instantiate-a-new-didjwk)
   - [Simple Verifiable Credential Creation & Signing](#simple-verifiable-credential-creation--signing)
- [Kotlin Examples](#kotlin-examples)
   - [Instantiate a new did:jwk](#instantiate-a-new-didjwk-1)
   - [Simple Verifiable Credential Creation & Signing](#simple-verifiable-credential-creation--signing-1)


## Features
- DID creation and management (support for multiple DID methods).
- Verifiable Credential creation, signing, and verification.
- Status List Credentials for revocation and suspension.
- Verifiable Presentation creation and signing.
- Presentation Exchange support to handle credential selection based on definitions and generating submissions.
- Cross-platform support with multi-language bindings

## Getting Started

To start developing applications and services with the Web5 RS SDK, the following steps will guide
you through setting up your local development environment.

For detailed documentation on usage refer to the
[API reference documentation](docs/API_DESIGN.md). Additionally, comprehensive
guides can be found at the [TBD Developer site](https://developer.tbd.website/docs/) to enhance
your understanding of the underlying concepts and how to implement them effectively.

### Cloning

This repository uses git submodules. To clone this repo with submodules:
```sh
git clone --recurse-submodules git@github.com:TBD54566975/web5-rs.git
```

Or to add submodules after cloning:
```sh
git submodule update --init
```

## Development Prerequisites

### Hermit

This project uses hermit to manage tooling like the Rust compiler, Java Development Kit and Maven project management system.
See [this page](https://cashapp.github.io/hermit/usage/get-started/) to set up Hermit on your machine - make sure to
download the open source build and activate it for the project.

Once you've installed Hermit and before running builds on this repo,
run from the root:

```shell
source ./bin/activate-hermit
```

This will set your environment up correctly in the
terminal emulator you're on. Executing `just` commands should "just work", no
matter the underlying tooling used (ie. `rustc`, `cargo`, `mvn`, `java`, etc).

## Building and Testing

To run, find a build target from the table below and use `just`:

```shell
$> just [buildTarget]
```

| Command       | Description |
| ------------- | ----------- |
| `setup`       | Initalizes the environment, including `git` submodules, `rustup`, etc.  |
| `build`       | Builds the Rust core |
| `test`        | Tests the Rust core |
| `lint`        | Performs code formatting on the Rust core |
| `bind`        | Builds all language bindings |
| `bind-kotlin` | Builds the Kotlin language bindings |
| `test-bound` | Tests all language bindings |
| `test-kotlin` | Tests the Kotlin language bindings |

For instance:

```shell
$> just build
```

## Binding Process

The binding process follows these key steps:

1. **Core Rust Development**
   All the core logic for working with DIDs, verifiable credentials, and cryptographic signing and verification is implemented in Rust. Rust is chosen as the core layer for its memory safety, performance, and cross-platform capabilities.

2. **Building the Kotlin Bindings**  
   The Kotlin bindings are generated from the core Rust code and live in the `bound/kt` directory. These bindings allow Kotlin applications to access the functionality of the core Rust libraries through idiomatic Kotlin APIs.

3. **Packaging & Distribution**  
   The Kotlin bindings are packaged and distributed as a Kotlin library, which can be imported and used in Kotlin applications just like any other dependency.

## API Documentation
For the full detailed API design and usage examples, refer to the [API Design Document](docs/API_DESIGN.md)

## Basic Usage

The SDK allows developers to work with decentralized identifiers (DIDs), verifiable credentials, and presentation exchanges. Below are the key use cases:

1. **DidJwk Creation**  
   You can create DIDs using the `Did::create` method.

2. **Verifiable Credential Creation & Signing**  
   Create a verifiable credential using `VerifiableCredential::create` and sign it with a DID.

3. **Verifiable Presentation Creation & Signing**  
   Use the SDK to create and sign verifiable presentations with `VerifiablePresentation::create` and `sign`.

4. **Presentation Exchange**  
   Select the appropriate credentials and generate presentations based on the presentation definitions using `PresentationDefinition::select_credentials` and `create_presentation_from_credentials`.

## Rust Examples
### Instantiate a new `did:jwk`

```rust
let did_jwk = DidJwk::create(None);
println!("Created DID JWK: {}", did_jwk.did.uri);
```

### Simple Verifiable Credential Creation & Signing

```rust
let vc = VerifiableCredential::create(issuer, credential_subject, None)?;
let vc_jwt = vc.sign(bearer_did, None)?;
```

## Kotlin Examples
### Instantiate a new `did:jwk`

```kotlin
val didJwk = DidJwk.create()
println("Created DID JWK: ${didJwk.did.uri}")
```

### Simple Verifiable Credential Creation & Signing

```kotlin
val vc = VerifiableCredential.create(issuer, credentialSubject)
val vcJwt = vc.sign(bearerDid)
```
