#!/bin/sh
# BetterCraft: builds the launcher icons in the macOS shape (824 px rounded square centred on a
# transparent 1024 px canvas) from full-bleed art:
#   logo-study.icns + bettercraft-launcher-1024.png  from launcher-icon-source.png (Study set)
#   logo-normal.icns + logo-normal-256.png            from logo-normal-source.png  (Normal set; also the
#                                                      app bundle icon and the Dock icon while running)
set -eu
cd "$(dirname "$0")/.."
OUT=assets/bettercraft
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

icns() { # icns <icon-1024.png> <out.icns>
  SET="$TMP/icon.iconset"
  rm -rf "$SET"; mkdir "$SET"
  for s in 16 32 128 256 512; do
    sips -z $s $s "$1" --out "$SET/icon_${s}x${s}.png" >/dev/null
    d=$((s * 2))
    sips -z $d $d "$1" --out "$SET/icon_${s}x${s}@2x.png" >/dev/null
  done
  iconutil -c icns "$SET" -o "$2"
}

swift scripts/make-launcher-icon.swift "$OUT/launcher-icon-source.png" "$TMP/study-1024.png"
icns "$TMP/study-1024.png" "$OUT/logo-study.icns"
cp "$TMP/study-1024.png" "$OUT/bettercraft-launcher-1024.png"
echo "wrote $OUT/logo-study.icns"

swift scripts/make-launcher-icon.swift "$OUT/logo-normal-source.png" "$TMP/normal-1024.png"
icns "$TMP/normal-1024.png" "$OUT/logo-normal.icns"
sips -z 256 256 "$TMP/normal-1024.png" --out "$OUT/logo-normal-256.png" >/dev/null
echo "wrote $OUT/logo-normal.icns and logo-normal-256.png"
