#!/bin/bash
# BetterCraft: builds "BetterCraft Updater.app" (the Sparkle helper, see Sources/BetterCraftUpdater/main.swift)
# into <dest>, normally "<Launcher>.app/Contents/Helpers". Ad-hoc signed; the outer signature
# (package.sh, sign-app.sh) covers it. Today's Xcode builds it for macOS 12 and later; on macOS 11
# the launcher runs as before, without in-app updates.
# Usage: packaging/macos/updater/build.sh <dest-dir> <version> [universal|aarch64|x86_64]
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
DEST="$1"
VERSION="$2"
ARCH="${3:-universal}"
if [ -z "${DEVELOPER_DIR:-}" ] && [ -d /Applications/Xcode.app/Contents/Developer ]; then
  export DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer
fi
case "$ARCH" in
  universal) ARCHS=(--arch arm64 --arch x86_64) ;;
  aarch64) ARCHS=(--arch arm64) ;;
  x86_64) ARCHS=(--arch x86_64) ;;
  *) echo "unknown arch $ARCH" >&2; exit 2 ;;
esac

BUILD="${CARGO_TARGET_DIR:-$ROOT/target}/sparkle-updater"
swift build --package-path "$HERE" --scratch-path "$BUILD" -c release "${ARCHS[@]}"
BIN_DIR="$(swift build --package-path "$HERE" --scratch-path "$BUILD" -c release "${ARCHS[@]}" --show-bin-path)"

APP="$DEST/BetterCraft Updater.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources" "$APP/Contents/Frameworks"
cp "$BIN_DIR/BetterCraftUpdater" "$APP/Contents/MacOS/BetterCraft Updater"
cp -R "$BIN_DIR/Sparkle.framework" "$APP/Contents/Frameworks/"
install_name_tool -add_rpath "@executable_path/../Frameworks" "$APP/Contents/MacOS/BetterCraft Updater" 2>/dev/null || true
strip -x "$APP/Contents/MacOS/BetterCraft Updater" 2>/dev/null || true
cp "$ROOT/assets/bettercraft/logo-normal.icns" "$APP/Contents/Resources/AppIcon.icns"
cp "$BUILD/checkouts/Sparkle/LICENSE" "$APP/Contents/Resources/Sparkle-LICENSE.txt"
cat >"$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key><string>en</string>
  <key>CFBundleName</key><string>BetterCraft Updater</string>
  <key>CFBundleDisplayName</key><string>BetterCraft Launcher</string>
  <key>CFBundleExecutable</key><string>BetterCraft Updater</string>
  <key>CFBundleIdentifier</key><string>app.witchmymac.bettercraft-updater</string>
  <key>CFBundleIconFile</key><string>AppIcon</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>LSMinimumSystemVersion</key><string>12.0</string>
  <key>LSUIElement</key><true/>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSHumanReadableCopyright</key><string>Sparkle: MIT License, https://sparkle-project.org</string>
</dict>
</plist>
PLIST
plutil -lint "$APP/Contents/Info.plist" >/dev/null
codesign --force --options runtime --sign - "$APP/Contents/Frameworks/Sparkle.framework"
codesign --force --options runtime --sign - "$APP"
echo "Built $APP"
