#!/bin/sh
# BetterCraft: builds assets/better-icons-craft/ (the original icons with the yellow "Craft" side
# badge) from assets/better-icons/icns/, in the same layout as assets/better-icons/.
set -eu
cd "$(dirname "$0")/.."
SRC=assets/better-icons
OUT=assets/better-icons-craft
LIMIT=12052 # eframe's default icon: the largest PNG the Dock patch can put in the executable
mkdir -p "$OUT/icns" "$OUT/dock" "$OUT/small"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
for f in "$SRC"/icns/*.icns; do
  id="$(basename "$f" .icns)"
  swift scripts/badge-icon.swift "$f" "$TMP/$id-1024.png"
  set="$TMP/$id.iconset"
  mkdir "$set"
  for s in 16 32 128 256 512; do
    sips -z $s $s "$TMP/$id-1024.png" --out "$set/icon_${s}x${s}.png" >/dev/null
    d=$((s * 2))
    sips -z $d $d "$TMP/$id-1024.png" --out "$set/icon_${s}x${s}@2x.png" >/dev/null
  done
  iconutil -c icns "$set" -o "$OUT/icns/$id.icns"
  sips -Z 256 "$TMP/$id-1024.png" --out "$OUT/$id.png" >/dev/null
  sips -Z 96 "$TMP/$id-1024.png" --out "$OUT/small/$id-96.png" >/dev/null
  sips -Z 64 "$TMP/$id-1024.png" --out "$OUT/small/$id-64.png" >/dev/null
  fit=""
  for s in 256 240 224 208 192 176 160 144 128 120 112 104 96; do
    sips -Z $s "$TMP/$id-1024.png" --out "$TMP/dock.png" >/dev/null
    if [ "$(stat -f %z "$TMP/dock.png")" -le $LIMIT ]; then cp "$TMP/dock.png" "$OUT/dock/$id.png"; fit=$s; break; fi
  done
  [ -n "$fit" ] || { echo "error: no Dock PNG of $id fits $LIMIT bytes" >&2; exit 1; }
  echo "$id: dock ${fit}px"
done
