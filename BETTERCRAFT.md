# BetterCraft Launcher

A fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher) that stays in step
with it (`.github/workflows/bettercraft-sync.yml` merges upstream every 6 hours; or run
`scripts/sync-upstream.sh`). It adds two toolbar buttons:

- **Compact** (on by default): one slim row per app (icon, name, one status line, the main button)
  instead of the tall cards.
- **Original Icons**: swaps each Craft App's icon for the original program's, in the launcher and,
  on macOS, on the installed `.app` bundles (Finder, Launchpad, Dock). An app update replaces the
  bundle and drops the custom icon, so the launcher puts it back within seconds. Turning the button
  off restores the Craft icons.

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
workflow and script. Upstream files touched: one-line hooks in `lib.rs`, `views.rs` (toolbar and
`grid_block`) and `icons.rs`, plus the `upstream-look` feature in `crates/ui-egui/Cargo.toml` so
upstream's UI tests keep checking upstream's cards. The original icons are trademarks of their owners and are used here for personal use.
