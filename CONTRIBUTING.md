# Contribution Guide 

There are many ways to be an open source contributor, and we're here to help you on your way! You may:

* Propose ideas in our 
  [discord](https://discord.gg/tbd)
* Raise an issue or feature request in our [issue tracker](LINK_HERE)  ___***FIX LINK AND REMOVE THIS NOTICE***___
* Help another contributor with one of their questions, or a code review
* Suggest improvements to our Getting Started documentation by supplying a Pull Request
* Evangelize our work together in conferences, podcasts, and social media spaces.

This guide is for you.

## 🎉 Hacktoberfest 2024 🎉

`web5-rs` is a participating in Hacktoberfest 2024! We’re so excited for your contributions, and have created a wide variety of issues so that anyone can contribute. Whether you're a seasoned developer or a first-time open source contributor, there's something for everyone.

### Here's how you can get started:
1. Read the [code of conduct](https://github.com/taniashiba/web5-rs/blob/main/CODE_OF_CONDUCT.md).
2. Choose a task from this project's Hacktoberfest issues in our [Project Hub](https://github.com/TBD54566975/web5-rs/issues/322). Each issue has the 🏷️ `hacktoberfest` label.
5. Comment ".take" on the corresponding issue to get assigned the task.
6. Fork the repository and create a new branch for your work.
7. Make your changes and submit a pull request.
8. Wait for review and address any feedback.

### 🏆 Leaderboard & Prizes
Be among the top 10 with the most points to snag custom swag just for you from our TBD shop! To earn your place in the leaderboard, we have created a points system that is explained below. As you complete tasks, you will automatically be granted a certain # of points.

#### Point System
| Weight | Points Awarded | Description |
|---------|-------------|-------------|
| 🐭 **Small** | 5 points | For smaller tasks that take limited time to complete and/or don't require any product knowledge. |
| 🐰 **Medium** | 10 points | For average tasks that take additional time to complete and/or require some product knowledge. |
| 🐂 **Large** | 15 points | For heavy tasks that takes lots of time to complete and/or possibly require deep product knowledge. |

#### Prizes
Top 10 contributors with the most points will be awarded TBD x Hacktoberfest 2024 swag. The Top 3 contributors will have special swag customized with your GitHub handle in a very limited design. (more info in our Discord)



### 👩‍ Need help?
Need help or have questions? Feel free to reach out by connecting with us in our [Discord community](https://discord.gg/tbd) to get direct help from our team in the `#hacktoberfest` project channel.

Happy contributing!

---


## Development Prerequisites

___***UPDATE TABLE OF PROJECT DEPS AND INSTALLATION NOTES***___

| Requirement | Tested Version | Installation Instructions                            |
|-------------|----------------|------------------------------------------------------|
| Go          | 1.17.6         |[go.dev](https://go.dev/doc/tutorial/compile-install) |
| Mage        | 1.12.1         |[magefile.org](https://magefile.org/)                 |
| Java        | 17.0.2         | Below, recommended via [SDKMan](https://sdkman.io)   |

### Go

This project is written in Go, a modern, open source programming language. 

You may verify your `go` installation via the terminal:

```
$> go version
go version go1.17.6 darwin/amd64
```

If you do not have go, we recommend installing it by:

#### MacOS

##### Homebrew
```
$> brew install go
```

#### Linux

See the [Go Installation Documentation](https://go.dev/doc/install).

### Mage

The build is run by Mage.

You may verify your `mage` installation via the terminal:

```
$> mage --version
Mage Build Tool 1.12.1
Build Date: 2021-12-15T21:00:02Z
Commit: 2f1ec40
built with: go1.17.6
```

#### MacOS

##### Homebrew

```
$> brew install mage
```

#### Linux

Installation instructions are on the [Magefile home page](https://magefile.org/).

### Java

This project is written in Java, a typesafe, compiled programming language. 

You may verify your `java` installation via the terminal by running `java -version`.

If you do not have Java, we recommend installing it 
via [SDKMan](https://sdkman.io/install). This is a project which will allow you 
to easily install the Java Development Kit (JDK), runtime (JRE), and related frameworks, 
build tools, and runtimes.

After you've installed SDKMan, you may install Java:

#### SDKMan (cross-platform instructions)

```shell
$> sdk install java 
 ...
Do you want java 17.0.2-open to be set as default? (Y/n): Y
Setting java 17.0.2-open as default.
```

You may test your installation:

```shell
$> java -version
openjdk version "17.0.2" 2022-01-18
OpenJDK Runtime Environment (build 17.0.2+8-86)
OpenJDK 64-Bit Server VM (build 17.0.2+8-86, mixed mode, sharing)
```

---
**NOTE**

You may additionally look for other Java versions to install by running `sdk list java`:

...or other installation candidates like Apache Ant, Apache Maven, etc, by running `sdk list`.

Consult the SDKMan documentation for more info.

---

## Build (Mage)

```
$> mage build
```

## Build (Java / Gradle)

### macOS / Linux
```shell
$> ./gradlew build
```

### Windows
```shell
$> gradlew.bat build
```

## Test (Mage)

```
$> mage test
```

## Test (Java / Gradle)

### macOS / Linux
```shell
$> ./gradlew test
```

### Windows
```shell
$> gradlew.bat test
```

---
**NOTE**

You may also combine Gradle build targets in one call, like:

```shell
$> ./gradlew clean build test
```

---

## Communications

### Issues

Anyone from the community is welcome (and encouraged!) to raise issues via 
[GitHub Issues](LINK_HERE)  ___***FIX LINK AND REMOVE THIS NOTICE***___.

### Discussions

Design discussions and proposals take place in our [discord](https://discord.gg/tbd).

We advocate an asynchronous, written debate model - so write up your thoughts and invite the community to join in!

### Continuous Integration

Build and Test cycles are run on every commit to every branch on [CircleCI](LINK_HERE).

 ___***FIX LINK ABOVE AND REMOVE THIS NOTICE***___

## Contribution

We review contributions to the codebase via GitHub's Pull Request mechanism. We have 
the following guidelines to ease your experience and help our leads respond quickly 
to your valuable work:

* Start by proposing a change either in Issues (most appropriate for small 
  change requests or bug fixes) or in Discussions (most appropriate for design 
  and architecture considerations, proposing a new feature, or where you'd 
  like insight and feedback)
* Cultivate consensus around your ideas; the project leads will help you 
  pre-flight how beneficial the proposal might be to the project. Developing early 
  buy-in will help others understand what you're looking to do, and give you a 
  greater chance of your contributions making it into the codebase! No one wants to 
  see work done in an area that's unlikely to be incorporated into the codebase.
* Fork the repo into your own namespace/remote
* Work in a dedicated feature branch. Atlassian wrote a great 
  [description of this workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow)
* When you're ready to offer your work to the project, first:
* Squash your commits into a single one (or an appropriate small number of commits), and 
  rebase atop the upstream `main` branch. This will limit the potential for merge 
  conflicts during review, and helps keep the audit trail clean. A good writeup for 
  how this is done is 
  [here](https://medium.com/@slamflipstrom/a-beginners-guide-to-squashing-commits-with-git-rebase-8185cf6e62ec), and if you're 
  having trouble - feel free to ask a member or the community for help or leave the commits as-is, and flag that you'd like 
  rebasing assistance in your PR! We're here to support you.
* Open a PR in the project to bring in the code from your feature branch.
* The maintainers noted in the `CODEOWNERS` file will review your PR and optionally 
  open a discussion about its contents before moving forward.
* Remain responsive to follow-up questions, be open to making requested changes, and...
  You're a contributor!
* And remember to respect everyone in our global development community. Guidelines 
  are established in our `CODE_OF_CONDUCT.md`.
