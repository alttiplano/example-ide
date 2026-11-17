//! The product's name, identifiers and version.
//! This is the only place in the code where the name is written.

/// Shown to people: window title, dock and app menu, About, update dialog.
/// After changing it, run `scripts/install-desktop.sh` so the dock shows it too.
pub const DISPLAY_NAME: &str = "Example IDE";

/// Used by machines: binary, package and .desktop file name.
pub const MACHINE_NAME: &str = "example-ide";

/// Where releases are published, as `owner/repo` on GitHub.
pub const GITHUB_REPO: &str = "alttiplano/example-ide";

/// The version from the workspace `Cargo.toml`, filled in at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
