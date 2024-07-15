# Work in progress

This repo is not ready for consumption, and is under heavy development. Right now the Rust core bindings work only for Apple Silicon systems.

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
$> just test-kotlin
...
[INFO] Tests run: 10, Failures: 0, Errors: 0, Skipped: 0
[INFO]
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  10.035 s
[INFO] Finished at: 2024-07-15T00:38:03-04:00
[INFO] ------------------------------------------------------------------------
```

## Tooling Details

While we execute most commands with `just`, some developers may like finer-grained control over the underlying build systems. This is a guide to those tools.

### Rust

This project is written in [Rust](https://www.rust-lang.org/), a modern, performant, statically-linked programming language. It's installed and configured on you `$PATH` via Hermit, above.

You may verify your `rust` installation via the terminal:

```shell
$> which rustup
~/web5-rs/bin/rustup // For instance

$> rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)
```

You may need to initialize your `rustc` environment if you see:

```shell
error: rustup could not choose a version of rustc to run, because one wasn't specified explicitly, and no default is configured.
help: run 'rustup default stable' to download the latest stable release of Rust and set it as your default toolchain.
```

Fix by executing:

```shell
$> rustup default stable
info: ... // Downloading things
info: default toolchain set to 'stable-aarch64-apple-darwin'

  stable-aarch64-apple-darwin installed - rustc 1.79.0 (129f3b996 2024-06-10)
```

### Maven

The Java bindings are built with the
[Maven Project Management](https://maven.apache.org/) tool.
It is installed via Hermit above.

If you want to build an artifact on your local filesystem, you can do so by running the
following command, specifying the `-f` flag to point to the `pom.xml` for the Kotlin project.

```shell
mvn -f bound/kt/pom.xml clean verify
```

This will first clean all previous builds and compiled code, then:
compile, test, and build the artifacts in each of the submodules
of this project in the `$moduleName/target` directory, for example:

```shell
ls -l bound/kt/target/
```

You should see similar to:

```shell
total 57416
drwxr-xr-x@  8 alr  staff       256 Jul 15 00:38 classes
drwxr-xr-x@  3 alr  staff        96 Jul 15 00:37 generated-sources
drwxr-xr-x@  3 alr  staff        96 Jul 15 00:38 generated-test-sources
drwxr-xr-x@  3 alr  staff        96 Jul 15 00:42 maven-archiver
drwxr-xr-x@  3 alr  staff        96 Jul 15 00:37 maven-status
drwxr-xr-x@ 14 alr  staff       448 Jul 15 00:42 surefire-reports
drwxr-xr-x@  4 alr  staff       128 Jul 15 00:38 test-classes
-rw-r--r--@  1 alr  staff  29123862 Jul 15 00:42 web5-core-kt-1.0-SNAPSHOT.jar
```

If you'd like to skip packaging and test only, run:

```shell
mvn -f bound/kt/pom.xml test
```

You may also run a single test; use the `-Dtest=` parameter to denote which test to run, for example:

```shell
mvn -f bound/kt/pom.xml test -Dtest=TestClassName
```

To install builds into your local Maven repository, run from the root:

```shell
mvn -f bound/kt/pom.xml install
```

For more, see the documentation on [Maven Lifecycle](https://maven.apache.org/guides/introduction/introduction-to-the-lifecycle.html).
