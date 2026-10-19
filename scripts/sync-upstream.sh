#!/bin/sh
# Merge the latest storytold/craft-launcher into this fork locally and push.
set -eu
cd "$(dirname "$0")/.."
git remote get-url upstream >/dev/null 2>&1 || git remote add upstream https://github.com/storytold/craft-launcher.git
git fetch upstream main
git merge --no-edit upstream/main
git push origin HEAD:main
