# BetterCraft Launcher

A fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher) that stays in step
with it (`.github/workflows/bettercraft-sync.yml` merges upstream every 6 hours; or run
`scripts/sync-upstream.sh`). The BetterCraft build keeps only what matters: install apps, update them, and set their Mac icons
and names. It drops upstream's noise (app pages, release notes, announcements, social links,
search, sort, the grid/table switch) and upstream's launcher self-update, which would replace this
fork with ArtCraft Launcher; update the fork with `scripts/sync-upstream.sh` and
`scripts/install-mac.sh`. Updates read the version from the app's `Info.plist`, which the icon and
name changes never touch, and the icon/name pass skips any app the launcher is downloading or
installing, then dresses the new version within seconds.

What it adds:

- **Compact rows** (always on, no button): one slim row per app with its icon pair (the Craft icon small and
  behind, the original program's icon it gets on the Mac larger and in front), name, one status
  line and the main button, instead of the tall cards.
- **Mac icons [ Off | Icons | + Names ]** (toolbar): what the installed `.app` bundles look like in
  Finder, Launchpad and the Dock (macOS). *Icons* sets each app's custom icon to the original
  program's. *+ Names* also shows the original program's name (e.g. PhotoCraft → Adobe Photoshop)
  through a localized display name; the `.app` keeps its file name, so the launcher still finds and
  updates it. Adding names changes sealed files, so the app is re-signed ad hoc (keeping its
  entitlements); *Off* or *Icons* removes the names again. The menu bar keeps the executable's name.
  The Dock is forced too (`bettercraft/dock_icon.rs`): the bundle's `.icns` (shown while an app
  starts) and the icon the app hands eframe at runtime (eframe's default PNG, or the app's own PNG
  embedded in its executable, found by comparing it with the app's Craft icon) are replaced, with
  the Craft originals kept in the bundle to put back. Each installed row shows an ORIGINAL / CRAFT
  badge. App updates (through this launcher) replace the bundle; the launcher re-applies the
  choice within seconds while it runs, and an app that is open shows the new Dock icon from its
  next start.

The icons used:

| Craft App | Original program (icon and name) |
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
