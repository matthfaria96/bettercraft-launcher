Read [AGENTS.md](AGENTS.md) first: it is the guide for every agent and contributor working in this repository.

New workspace crates must include `[lints] workspace = true` in `Cargo.toml` to inherit
`[workspace.lints]`. Production crate roots must declare the required crate-level lint attributes
before modules or other crate items; crate-level `//!` documentation may precede them.

## CI workflows
- Workflows in `.github/workflows/` call the shared workflows in [`storytold/craft-common-actions`](https://github.com/storytold/craft-common-actions) at `@main`, with their inputs and secrets. No other jobs: if no shared workflow does what's needed, it doesn't go here.
- Don't add or change CI in a feature or fix PR. If the existing workflows can't build or test a change, link an existing craft-common-actions issue or PR, or open a new issue, or a PR only for a small fix that is safe for every app and does not increase CI time.
- Some workflow files here still run their own jobs. Don't add to them. Moving a job into craft-common-actions is its own task, never part of another PR, and the job stays here until a shared workflow runs it.

## Showcase screenshots (witchmymac.com)

`BETTERCRAFT_SHOWCASE=1` (fork-only, `apps/artcraft-launcher/src/showcase.rs`) runs the launcher on a
demo state in memory for the site's screenshots: no network, no detection, every command ignored
(nothing is downloaded, installed, opened or removed), no tray icon, no Sparkle check, no
single-instance handshake, and the icon pass (`bettercraft::showcase`) only reports the badges the
choice would give, never touching an installed app. `BETTERCRAFT_SHOWCASE=settings` opens Settings;
`BETTERCRAFT_SHOWCASE_THEME=light|dark` and `BETTERCRAFT_SHOWCASE_SIZE=1120x900` set the theme and
window size. Run it from a copy with another bundle id and `HOME`/`CFFIXED_USER_HOME` pointing at a
temp folder (the icon mode and set are read from `$HOME/Library/Application Support/BetterCraft
Launcher/bettercraft-mac-mode` and `bettercraft-icon-set`), plus `ARTCRAFT_LAUNCHER_LOCALE=en`.
