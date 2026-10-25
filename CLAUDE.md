Read [AGENTS.md](AGENTS.md) first: it is the guide for every agent and contributor working in this repository.

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
