#!/bin/sh
# Merge the latest storytold/craft-launcher into this fork locally and push.
set -eu
cd "$(dirname "$0")/.."
git remote get-url upstream >/dev/null 2>&1 || git remote add upstream https://github.com/storytold/craft-launcher.git
git fetch upstream main
git merge --no-edit upstream/main || { [ "$(git diff --name-only --diff-filter=U)" = ".github/dependabot.yml" ] && git rm -q .github/dependabot.yml && git commit --no-edit; }
git rm -q --ignore-unmatch .github/dependabot.yml; git diff --cached --quiet || git commit -m "chore: keep Dependabot off in the fork"
git push origin HEAD:main
