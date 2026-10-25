// swift-tools-version: 5.9
// BetterCraft: the Sparkle helper that updates BetterCraft Launcher (see main.swift).
import PackageDescription

let package = Package(
    name: "BetterCraftUpdater",
    platforms: [.macOS(.v11)],
    dependencies: [
        // MIT. In-app updates (feed at witchmymac.com/appcast/bettercraft.xml).
        .package(url: "https://github.com/sparkle-project/Sparkle", from: "2.10.0"),
    ],
    targets: [
        .executableTarget(
            name: "BetterCraftUpdater",
            dependencies: [.product(name: "Sparkle", package: "Sparkle")],
            path: "Sources/BetterCraftUpdater"
        ),
    ]
)
