# BetterCraft Launcher

A fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher) that stays in step
with it (`.github/workflows/bettercraft-sync.yml` merges upstream every 6 hours; or run
`scripts/sync-upstream.sh`). The only addition is an **Original Icons** button in the toolbar,
which swaps each Craft App's icon for the original program's:

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

Fork-only files: `crates/ui-egui/src/better_icons.rs`, `assets/better-icons/`, this file, the sync
workflow and script. Upstream files touched: one line each in `lib.rs` and `views.rs`, and a hook in
`icons.rs`. The original icons are trademarks of their owners and are used here for personal use.
