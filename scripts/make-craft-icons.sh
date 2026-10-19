#!/bin/sh
# BetterCraft: the launcher's own icons for the Craft Apps, from the 512 px PNGs kept in
# assets/craft-icons/src/ (<App>_512.png, art filling all but 24 px of each side).
# Usage: scripts/make-craft-icons.sh [folder like ~/Downloads/Crafts_Icons to copy the sources from]
set -eu
cd "$(dirname "$0")/.."
SRC=assets/craft-icons/src
if [ $# -ge 1 ]; then
  mkdir -p "$SRC"
  for d in "$1"/*Craft; do app="$(basename "$d")"; cp "$d/${app}_512.png" "$SRC/${app}_512.png"; done
fi
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
for f in "$SRC"/*_512.png; do
  app="$(basename "$f" _512.png)"
  id="$(echo "$app" | tr '[:upper:]' '[:lower:]')"
  swift scripts/fit-icon.swift "$f" 0.046875 "$TMP/$id.png"
done
scripts/make-icon-set.sh "$TMP" assets/craft-icons
