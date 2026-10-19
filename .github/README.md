<p align="center">
  <img src="../docs/bettercraft/logo.png" alt="BetterCraft Launcher" width="128">
</p>

<h1 align="center">BetterCraft Launcher</h1>

<p align="center"><b>English</b> · <a href="README.pt-BR.md">Português</a></p>

<p align="center">
  <b>The Craft App Suite launcher, rethought: no visual noise, straight to the point, with better icons.</b><br>
  A better take on the <a href="https://github.com/storytold/craft-launcher">ArtCraft Launcher</a>
  experience, always kept up to date with it.
</p>

<p align="center">
  <img src="../docs/bettercraft/screenshot.png" alt="BetterCraft Launcher on macOS: the list of installed Craft Apps, each with its original icon beside the redesigned one, its version and an Open button" width="760">
</p>

## What it is

BetterCraft Launcher installs, updates and opens the apps of the Craft App Suite (PhotoCraft,
VectorCraft, FilmCraft, LightCraft, PdfCraft, EffectCraft, DesignCraft, WordCraft, DeckCraft,
GridCraft, CADCraft, SoundCraft and FigCraft), just like the original. What changes is the
experience: only what matters stays.

## What's better

- **No visual noise.** A compact list, one row per app: icon, name, version and one button. App
  pages, release notes, announcements, social links, search, sorting and the grid/table switch
  are gone.
- **Straight to the point.** The toolbar has two things: *Which icons?* and *Update*. Each app's
  menu has only Open, Show in Finder, Reinstall and Uninstall.
- **Redesigned icons.** With *Redesign*, every installed Craft App gets a new, clearer and more
  consistent icon in Finder, Launchpad and the Dock, even while the app is running. *Craft
  originals* puts the stock icons back. Each row shows which one the app is using.
- **Updates without surprises.** The version is read from the app itself, so updates work with
  either icon. The launcher never touches an app while it updates and dresses the new version
  right after.
- **Lean settings.** Updates, appearance and the icon style. Nothing else.
- **Always in step with the original.** A workflow brings in the changes from
  [storytold/craft-launcher](https://github.com/storytold/craft-launcher) every 6 hours. All new
  code lives in `crates/ui-egui/src/bettercraft/`, with one-line hooks elsewhere.

## Study mode (unofficial)

In Settings, the *Study* card swaps the apps' icons and names for those of the programs they
imitate (PhotoCraft → Adobe Photoshop, WordCraft → Microsoft Word…), with a yellow CRAFT ribbon on
the icon's corner. It is a test mode, not official. Those programs' icons and names are
trademarks of their owners.

## Install on macOS

Requires [Rust](https://rustup.rs) and the Xcode command line tools.

```sh
git clone https://github.com/matthfaria96/bettercraft-launcher.git
cd bettercraft-launcher
scripts/install-mac.sh
```

The script builds the app, installs it to `/Applications/BetterCraft Launcher.app` and opens it.
To pull in the original's latest changes and reinstall:

```sh
scripts/sync-upstream.sh && scripts/install-mac.sh
```

Technical details in [BETTERCRAFT.md](../BETTERCRAFT.md).

## License

Same as the original: MIT or Apache-2.0. The ArtCraft name and mark belong to their owners
(`docs/brand/LICENSE-brand.txt`).
