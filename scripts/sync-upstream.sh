#!/bin/sh
# Merge the latest storytold/craft-launcher into this fork locally, check it like CI does and push.
# On a conflict or a failing check nothing is pushed: fix the files, commit, and run
# scripts/check.sh then `git push origin HEAD:main`.
set -eu
cd "$(dirname "$0")/.."
git remote get-url upstream >/dev/null 2>&1 || git remote add upstream https://github.com/storytold/craft-launcher.git
git fetch upstream main
if ! git merge --no-edit upstream/main; then
  if [ "$(git diff --name-only --diff-filter=U)" = ".github/dependabot.yml" ]; then
    git rm -q .github/dependabot.yml
    git commit --no-edit
  else
    echo "Merge conflicts to resolve:" >&2
    git diff --name-only --diff-filter=U >&2
    exit 1
  fi
fi
git rm -q --ignore-unmatch .github/dependabot.yml
git diff --cached --quiet || git commit -m "chore: keep Dependabot off in the fork"
if ! scripts/check.sh; then
  echo "The merge is committed locally but fails the CI checks: fix, commit, then push." >&2
  exit 1
fi
git push origin HEAD:main
