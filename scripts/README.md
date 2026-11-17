# scripts

Small shell helpers for developers, run by hand (for example `scripts/setup.sh`).

The app never runs these. Helpers that must work on every OS, or that CI depends on,
belong in an `xtask/` crate instead, written in Rust.
