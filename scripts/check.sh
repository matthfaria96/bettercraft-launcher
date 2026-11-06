#!/bin/sh
# BetterCraft: run the checks GitHub's CI runs on main (ci.yml), so a merge or a change that would
# turn CI red is caught before it lands. Used by scripts/sync-upstream.sh and by the upstream sync
# workflow; run it by hand before pushing too.
set -eu
cd "$(dirname "$0")/.."

# CI builds with --locked: a Cargo.lock that needs updating fails there, so fail here first.
cargo fetch --locked
# fmt --check, clippy -D warnings, the tests and the wasm check (ci.yml's "test" job).
cargo xtask ci
# ci.yml's "web" job: clippy for the browser build, where desktop-only code must not be dead.
cargo clippy -p artcraft-launcher-core -p artcraft-launcher-web --target wasm32-unknown-unknown --locked -- -D warnings
