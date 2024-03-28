// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Web5",
    products: [
        .library(
            name: "Web5",
            targets: ["Web5"]),
    ],
    targets: [
        .binaryTarget(
          name: "Web5CoreRS",
          path: "libweb5-rs.xcframework"
        ),
        .target(
            name: "Web5",
            dependencies: [.target(name: "UniFFI")],
            path: "Sources/Web5"
        ),
        .target(
            name: "UniFFI",
            dependencies: [.target(name: "Web5CoreRS")],
            path: "Sources/UniFFI"
        ),
    ]
)