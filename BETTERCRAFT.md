# BetterCraft Launcher

A fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher) that stays in step
with it (`.github/workflows/bettercraft-sync.yml` merges upstream every 6 hours; or run
`scripts/sync-upstream.sh`). It adds two toolbar buttons:

- **Compact rows** (always on, no button): one slim row per app with its icon pair (the Craft icon small and
  behind, the original program's icon it gets on the Mac larger and in front), name, one status
  line and the main button, instead of the tall cards.
- **Mac icons ● ON/OFF**: the switch shows its state (and `…` while it works; the tooltip counts the apps done). On, it gives the installed `.app` bundles the original programs' icons in Finder,
  Launchpad and the Dock (macOS). An app update replaces the bundle and drops the custom icon, so
  the launcher puts it back within seconds while it runs. Turning the button off restores the Craft
  icons. Inside the launcher the Craft icons stay as they are.

The icons used:

| Craft App | Original |
|---|---|
| PhotoCraft | Adobe Photoshop |
| VectorCraft | Adobe Illustrator |
| FilmCraft | Adobe Premiere Pro |
| LightCraft | Adobe Lightroom |
| PdfCraft | Adobe Acrobat |
| EffectCraft | Adobe After Effects |
| DesignCraft | Adobe InDesign |
| FigCraft | Figma |
| WordCraft | Microsoft Word |
| SoundCraft | Adobe Audition |
| DeckCraft | Microsoft PowerPoint |
| GridCraft | Microsoft Excel |
| CADCraft | AutoCAD |

Fork-only files: `crates/ui-egui/src/bettercraft/`, `assets/better-icons/`, this file, the sync
workflow and script. Upstream files touched: one-line hooks in `lib.rs` and `views.rs` (toolbar and
`grid_block`), plus the `upstream-look` feature in `crates/ui-egui/Cargo.toml` so
upstream's UI tests keep checking upstream's cards. The original icons are trademarks of their owners and are used here for personal use.

## Launcher icon and installing on macOS

The BetterCraft icon (`assets/bettercraft/launcher-icon-source.png`) replaces the ArtCraft icon in
the app bundle. `scripts/make-launcher-icon.sh` turns the source art into
`assets/bettercraft/bettercraft-launcher.icns` (macOS rounded square with margin), and
`scripts/install-mac.sh` builds the app, swaps that icon in, re-signs, installs
`/Applications/BetterCraft Launcher.app` and fails if the installed icon isn't byte-for-byte ours.
Upstream's own icon files are left untouched, so their changes never conflict with this one.
