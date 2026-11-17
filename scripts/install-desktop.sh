#!/usr/bin/env bash
# Registers the app with the desktop for your user only (no sudo), so the dock and the
# Activities view show the real name and icon while you develop with `cargo run`.
#
#   scripts/install-desktop.sh            install (or refresh after changing the icon)
#   scripts/install-desktop.sh --remove   undo
#
# The .deb does the same system-wide; this is only for development.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
id="$(sed -n 's/^pub const MACHINE_NAME: &str = "\(.*\)";/\1/p' "$root/crates/brand/src/lib.rs")"
apps="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
icons="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor"

refresh() {
    # Only refresh an icon cache that already exists; without one, GTK reads the folders directly.
    if [ -f "$icons/icon-theme.cache" ]; then gtk-update-icon-cache -q -t -f "$icons" || true; fi
    update-desktop-database -q "$apps" 2>/dev/null || true
}

if [ "${1:-}" = "--remove" ]; then
    rm -f "$apps/$id.desktop"
    find "$icons" -path "*/apps/$id.*" -delete 2>/dev/null || true
    refresh
    echo "Removed $id from the desktop."
    exit 0
fi

# Icons: same layout as assets/app-icon/hicolor/.
mkdir -p "$icons"
cp -r "$root/assets/app-icon/hicolor/." "$icons/"

# Desktop entry: point Exec at the dev build instead of /usr/bin.
mkdir -p "$apps"
sed "s|^Exec=$id|Exec=$root/target/debug/$id|" "$root/packaging/linux/$id.desktop" >"$apps/$id.desktop"

refresh
echo "Installed $id.desktop and icons for $USER. Restart the app to see them."
