# 🏗️ Example IDE — Architecture

A desktop IDE written in Rust with **egui/eframe 0.37**. Linux first, shipped as a `.deb`, updating
itself from GitHub Releases.

## 📁 Project structure

```sh
example-ide/
├── .cargo/
│   └── config.toml                  # Cargo settings for this project (empty until needed)
├── .config/                         # Settings for extra tools (nextest, typos…) — empty for now
├── .github/
│   └── workflows/                   # CI and release pipelines — Phase 3 and 5
├── apps/                            # Programs you run and ship
│   └── example-ide/
│       ├── Cargo.toml               # Package "example-ide"; the .deb settings go here later
│       └── src/main.rs              # Opens the window. No UI code, no logic
├── crates/                          # Libraries that do the work, one job each
│   ├── brand/
│   │   └── src/lib.rs               # Name, repo, version — the only place the name is written
│   └── ui/                          # Everything on screen (the only crate using egui)
│       └── src/
│           ├── lib.rs               # The App: state + panel layout
│           ├── panels/
│           │   ├── chat.rs          # Left: messages + input
│           │   ├── editor.rs        # Middle: file name + code box
│           │   ├── files.rs         # Right: fake file list
│           │   └── status_bar.rs    # Bottom: name, version, "Check for updates"
│           └── update_dialog.rs     # Update dialog; takes its data as a parameter
├── assets/                          # Icons, fonts, images the app ships with
│   └── app-icon/                    # The app icon — see its README.md
│       ├── app-icon.svg             # The master: the only icon file you edit
│       ├── options/                 # Colour choices to copy over the master
│       ├── app-icon.png             # Generated: window icon (embedded by main.rs)
│       └── hicolor/                 # Generated: Linux icon sizes, named after the app id
├── packaging/
│   └── linux/
│       └── example-ide.desktop      # Name + icon in the dock and app menu; file name = app id
├── scripts/                         # Developer shell helpers
│   ├── app-icon.sh                  # Renders app-icon.svg into app-icon.png and hicolor/
│   └── install-desktop.sh           # Registers the dev build with the desktop (--remove to undo)
├── target/                          # Build output (gitignored)
├── .gitattributes                   # Line endings: LF everywhere
├── .gitignore
├── AGENTS.md                        # Rules for AI assistants
├── ARCHITECTURE.md                  # This file
├── CLAUDE.md                        # Points Claude Code to AGENTS.md
├── Cargo.lock                       # Exact dependency versions (committed)
├── Cargo.toml                       # Workspace: version, shared deps, lints, build profiles
├── clippy.toml                      # Code-checker settings
├── guide.md                         # The mission plan
├── LICENSE                          # MIT
├── README.md
├── rust-toolchain.toml              # Rust compiler: stable + rustfmt + clippy
└── rustfmt.toml                     # Code style: 120 columns
```

## 🧱 Layers

A crate may depend only on crates in **lower** layers.

| Layer | Crates | Role | egui? |
|---|---|---|---|
| L0 | `brand` | Name, repo, version. No dependencies | No |
| L1 | later `update`, `text`, `project`, `settings`, `syntax`, `lsp`, `terminal`, `account` | One job each; pure logic; tested without a window | **No** |
| L2 | later `engine` | Combines L1 into "the open session"; owns the command list | **No** |
| L3 | `ui` | Draws state; turns clicks and keys into calls to lower layers | Yes |
| L4 | `apps/example-ide` | Starts everything | Yes |

## 📏 Rules

1. **egui only in L3 and L4.** Logic crates return data or send messages; they never draw.
2. **L1 crates don't depend on each other** unless there is a clear reason.
3. **One crate, one job.** If its description needs "and", split it.
4. **`main.rs` stays small:** read arguments, build the state, open the window.
5. **Internal crates are named `ide-*`**, not after the product, so a rename touches few files.
6. **Shared dependencies are declared once** in the root `Cargo.toml`; crates write `name.workspace = true`.
7. **A new feature** = an L1 crate (with tests) + a panel in `ui` + one line in the app.
8. **Never block the window.** Slow work runs on a background thread and reports back.
9. **The app id ties the desktop together.** `MACHINE_NAME` is the Wayland app id, the `.desktop`
   file name and the icon file name. If they differ, the dock shows the raw id and a generic icon.
