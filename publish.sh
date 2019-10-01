#!/bin/bash

cargo clippy
cargo fmt --all
cargo test --all --all-features

nvim Cargo.toml
cargo build

nvim CHANGELOG.md
nvim src/lib.rs

cargo readme > README.md

cargo publish --dry-run --allow-dirty

git commit
