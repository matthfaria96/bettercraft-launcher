# BetterCraft Launcher

A fork of [storytold/craft-launcher](https://github.com/storytold/craft-launcher) that stays in step
with it (`.github/workflows/bettercraft-sync.yml` merges upstream every 6 hours; or run
`scripts/sync-upstream.sh`). The BetterCraft build keeps only what matters: install apps, update them, and set their icons
and names (on macOS, Windows and Linux). It drops upstream's noise (app pages, release notes, announcements, social links,
search, sort, the grid/table switch) and upstream's launcher self-update, which would replace this
fork with ArtCraft Launcher; update the fork with `scripts/sync-upstream.sh` and
`scripts/install-mac.sh`. Updates read the version from the app's `Info.plist`, which the icon and
name changes never touch, and the icon/name pass skips any app the launcher is downloading or
installing, then dresses the new version within seconds.

What it adds:

- **Icons [ Clone | New ]** (toolbar): *New* gives every installed Craft App the BetterCraft
  icon (`assets/craft-icons/`, from `scripts/make-craft-icons.sh`) in Finder, Launchpad and the
  Dock on macOS (including the Dock icon of a running app), in the Start menu, taskbar and
  desktop on Windows, or in the applications menu and dock on Linux (AppImages); the apps keep
  their names. *Clone* puts each app's own icon back. Each installed row shows NEW (green) or
  CLONE (red).
- **Settings → Icons: Normal | Study** (two cards with previews): *Normal* is the official look
  above. *Study* (unofficial, for testing) uses the original programs' icons with a yellow CRAFT
  corner ribbon (`assets/study-icons/`, from `scripts/make-study-icons.sh`) and shows the original
  programs' names (e.g. PhotoCraft → Adobe Photoshop) through a localized display name (macOS) or
  the shortcut's name (Windows). Switching re-dresses the installed apps at once.

How it stays safe: the `.app` keeps its file name and version keys, so the launcher still finds,
checks and updates it. After a change the app is re-signed ad hoc (keeping its entitlements). The
Craft originals (bundle `.icns`, the icons embedded in the executable, with their offsets) are kept
inside the bundle, so going back never depends on the icon pictures. App updates replace the
bundle; the launcher dresses the new version within seconds and never touches an app mid-update.

## Windows

On Windows the launcher dresses the same two sets through the Start menu, desktop and taskbar
shortcuts (`.lnk` files) that point at each installed Craft App:

- **Redesign** points each shortcut's icon at the BetterCraft `.ico` the launcher writes into
  `%APPDATA%\BetterCraft Launcher\icons` (one PNG frame per icon, wrapped as an ICO); the
  shortcuts keep their names.
- **Study** also renames each shortcut to the original program's name (e.g. `PhotoCraft` →
  `Adobe Photoshop`), so the name shows everywhere the shortcut does.

Shortcuts are found by walking the Start menu (all-users and per-user), the desktop and the
taskbar's pinned items for the app's `.exe`. The `.lnk` binary format is the shell's own, so the
launcher drives `WScript.Shell` through PowerShell rather than parsing the file itself; a rename
is a plain file rename, which leaves every other shortcut property untouched. Everything it
changes is recorded in a marker under `%APPDATA%\BetterCraft Launcher\apps`, so switching back to
"Clone" restores each shortcut's original icon and name without keeping the pictures.

The running window's taskbar icon (the app draws it itself, so it is not a shortcut) and the
`.exe`'s own Explorer icon (embedded in a signed executable) are left alone. Shortcuts the shell
stores for all users (e.g. a machine-wide install under `Program Files`) need the launcher to run
as administrator to be rewritten; a permission failure shows in the toolbar's "Last error" and
never touches the app.

## Linux

On Linux the launcher dresses the apps it installed as **AppImages** (the install method whose
freedesktop files it owns): the menu entry `<data>/applications/ai.storyteller.<app>.desktop`
(marked `X-ArtCraft-Launcher=true`) and its icon
`<data>/icons/hicolor/128x128/apps/ai.storyteller.<app>.png`.

- **Redesign** overwrites the icon PNG with the BetterCraft icon (resized to 128 px); the entry
  keeps its name.
- **Study** also changes the entry's `Name=` to the original program's name (e.g. `PhotoCraft` →
  `Adobe Photoshop`).

The state lives in the entry itself (an `X-BetterCraft-Style` key), so an update that rewrites the
entry drops the dressing and the next pass dresses the new copy — the same in-place approach as
macOS. "Clone" restores the app's own 128 px picture and the app name. Packages (`.deb`, `.rpm`,
Arch), Flatpaks and tarballs are left to their own desktop integration.

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

## Installing

On macOS, install the latest release with one line in Terminal:

```sh
curl -fsSL https://witchmymac.com/install.sh | bash -s bettercraft
```

or download `BetterCraftLauncher-x.y.z.dmg` from the
[Releases](https://github.com/matthfaria96/bettercraft-launcher/releases) page or
[witchmymac.com](https://witchmymac.com) (see the README for all three ways to install; the app is ad-hoc
signed, not notarized). Otherwise BetterCraft Launcher is built from source (the fork drops
upstream's release workflow and self-update). You need Rust 1.90 or newer and, on Linux, the
windowing headers.

**macOS** — `scripts/install-mac.sh` builds the app, installs it as
`/Applications/BetterCraft Launcher.app` and opens it:

```sh
./scripts/install-mac.sh
```

**Windows** — `scripts/install-windows.ps1` builds the app, installs it for the current user as
`%LOCALAPPDATA%\Programs\BetterCraft Launcher` with a Start menu shortcut, and opens it (PowerShell):

```powershell
.\scripts\install-windows.ps1
```

**Linux** — `scripts/install-linux.sh` builds the app, installs it as
`~/.local/bin/artcraft-launcher` with a menu entry and icon, and opens it. Install the windowing
headers first (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`):

```sh
./scripts/install-linux.sh
```

Then install the Craft apps and set their icons: the toolbar's "Which icons?" switches between the
apps' own icons and the BetterCraft icons, and Settings → Icons has the Normal and Study sets.
On Windows, apps installed for all users (under `Program Files`) have their Start menu shortcuts in
an all-users folder, so changing their icons needs the launcher running as administrator.

Fork-only files: `crates/ui-egui/src/bettercraft/`, `assets/craft-icons/`, `assets/study-icons/`,
`assets/bettercraft/`, `scripts/`, this file and the sync workflow. Upstream files touched: small
hooks in `lib.rs`, `views.rs`, `app.rs` (no upstream self-update) and `settings_dialog.rs`, plus the `upstream-look` feature in `crates/ui-egui/Cargo.toml` so
upstream's UI tests keep checking upstream's cards. The Study set's original program icons and names are trademarks of their owners, used here for
personal study only.

## Launcher icon

The launcher's own icon follows the icon set. Normal uses the official logo
(`assets/bettercraft/logo-normal.icns`), which `scripts/install-mac.sh` puts in the bundle, then
re-signs, installs `/Applications/BetterCraft Launcher.app` and checks the installed icon is
byte-for-byte that one. In Study the launcher shows the older "A" logo
(`assets/bettercraft/logo-study.icns`, from `scripts/make-launcher-icon.sh`) as its Finder icon and
its running Dock icon, and the official one again when switched back. Upstream's own icon files
are left untouched.

Settings in this build hold only Updates, Appearance and icons (no language, pre-releases,
install folder, background options or Quit button).
