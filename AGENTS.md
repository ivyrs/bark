# Bark contributor guide

## Project

Bark is a small Rust and GTK4 desktop shell for the niri Wayland compositor.
The intended scope is a minimal bar, launcher, wallpaper-derived themes, niri
support, and possible future Mango and Keymapp/ZSA integration.

Keep the architecture proportional to that scope. Prefer a direct solution
over frameworks, generic traits, service containers, or speculative backend
abstractions.

## Current structure

- `src/main.rs`: GTK application, layer-shell window, and top-level layout.
- `src/clock.rs`: clock widget and GLib timer.
- `src/workspaces.rs`: workspace presentation.
- `src/compositors/niri.rs`: niri IPC and workspace data.

Keep compositor modules free of GTK concerns. UI modules may consume
compositor data. Do not introduce a shared compositor trait until a second
backend actually exists.

## Rust and GTK guidelines

- Use Rust 2024 and preserve `unsafe_code = "forbid"`.
- Treat Clippy's pedantic lints as review prompts, not absolute design rules.
- Prefer explicit runtime fallbacks over crashing the whole bar with
  `unwrap()` or `expect()`.
- Keep GTK work on the main thread. Never block the GTK main loop with niri's
  event stream; use a background thread and hand data back safely.
- GTK object clones are reference-counted handles, not duplicate widgets.
- Prefer GLib facilities already available through GTK before adding a crate.
- Add dependencies only when they materially simplify a real requirement.

## Development workflow

Run before considering a change complete:

```sh
cargo fmt --check
cargo check
cargo clippy
```

Run `cargo test` when tests exist or when a change affects testable logic.

GTK identifies Bark by `rs.ivy.bark`. A second invocation activates the
existing process, so stop the running process before testing a newly compiled
binary. Keep the early guard in `build_ui()` that reuses an existing window.

## Working style

- Make small, reviewable edits; do not rewrite whole files unnecessarily.
- Explain compiler errors and Rust/GTK concepts before applying a fix.
- Prefer hints and incremental steps when mentoring the project owner.
- Avoid unrelated cleanup and speculative features.
- Preserve module boundaries unless a concrete responsibility requires a new
  one.
- Do not commit, amend, push, or create pull requests unless explicitly asked.
