#!/bin/sh
# BetterCraft: the Study/Test icons (the original programs' icons with the CRAFT corner ribbon),
# from the .icns kept in assets/study-icons/src/.
set -eu
cd "$(dirname "$0")/.."
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
for f in assets/study-icons/src/*.icns; do
  id="$(basename "$f" .icns)"
  swift scripts/badge-icon.swift "$f" "$TMP/$id.png"
done
scripts/make-icon-set.sh "$TMP" assets/study-icons
