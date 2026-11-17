# Architecture

One Cargo **workspace**: programs in `apps/`, libraries in `crates/`, shared settings in the root
`Cargo.toml`.

## Layers

A crate may depend only on crates in **lower** layers.

| Layer | Crates | Role | May use egui? |
|---|---|---|---|
| L0 | `brand` | Name, repository, version. No dependencies | No |
| L1 | later `update`, `text`, `project`, `settings`, `syntax`, `lsp`, `terminal`, `account` | One job each; pure logic; unit-tested without a window | **No** |
| L2 | later `engine` | Combines L1 into "the open session"; owns the command registry | **No** |
| L3 | `ui` | Draws state; turns clicks and keys into calls to lower layers | Yes |
| L4 | `apps/example-ide` | Starts everything | Yes |

## Rules

1. **egui appears only in L3 and L4.** A logic crate that needs to tell the UI something returns
   data or sends a message; it never draws.
2. **L1 crates don't depend on each other** unless there is a clear reason.
3. **One crate, one job.** If its description needs "and", consider splitting it.
4. **`main.rs` stays small:** read arguments, build the state, open the window.
5. **Internal crates are named `ide-*`**, not after the product, so a rename touches few files.
6. **Shared dependencies are declared once** in the root `Cargo.toml` under
   `[workspace.dependencies]`; crates write `name.workspace = true`.
7. **A new feature** = an L1 crate for its logic (with tests) + a panel or widget in `ui` + one
   line in the app to connect them.
8. **Never block the window.** Network, disk and other slow work run on a background thread and
   report back; the UI thread only draws.

## What is where

| Path | What |
|---|---|
| `apps/example-ide/src/main.rs` | Window options, then `eframe::run_native` |
| `crates/brand/src/lib.rs` | The only place the product name is written |
| `crates/ui/src/lib.rs` | The `App` struct and the panel layout |
| `crates/ui/src/panels/` | `chat.rs`, `editor.rs`, `files.rs`, `status_bar.rs` |
| `crates/ui/src/update_dialog.rs` | The update dialog; takes its data as a parameter |
