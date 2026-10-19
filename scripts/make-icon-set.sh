#!/bin/sh
# BetterCraft: turns a folder of 1024 px icons (<id>.png, already on the macOS grid) into the
# layout the launcher embeds: icns/<id>.icns, <id>.png (256 px), dock/<id>.png (fits eframe's
# default icon, for the Dock patch) and small/<id>-96.png, small/<id>-64.png.
# Usage: scripts/make-icon-set.sh <dir-of-1024-pngs> <out-dir>
set -eu
IN="$1"
OUT="$2"
LIMIT=12052 # eframe's default icon: the largest PNG the Dock patch can put in the executable
mkdir -p "$OUT/icns" "$OUT/dock" "$OUT/small"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
for f in "$IN"/*.png; do
  id="$(basename "$f" .png)"
  set="$TMP/$id.iconset"
  mkdir "$set"
  for s in 16 32 128 256 512; do
    sips -z $s $s "$f" --out "$set/icon_${s}x${s}.png" >/dev/null
    d=$((s * 2))
    sips -z $d $d "$f" --out "$set/icon_${s}x${s}@2x.png" >/dev/null
  done
  iconutil -c icns "$set" -o "$OUT/icns/$id.icns"
  sips -Z 256 "$f" --out "$OUT/$id.png" >/dev/null
  sips -Z 96 "$f" --out "$OUT/small/$id-96.png" >/dev/null
  sips -Z 64 "$f" --out "$OUT/small/$id-64.png" >/dev/null
  fit=""
  for s in 256 240 224 208 192 176 160 144 128 120 112 104 96; do
    sips -Z $s "$f" --out "$TMP/dock.png" >/dev/null
    if [ "$(stat -f %z "$TMP/dock.png")" -le $LIMIT ]; then cp "$TMP/dock.png" "$OUT/dock/$id.png"; fit=$s; break; fi
  done
  [ -n "$fit" ] || { echo "error: no Dock PNG of $id fits $LIMIT bytes" >&2; exit 1; }
  echo "$id: dock ${fit}px"
done
