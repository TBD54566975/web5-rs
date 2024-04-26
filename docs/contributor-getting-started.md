# Contributor Getting Started <!-- omit in toc -->

- [The "Onion"](#the-onion)
  - [1. `core`](#1-core)
  - [2. `bindings`](#2-bindings)
  - [3. `binded`](#3-binded)
  - [4. idiomatic](#4-idiomatic)
- [Constriction of Cross-Language Binding](#constriction-of-cross-language-binding)

## The "Onion"

![Onion](./onion.png)

- `core` is unbridled rust code, without compromise for outer layers
- `bindings` is rust code to make bindings possible
- `binded` is non-rust code without concern for idioms
- idiomatic is the expressive non-rust project, well-rounded, intended for developer consumption

### 1. `core`

`core` is where the uncompromising web5 rust code exists, without regard for binding. No circular dependencies shall exist. See each crate for more information.

![Dependency Diagram](./dependency-diagram.png)

### 2. `bindings`

`bindings` is where rust code exist for the purposes of binding to non-rust languages. This project makes use of [UniFFI](https://github.com/mozilla/uniffi-rs) for building and generating the cross-language bindings.

In the case wherein an unknown type must cross the binding boundary (for example, in JavaScript that would be an `Object` or in Golang that would be an `interface {}`), we make available a custom type called `Unknown`. `Unknown` must be JSON-serializable safe.

> ⚠️ **WARNING**: `Uknown` is not currently implemented, but stubbed in here for us to come back to

### 3. `binded`

`binded` is where the non-rust project exists without concern for idiomatic patterns in the given target language. Each binded project must have comprehensive unit test coverage. Current languages supported:

- Kotlin
- Swift

### 4. idiomatic

Idiomatic is the layer in which the non-rust project is built for developer consumption.

> ⚠️ **WARNING**: Unclear exactly how this will manifest, TBD

## Constriction of Cross-Language Binding

![Binding Problem](./binding-problem.png)