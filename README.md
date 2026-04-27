# Tesseracs - Rust Template

A practical Rust starter for [Tesseracs](https://github.com/tesseracs) chat sessions.

Clone URL: `https://github.com/tesseracs/templates.rust`

## What this template shows

- A library-first layout with reusable modules in `src/`.
- Simple domain structs plus a formatter that produces console output.
- A tiny binary in `src/main.rs` that stays easy to replace.

## Layout

- `src/main.rs` - binary entry point.
- `src/lib.rs` - public module exports and sample project builder.
- `src/domain.rs` - project/task data structures.
- `src/report.rs` - string rendering helpers.
- `run.sh` - runs the crate with Cargo.

## Run

```sh
./run.sh
```

Or with Cargo directly:

```sh
cargo run
```

## Suggested next edits

- Replace the sample tasks with your real workflow.
- Move parsing, API calls, or file IO into new modules under `src/`.
- Add tests around `report::render_summary` when behavior becomes important.
