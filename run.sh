#!/bin/sh
set -e
cargo build --release
./target/release/templates_rust
