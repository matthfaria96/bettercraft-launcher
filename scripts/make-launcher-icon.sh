#!/bin/sh
# BetterCraft: builds assets/bettercraft/bettercraft-launcher.icns from launcher-icon-source.png.
set -eu
cd "$(dirname "$0")/.."
OUT=assets/bettercraft
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
swift scripts/make-launcher-icon.swift "$OUT/launcher-icon-source.png" "$TMP/icon-1024.png"
SET="$TMP/icon.iconset"
mkdir "$SET"
for s in 16 32 128 256 512; do
  sips -z $s $s "$TMP/icon-1024.png" --out "$SET/icon_${s}x${s}.png" >/dev/null
  d=$((s * 2))
  sips -z $d $d "$TMP/icon-1024.png" --out "$SET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$SET" -o "$OUT/bettercraft-launcher.icns"
cp "$TMP/icon-1024.png" "$OUT/bettercraft-launcher-1024.png"
echo "wrote $OUT/bettercraft-launcher.icns"
