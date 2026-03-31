//! Library crate: imported by `main.rs` to show multi-file Rust layout.

pub fn greet(name: &str) -> String {
    format!("Hello from {} (Rust template)", name)
}
