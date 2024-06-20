# Custom DSL <!-- omit in toc -->

**Last Updated:** May 30, 2024

**Version:** 0.1.0

The design definitions within the Custom DSL are intended to span any programming language, so long as the given programming language supports the [High-Level Concepts](#high-level-concepts) and [Primitive Concepts](#primitive-concepts) in one form or another. The instantiations of these concepts will be unique to the given idioms of the target programming language.

- [Limitations](#limitations)
- [Primitive Concepts](#primitive-concepts)
- [High-Level Concepts](#high-level-concepts)
  - [Polymorphic Base Class](#polymorphic-base-class)
  - [Class](#class)
  - [Enumeration](#enumeration)

# Limitations

In order to achieve the goal of defining concrete design definitions which span multiple languages, we must make some sacrifices in our design. Namely, this design excludes ***generics*** and ***variadic function parameters***, because both lack broad support & consistency across target programming languages. Implementations may choose to utilize these concepts in their internals, but the publicly accessible API must exclude these concepts.

The Custom DSL does not assert requirements as to the artifact makeup (i.e. npm packages, rust crates, go modules, etc.) of the API. It is recommended to implement the entirety of an API design in a single artifact, but each implementation may choose to create multiple artifacts. However, the APID makes no regards for the matter of circular dependencies, and so it may become unviable to implement the APID in it's completeness across multiple artifacts.

The Custom DSL does not offer a primitive type conceptually equivalent to a JSON Object wherein the concept is a hash map with N-number of distinct value types, where N > 1. In other words, the Custom DSL offers a hash map primitive 
concept, but the key and value types are considered to uniform over the entirety of the hash map. 

> [!WARNING]
> Concepts required but missing: 
> - Errors.
> - JSON Object (see above paragraph).
> - JSON serialization naming.
> - Namespacing.

# Primitive Concepts

| Type              | Representation                          |
| :---------------- | :-------------------------------------- |
| string            | `string`                                |
| byte              | `byte`                                  |
| boolean           | `bool`                                  |
| integer           | `int`                                   |
| array             | `[]T`                                   |
| optional/nullable | `T?`                                    |
| hash map          | `Map<T1, T2>`                           |
| function          | `func_name(param1: T1, param2: T2): T3` |
| mixed type        | `T1 \| T2`                              |

# High-Level Concepts

## Polymorphic Base Class

- `INTERFACE InterfaceName`: Defines a a polymorphic base class.
- `METHOD methodName(param: T1): T2`: Defines an instance method that any class implementing the interface must implement.

**Example**

```psuedocode
INTERFACE Shape
  METHOD area(): int
  METHOD perimeter(): int
```

> [!NOTE]
> Polymorphic base class definitions may have a `CONSTRUCTOR` to indicate assumptions of encapsulation for implementations; given a target language does not support constructor's on the polymorphic base class, then the feature can be disregarded but must be implemented in the implementation of the polymorphic base class.

## Class

- `CLASS ClassName`: Defines a class.
- `IMPLEMENTS InterfaceName`: Defines a class implementation of a polymorphic base class.
- `PUBLIC DATA memberName: T`: Type: Defines a public data member.
- `CONSTRUCTOR(param: T1)`: Defines a constructor for a class.
- `METHOD methodName(param: T1): T2`: Defines an instance method on the class.
- `STATIC METHOD methodName(param: T1): T2`: Defines an instance method on the class.

**Example**

```psuedocode
CLASS Circle IMPLEMENTS Shape
  PUBLIC DATA radius: int
  CONSTRUCTOR(radius: int)
  METHOD area(): int
  METHOD perimeter(): int
  STATIC METHOD unit_circle(): Circle
```

> [!NOTE]
> `STATIC METHOD`'s may be implemented on a `CLASS` given the implementation language supports the feature, but else can be a function (not associated with a `CLASS`), and in which case the function name should be prefixed with the `CLASS` name defined here.

## Enumeration

- `ENUM EnumName`: Defines an enumeration.

**Example:**

```psuedocode
ENUM Color
  RED
  GREEN
  BLUE
```
