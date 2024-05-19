# `web5-rs` Crates <!-- omit in toc -->

The objective of these documents is to declare the Web5 SDK API surface. The documentation of the API must be agnostic to any one programming language. These markdown documents MUST be actively maintained such that the design docs reflect the current.

TODO:
- enforce in CI pipeline?
- how does WIPs fit into this?

---

## Requirements

- no circular dependencies
- no peer dependencies


- lives close to the code
  - actively maintained
  - change to the underlying code must be reflected in design
  - TODO would be nice if we could enforce this programatically 


- single view dependency diagram


## Design Concepts

- *Data Structure*: serializable ...
- *Instance Methods*: functions operated on a given instance of a *Data Structure.*
- *Associated Functions*: functions which are associated to a given *Data Structure* but which are callable without a given instance.
- *Functions*: functions 