#!/bin/sh
# BetterCraft: build the macOS app with the BetterCraft icon and install it as
# /Applications/BetterCraft Launcher.app. Fails if the installed icon isn't exactly ours.
set -eu
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"
ARCH="$(uname -m | sed 's/arm64/aarch64/')"
ICON=assets/bettercraft/logo-normal.icns
DEST="/Applications/BetterCraft Launcher.app"

packaging/macos/package.sh --arch "$ARCH"
BUILT="${CARGO_TARGET_DIR:-target}/macos-package/ArtCraft Launcher.app"
# Info.plist's CFBundleIconFile names the icon; replace that file with ours.
NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIconFile' "$BUILT/Contents/Info.plist")"
case "$NAME" in *.icns) ;; *) NAME="$NAME.icns" ;; esac
cp "$ICON" "$BUILT/Contents/Resources/$NAME"
codesign --force --deep --sign - "$BUILT"
codesign --verify --deep --strict "$BUILT"

pkill -f "$DEST/Contents/MacOS/" 2>/dev/null || true
sleep 1
rm -rf "$DEST"
ditto "$BUILT" "$DEST"
touch "$DEST"
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f "$DEST"

if ! cmp -s "$ICON" "$DEST/Contents/Resources/$NAME"; then
  echo "error: installed icon is not the BetterCraft icon" >&2
  exit 1
fi
echo "Installed $DEST with the BetterCraft icon."
open "$DEST"
