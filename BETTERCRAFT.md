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

- **Mac icons [ Clone | New ]** (toolbar): *New* gives every installed Craft App the BetterCraft
  icon (`assets/craft-icons/`, from `scripts/make-craft-icons.sh`) in Finder, Launchpad and the
  Dock, including the Dock icon of a running app; the apps keep their names. *Clone* puts each
  app's own icon back. Each installed row shows NEW (green) or CLONE (red).
- **Settings → Mac icons: Normal | Study** (two cards with previews): *Normal* is the official look
  above. *Study* (unofficial, for testing) uses the original programs' icons with a yellow CRAFT
  corner ribbon (`assets/study-icons/`, from `scripts/make-study-icons.sh`) and shows the original
  programs' names (e.g. PhotoCraft → Adobe Photoshop) through a localized display name. Switching
  re-dresses the installed apps at once.

How it stays safe: the `.app` keeps its file name and version keys, so the launcher still finds,
checks and updates it. After a change the app is re-signed ad hoc (keeping its entitlements). The
Craft originals (bundle `.icns`, the icons embedded in the executable, with their offsets) are kept
inside the bundle, so going back never depends on the icon pictures. App updates replace the
bundle; the launcher dresses the new version within seconds and never touches an app mid-update.

Study set (original program and name for each Craft App):

| Craft App | Original program |
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

Fork-only files: `crates/ui-egui/src/bettercraft/`, `assets/craft-icons/`, `assets/study-icons/`,
`assets/bettercraft/`, `scripts/`, this file and the sync workflow. Upstream files touched: small
hooks in `lib.rs`, `views.rs`, `app.rs` (no upstream self-update) and `settings_dialog.rs`, plus the `upstream-look` feature in `crates/ui-egui/Cargo.toml` so
upstream's UI tests keep checking upstream's cards. The Study set's original program icons and names are trademarks of their owners, used here for
personal study only.

## Launcher icon and installing on macOS

The launcher's own icon follows the icon set. Normal uses the official logo
(`assets/bettercraft/logo-normal.icns`), which `scripts/install-mac.sh` puts in the bundle, then
re-signs, installs `/Applications/BetterCraft Launcher.app` and checks the installed icon is
byte-for-byte that one. In Study the launcher shows the older "A" logo
(`assets/bettercraft/logo-study.icns`, from `scripts/make-launcher-icon.sh`) as its Finder icon and
its running Dock icon, and the official one again when switched back. Upstream's own icon files
are left untouched.

Settings in this build hold only Updates, Appearance and Mac icons (no language, pre-releases,
install folder, background options or Quit button).
