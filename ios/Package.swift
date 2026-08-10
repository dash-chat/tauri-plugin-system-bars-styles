// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "tauri-plugin-system-theme",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "tauri-plugin-system-theme",
            type: .static,
            targets: ["tauri-plugin-system-theme"])
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        .target(
            name: "tauri-plugin-system-theme",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources")
    ]
)
