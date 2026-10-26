# rust-lab

A scratch Cargo project for learning Rust, kept apart from Warsha. Nothing here ships.

## Rules for agents (Claude, opencode)
- Toolchain: stable Rust (1.99), edition 2024.
- Before calling a task done: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- No new dependency in `Cargo.toml` without the owner's OK; say which crate and why first.
- No `unsafe`.
- Write our own code and words. Never paste text or long code from the private books or course material.
- Small steps: one exercise or concept per commit, with a test that shows it working.
