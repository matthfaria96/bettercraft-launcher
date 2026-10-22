#!/bin/sh
# BetterCraft: build the Linux app, install it for the current user and open it.
# Installs to ~/.local/bin with a menu entry and icon in ~/.local/share.
set -eu
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"

cargo build --release -p artcraft-launcher

BIN="$HOME/.local/bin/artcraft-launcher"
mkdir -p "$HOME/.local/bin"
rm -f "$BIN"
cp target/release/artcraft-launcher "$BIN"
chmod +x "$BIN"

DATA="$HOME/.local/share"
APPS="$DATA/applications"
ICONS="$DATA/icons/hicolor/256x256/apps"
mkdir -p "$APPS" "$ICONS"
cp assets/bettercraft/logo-normal-256.png "$ICONS/bettercraft-launcher.png"

cat > "$APPS/bettercraft-launcher.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=BetterCraft Launcher
Comment=Install, update and set the icons of the Craft App Suite
Exec=$BIN
Icon=bettercraft-launcher
Terminal=false
Categories=Utility;
EOF

"$BIN" &
