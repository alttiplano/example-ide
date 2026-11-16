# example-ide: build guide

> Written 2026-11-16. This file is the plan for the first mission of the IDE. It is meant to be
> read by an AI assistant in a fresh session and executed one phase at a time with the owner.
> `example` is a placeholder name; see [The name](#the-name).

## 1. The mission

Build the smallest possible desktop app and prove the whole delivery chain works:

1. A window with three panels: **chat** on the left, **code** in the middle, **files** on the right.
2. The code lives on GitHub and is built automatically (CI/CD).
3. The app is installed on the owner's PC from a `.deb`.
4. A new version is published, and the installed app updates itself **with one button**.

The mission is finished when an installed `0.0.1` updates itself to `0.0.2` from inside the app.

**Not in this mission:** a real code editor, a terminal, AI in the chat, login, profiles,
Windows, macOS. Those are in the [roadmap](#9-roadmap-after-the-mission) and must not be started
early.

## 2. Who you are working with

- The owner knows only the basics of Rust and comes from TypeScript and VS Code extensions.
  Explain each new Rust idea in a sentence or two when it first appears. Useful comparisons:
  crate = npm package, crates.io = npmjs.com, `Cargo.toml` = `package.json`,
  `Cargo.lock` = `package-lock.json`, `cargo add` = `npm install`.
- Work **one phase at a time**. At the end of each phase, show the owner how to see the result,
  and wait for their go-ahead before starting the next one.
- Ask before anything that is public or hard to undo: `git push`, creating a tag, publishing a
  release, installing system packages.
- Keep it simple. If something in this guide turns out to be wrong or outdated, say so and
  propose the fix; do not silently work around it.

## 3. Decisions already made

| Topic | Decision | Why |
|---|---|---|
| Language | Rust only | Speed; no webview |
| UI library | **egui + eframe**, official version from crates.io, pinned to `0.37` | Easiest to learn, large community, used by three fast apps we studied |
| Not used | Tauri, Electron, GPUI, Floem | Tauri/Electron are webviews; GPUI is hard and tied to Zed's repo; Floem is too young |
| Our own look | A `crates/ui` crate on top of egui | Our style and widgets live here; this is how the studied apps do it (`craft-ui`) |
| Fork of egui | **No**, unless a missing feature forces it | A fork must be re-merged by hand on every egui release |
| Repository | `https://github.com/alttiplano/example-ide`, public, MIT | The updater needs public releases |
| Platforms | Linux only for now | The owner can test it; Windows and macOS come later |
| Package format | `.deb` | Installs like a normal program on Zorin/Ubuntu/Debian |
| Where installers live | GitHub Releases of the repo, nowhere else | One copy; the website only links to it |
| CI/CD | GitHub Actions; a version tag publishes a release | No installers built by hand |
| Updater | Our own small crate, `crates/update` | See [Why our own updater](#why-our-own-updater) |
| First version | `0.0.1`; the update test is `0.0.2` | Owner's choice |
| Chat panel | Empty placeholder, no AI | Out of scope for this mission |

### The owner's machine (checked 2026-11-16)

- Zorin OS 18.1 (based on Ubuntu 24.04 "noble"), x86_64, Wayland session, glibc 2.39.
- Installed: `cargo`/`rustc` 1.100, `git`, `cc`, `pkexec`, `apt-get`, `dpkg-deb`.
- **Not** installed: `gh` (GitHub CLI), `pkg-config`, `cargo-deb`.

## 4. Lessons from the apps we studied

Three open-source Rust apps from one team were analysed: SoundCraft (~110k lines), PhotoCraft
(~1.0M lines) and VectorCraft (~1.37M lines). All use egui and feel fast. Zed (~1.82M lines, GPUI)
was the comparison. Startup times were not measured; these are the habits visible in their code,
and the rules this project adopts from day one:

1. **Layers.** Logic crates never depend on the UI library. Only `crates/ui` and the app may use
   egui. This keeps logic testable and makes a future UI change cheap.
2. **Few dependencies.** Their lock files are less than half the size of Zed's. Every new crate
   must be justified; prefer `default-features = false` and enable only what is used.
3. **Never block the window.** Anything slow (network, disk, the update download) runs on a
   background thread and reports back; the UI thread only draws.
4. **Do nothing at startup that can wait.** Open the window first; check for updates after.
5. **One place for the name and version.** See [The name](#the-name).
6. **Everything is a command** (later). Menus, shortcuts and a command palette should all call
   one registry of actions. Not needed in this mission, but do not design against it.

## 5. Project layout

The structure follows the studied apps (`apps/`, `crates/`, `packaging/`, `docs/`): one Cargo
**workspace** holding many small crates. A workspace is like a monorepo: one repository, one
`Cargo.lock`, one `target/` folder, several packages that depend on each other.

### Architecture for this mission

```sh
example-ide/
├── Cargo.toml                  # workspace: members, shared version, shared dependencies, profiles
├── Cargo.lock                  # exact dependency versions; committed
├── apps/
│   └── example-ide/            # L4  the program: main.rs opens the window and wires crates together
│       ├── Cargo.toml          #     also holds [package.metadata.deb] for the .deb
│       └── src/main.rs         #     stays small: no UI code, no logic
├── crates/
│   ├── brand/                  # L0  name, machine name, GitHub owner/repo, version. No dependencies
│   ├── update/                 # L1  update logic: check, download, verify, install. NO egui
│   └── ui/                     # L3  everything egui
│       └── src/
│           ├── lib.rs          #     the App struct and the frame layout
│           ├── theme.rs        #     colours, spacing, rounding: the look lives here
│           ├── panels/         #     chat.rs, editor.rs, files.rs, status_bar.rs
│           ├── widgets/        #     reusable pieces (the showcase card, later buttons, cards)
│           └── update_dialog.rs
├── assets/
│   └── icons/                  # app icon (original artwork only)
├── packaging/
│   └── linux/                  # example-ide.desktop and anything the .deb needs
├── docs/
│   ├── architecture.md         # the layer table below, kept up to date
│   └── releasing.md            # the release recipe
├── .github/workflows/
│   ├── ci.yml                  # every push: format, lint, test, build
│   └── release.yml             # version tag: build the .deb, publish the release
├── guide.md                    # this file
├── README.md
├── LICENSE                     # MIT
└── .gitignore                  # /target
```

### Where it grows (do not create these yet)

Empty crates are clutter. Each of these is created in the roadmap step that needs it:

```sh
crates/
├── brand/        # L0  (exists)
├── text/         # L1  the text buffer: rope, cursors, selections, undo
├── project/      # L1  folders, the file tree, opening and saving, file watching
├── settings/     # L1  user settings and where they are stored
├── syntax/       # L1  tree-sitter grammars and highlighting
├── lsp/          # L1  talking to language servers
├── terminal/     # L1  the terminal's process and screen state
├── account/      # L1  login token and the website's API
├── update/       # L1  (exists)
├── engine/       # L2  the open session (tabs, buffers) and the command registry
└── ui/           # L3  (exists) panels, widgets and the editor widget
xtask/            # build helpers written in Rust (`cargo xtask ...`), e.g. the layer check
```

### The layers

A crate may depend only on crates in **lower** layers. This one rule is what keeps the project
maintainable as it grows, and it is how the studied apps stay fast and testable.

| Layer | Crates | Role | May use egui? |
|---|---|---|---|
| L0 | `brand` | Constants shared by everything | No |
| L1 | `update`, later `text`, `project`, `settings`, `syntax`, `lsp`, `terminal`, `account` | One job each; pure logic; unit-tested without a window | **No** |
| L2 | later `engine` | Combines L1 into "the open IDE session"; owns the list of commands | **No** |
| L3 | `ui` | Draws state and turns clicks and keys into calls to lower layers | Yes |
| L4 | `apps/example-ide` | Starts everything | Yes |

Rules for whoever writes the code:

1. **egui appears only in L3 and L4.** If a logic crate needs to tell the UI something, it
   returns data or sends a message; it never draws.
2. **L1 crates do not depend on each other** unless there is a clear reason (for example
   `syntax` needing `text`). Sideways dependencies are how a codebase becomes a knot.
3. **One crate, one job.** If a crate's description needs the word "and", consider splitting it.
4. **`main.rs` stays small.** It reads arguments, builds the state, and starts the window.
5. **Internal crates are named `ide-*`** (`ide-brand`, `ide-update`, `ide-ui`), not after the
   product. Only the app package and binary are called `example-ide`, so renaming the product
   touches very few files.
6. **Shared dependencies are declared once** in the root `Cargo.toml` under
   `[workspace.dependencies]`; each crate writes `eframe.workspace = true`. One place to
   upgrade a version.
7. **Adding a feature means:** a new L1 crate for its logic (with tests), a panel or widget in
   `ui` to show it, and one line in `apps/example-ide` to connect them.
8. When there are more than a handful of crates, add an `xtask` command that fails CI if a
   crate breaks the layer rule (the studied apps do this with `cargo xtask layers`).

Workspace `Cargo.toml` essentials:

- `[workspace.package]` holds `version = "0.0.1"`, `edition = "2024"`, `license = "MIT"`; every
  crate uses `version.workspace = true`, so the version exists in one place.
- Build profiles, copied from the studied apps:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]   # dependencies compiled optimised, so egui is smooth in debug
opt-level = 2
debug = false

[profile.release]
opt-level = 3
lto = "thin"
```

### The name

`example` is a placeholder. All naming must come from **one constants file**, the `crates/brand`
crate, holding: display name (`Example IDE`),
machine name (`example-ide`), GitHub `owner/repo` (`alttiplano/example-ide`). The window title,
the updater and the About text read from there. The only other places the name appears are
`Cargo.toml` package names, the `.desktop` file and the workflow files; list them in `README.md`
so a rename is a short checklist.

**Naming checkpoint, before the first public announcement:** choose the final name → check the
domain, GitHub and every social network are free → buy the domain → register the accounts →
create the final repository and rename. Installed copies look for updates at the repository
address, so rename before real users install the app.

### egui 0.37 notes (checked against the 0.37.0 source)

The API differs from most older tutorials:

- The app implements `eframe::App` with `fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)`
  (older versions used `update(&mut self, ctx, frame)`). An optional `fn logic(&mut self, ctx, frame)`
  runs per-frame non-drawing work.
- Panels are `egui::Panel::left("id")`, `::right`, `::top`, `::bottom`, with `.default_size()`,
  `.min_size()`, `.exact_size()`, `.resizable()`, then `.show(ui, |ui| { ... })`. The middle is
  `egui::CentralPanel::default().show(ui, |ui| { ... })`. Add side panels **before** the central one.
- Start with `eframe::run_native("Example IDE", options, Box::new(|_cc| Ok(Box::new(App::default()))))`.
- Animations: `ctx.animate_bool(id, value)` and `ctx.animate_bool_with_easing(id, value, easing)`.

When unsure, read the source in `~/.cargo/registry/src/*/egui-0.37.0/` rather than guessing.

## 6. The phases

Each phase ends with a **check** the owner can see with their own eyes.

### Phase 0: preparation

1. Clone `https://github.com/alttiplano/example-ide` and confirm the owner can push to it
   (`git ls-remote` works; decide HTTPS or SSH). `gh` is not installed and is not required.
2. Confirm `cargo --version` works.
3. Copy this `guide.md` into the repository root.

**Check:** the repo is cloned and a test push of `guide.md` succeeds (ask first).

### Phase 1: the window with three panels

1. Create the workspace, `apps/example-ide`, `crates/brand` and `crates/ui` as in the layout
   above (`crates/update` is created in Phase 6).
2. `cargo add eframe` in the crates that need it; pin `0.37`.
3. Panels, all placeholder content:
   - **Left, chat:** a title, a scrolling list of messages, a text input at the bottom. Typing
     and pressing Enter just adds the text to the list.
   - **Middle, code:** a file-name label and egui's built-in multiline text box in code style
     (`TextEdit::multiline(..).code_editor()`), showing a few sample lines.
   - **Right, files:** a title and a fixed, fake list of file names; clicking one selects it.
   - **Bottom:** a thin status bar showing the app name and version (`env!("CARGO_PKG_VERSION")`).
     Showing the version is required: it is how the update is confirmed later.
4. The side panels are resizable by dragging.
5. **The update button, as UI only.** The owner wants to see it from the first day. Add a
   "Check for updates" button in the status bar. Clicking it opens the update dialog with fake
   data: app name, "0.0.1 → 0.0.2", a "What's new" link, and the buttons **Later** and
   **Update and restart**. Both buttons only close the dialog for now. Put the dialog in
   `crates/ui/src/update_dialog.rs` and have it take its data as parameters, so Phase 6 connects
   the real logic without redrawing it.

**Check:** `cargo run` opens a window with the three panels, the version in the status bar, and
an update button that opens the (fake) update dialog.

An unbuilt, untested draft of this phase exists on the owner's disk at
`/home/zorin/Downloads/hello-rust/test-ide/my-ide/` (single `main.rs`). It may be used as a
starting point, but it was never compiled.

### Phase 2: the look test

The owner wants a modern UI (animations, polished cards, pixel-art backgrounds, later blur).
Before investing further, prove egui can deliver it.

1. Add a small theme in `crates/ui`: dark colours, rounded corners, spacing, one accent colour.
2. Build **one showcase card** inspired by the owner's reference image (a storage-usage card):
   a rounded dark card with a title, a segmented coloured bar drawn with the painter, a small
   legend, and a button. Add one animation (for example the bar growing on open, or a hover
   highlight).
3. Show it somewhere unobtrusive, for example in the chat panel's empty state.

Known limit: **blur behind a panel** ("frosted glass") is not built into egui. It needs a
third-party crate (`backdrop-blur-egui`, only at 0.1.0) or a custom shader. Do not attempt it
in this mission; note it in the roadmap.

**Check:** the owner looks at the card and says whether the look is good enough to continue
with egui. If not, stop and discuss before going on.

### Phase 3: GitHub and CI

1. Add `README.md`, `LICENSE` (MIT), `.gitignore` (`/target`).
2. Add `.github/workflows/ci.yml`, running on every push and pull request on `ubuntu-24.04`:
   `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build`.
3. If the build fails on missing system libraries, install them in the workflow. egui's README
   lists these for Linux; verify which are actually needed rather than installing all blindly:
   `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev`.
4. Cache Cargo downloads and builds (`Swatinem/rust-cache`) so runs are quick.

**Check:** the push shows a green check mark on GitHub.

### Phase 4: the `.deb`, built locally

1. Use **`cargo-deb`** (`cargo install cargo-deb`, then `cargo deb -p example-ide`). It reads a
   `[package.metadata.deb]` section in `apps/example-ide/Cargo.toml`.
2. The package must install:
   - the binary to `/usr/bin/example-ide`
   - `packaging/linux/example-ide.desktop` to `/usr/share/applications/`
   - an icon to `/usr/share/icons/hicolor/256x256/apps/example-ide.png` (a simple placeholder
     icon is fine; it must be original, not copied from another product)
3. Name the output `example-ide-<version>-linux-x86_64.deb`. The updater relies on this pattern.
4. Install it: `sudo apt install ./example-ide-0.0.1-linux-x86_64.deb`.

**Check:** "Example IDE" appears in the system's application menu and opens from there.
Uninstall with `sudo apt remove example-ide` works.

### Phase 5: the release pipeline

1. Add `.github/workflows/release.yml`, triggered by pushing a tag `v*`:
   1. Fail if the tag (`v0.0.1`) does not match the workspace version (`0.0.1`).
   2. Build in release mode and run `cargo deb`.
   3. Write `SHA256SUMS.txt` (`sha256sum *.deb > SHA256SUMS.txt`).
   4. Create the GitHub release and upload the `.deb` and `SHA256SUMS.txt`.
      The workflow needs `permissions: contents: write`.
2. Build on `ubuntu-22.04` (older glibc) so the binary also runs on older systems than the
   owner's. If that causes trouble, `ubuntu-24.04` is acceptable for now.
3. Document the release recipe in `README.md`:
   bump the version in the workspace `Cargo.toml` → commit → `git tag v0.0.X` → `git push --tags`.

**Check:** tag `v0.0.1` produces a release on GitHub with both files; the owner removes the
local install and installs the downloaded `.deb` instead.

### Phase 6: the update button

Logic in `crates/update` (no egui), dialog in `crates/ui`.

**The flow:**

1. **Check.** After the window is visible, a background thread asks
   `https://api.github.com/repos/alttiplano/example-ide/releases/latest` (send a `User-Agent`
   header; GitHub rejects requests without one). No token is needed for a public repo; the limit
   is 60 requests per hour per IP, so check once at startup and when the user asks.
2. **Compare.** Parse the tag (`v0.0.2` → `0.0.2`) and compare with the running version using
   the `semver` crate. Only a **newer** version is offered.
3. **Offer.** A dialog: app name, "0.0.1 → 0.0.2", a "What's new" link to the release page,
   and two buttons: **Later** and **Update and restart**.
4. **Download.** On "Update and restart": download the `.deb` asset matching
   `example-ide-<version>-linux-x86_64.deb` and `SHA256SUMS.txt` into a temporary folder, on a
   background thread, showing progress.
5. **Verify.** Compute the file's SHA-256 and compare with the published one. On mismatch, delete
   the file and show an error. Never install an unverified file.
6. **Install.** Run `pkexec apt-get install -y <path-to-deb>`. The system shows its password
   window. If the user cancels, return to the dialog with a clear message.
7. **Restart.** When the install succeeds, start `/usr/bin/example-ide` as a new process and quit
   the current one.

**Rules:**

- Only download from URLs under `https://github.com/alttiplano/example-ide/releases/`. Treat
  everything in the API answer as untrusted: bound sizes, validate the checksum is 64 hex digits.
- Offer the button only when the running program is the installed one (its path is
  `/usr/bin/example-ide`). A `cargo run` development build shows a link to the releases page
  instead, so development never tries to replace itself.
- No network or disk work on the UI thread. Use a thread plus a channel, and call
  `ctx.request_repaint()` when a result arrives.
- No internet, API error or rate limit: stay quiet at startup; show a message only when the user
  asked for the check.
- Add a "Check for updates" item (a menu or a button in the status bar) for manual checks.
- For testing without publishing: an environment variable such as
  `EXAMPLE_IDE_PRETEND_VERSION=0.0.0` makes the app believe it is older, so the dialog can be
  tested against the real `v0.0.1` release. The studied apps use the same trick.

**Suggested dependencies** (keep to these unless there is a reason): `ureq` (HTTP, with rustls),
`serde` + `serde_json` (read the API answer), `semver` (compare versions), `sha2` (checksum).

**Unit tests** for the pure parts: tag parsing, version comparison, picking the right asset
from a release, parsing `SHA256SUMS.txt`, rejecting bad URLs and bad checksums.

**Check:** with the pretend-version variable, the dialog appears and the full flow runs.

#### Why our own updater

The studied apps share a library, `craft-update` (MIT/Apache). It was examined and **not**
chosen as a dependency:

- It is about 21,500 lines and covers five operating systems, six package formats and a separate
  launcher app; this mission needs one format on one system.
- It is built around that team's suite: default developer name, website links to their domain,
  `ai.storyteller.*` identifiers, and their release file naming.
- It is an internal library inside a repository of 100+ crates, with no promise of stability.

It is an excellent **reference**, and its design is what this phase copies: GitHub release →
`SHA256SUMS.txt` → verify → install the platform's way → restart. Its notes on Linux packages
say exactly what we do: "the system package manager, with the password prompt; then restart".
On the owner's disk: `~/.cargo/git/checkouts/craft-libs-*/*/crates/update/src/` (start with
`self_update.rs`, `updater.rs`, `release.rs`, `http.rs`).

### Phase 7: the real update test

1. Make one visible change (for example a new line in the status bar or a colour).
2. Bump the version to `0.0.2`, commit, tag `v0.0.2`, push the tag (ask first).
3. Wait for the release to appear on GitHub.
4. Open the **installed** `0.0.1` from the application menu.
5. The dialog offers `0.0.1 → 0.0.2`. Press **Update and restart**, enter the password.

**Check, and end of the mission:** the app restarts by itself and the status bar shows `0.0.2`
with the visible change.

## 7. Definition of done

- [ ] `cargo run` shows three panels and the version.
- [ ] The owner approved the look test.
- [ ] CI is green on GitHub.
- [ ] `v0.0.1` is a GitHub release with a `.deb` and `SHA256SUMS.txt`.
- [ ] The app is installed on the owner's PC from that `.deb` and opens from the menu.
- [ ] The installed `0.0.1` updated itself to `0.0.2` with the button.
- [ ] `README.md` explains how to run, build, release, and where the name appears.

## 8. Known limits to tell the owner about

- The update asks for the system password each time, because installing a `.deb` needs
  administrator rights. An AppImage build (no password) can be added later.
- The checksum protects against a corrupted or swapped download, not against someone who takes
  control of the GitHub repository. Signed releases are a later improvement.
- Only x86_64 Linux on Debian-family systems is covered.

## 9. Roadmap after the mission

In rough order; each is its own plan, not part of this guide.

1. **Naming checkpoint** and the final repository (see [The name](#the-name)).
2. **Open and save real files; a real file tree** for a chosen folder, with tabs.
3. **The editor widget.** egui's text box is too slow and too limited for large files. Write our
   own widget: text stored in a rope (`ropey`), draw only the visible lines, line numbers,
   selections. References: Zed's `crates/rope`, `crates/text`, `crates/editor` (read for ideas
   only: Zed is GPL-3, so do **not** copy its code into this MIT project) and Lapce.
4. **Syntax colouring** with `tree-sitter`.
5. **Language servers (LSP):** completion, errors, go to definition.
6. **Terminal, search, git**, each loaded only when used.
7. **Command registry and palette** (lesson 6).
8. **Chat with AI** in the left panel.
9. **Login and profile** through the website's API: the app opens the browser, receives a token
   and stores it in the system keyring. The desktop app never talks to the database directly.
10. **Our own UI layer grows:** more widgets, animations, backgrounds, blur.
11. **More platforms and formats:** AppImage, Windows (`.msi`), macOS (`.dmg`, needs paid code
    signing), and signed releases.

## 10. Reference material on the owner's disk

Under `/home/zorin/Downloads/hello-rust/test-ide/`:

| Path | What it is | Useful for |
|---|---|---|
| `analiza-1/` | SoundCraft (egui, ~110k lines) | The smallest full example: `apps/soundcraft/src/main.rs` (startup), `crates/ui-egui/src/updates_ui.rs` (update dialog), `packaging/linux/`, `docs/releasing.md` |
| `analiza-v/` | VectorCraft | `docs/architecture.md`: layers, commands, rendering off the UI thread |
| `analiza-ph/` | PhotoCraft | `AGENTS.md`: the team's working rules; `docs/` for performance notes |
| `zed-ide/` | Zed (GPUI, GPL-3) | Editor internals, **ideas only**; `crates/auto_update/` for another updater design |
| `my-ide/` | Unbuilt draft of Phase 1 | Optional starting point |
| `alttiplano-decisiones.pdf` | The owner's wider strategy (in Spanish) | Brand, repositories, website, login; context only |

These projects are references. Their code is MIT/Apache (the three Craft apps) or GPL-3 (Zed);
learn from them, but write this project's code fresh, and never copy their names, icons or art.

