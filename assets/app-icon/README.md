# App icon

`app-icon.svg` is the only file you edit. Everything else in this folder is generated from it.

## Change the icon

1. Pick a colour from `options/` (orange, green, peach) and copy it over `app-icon.svg`,
   or change the colour under `TILE COLOUR` in `app-icon.svg`,
   or replace `app-icon.svg` with your own square SVG (512×512 works best).
2. Run `scripts/app-icon.sh` — it renders `hicolor/` and `app-icon.png`.
3. Run `scripts/install-desktop.sh` to see it in the dock while developing, then restart the app.

## Files

| File | What it is | Used by |
|---|---|---|
| `app-icon.svg` | The master | You |
| `options/*.svg` | Colour choices | You |
| `app-icon.png` | 256 px render | Window icon, embedded by `apps/*/src/main.rs` (X11, Windows) |
| `hicolor/<size>/apps/<app id>.png`, `hicolor/scalable/apps/<app id>.svg` | Linux icon theme | Dock, app menu, `.deb` |

The `<app id>` is `MACHINE_NAME` from `crates/brand`. It must match the `.desktop` file name and
the `Icon=` line in `packaging/linux/`, or the desktop shows a generic icon.

Original artwork, MIT like the rest of the project.
