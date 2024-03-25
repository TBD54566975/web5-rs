// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "swift",
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .executable(
            name: "swift",
            targets: ["swift"]),
    ],
    targets: [
        // This is the target for the Rust integration.
        // It should match the module name of your Rust library.
        .systemLibrary(
            name: "jwkFFI", // Name for the C module
            path: "Sources/RustIntegration" // Path to the RustIntegration directory
        ),
        // This is the executable target for your Swift code.
        .executableTarget(
            name: "swift",
            dependencies: [
                "jwkFFI" // This depends on the Rust library.
            ]),
    ]
)