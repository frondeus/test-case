#!/usr/bin/env bash

set -e

cargo clean

cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-stable cargo +stable test --workspace --all-features
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-nightly cargo +nightly test --workspace --all-features
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-1.49.0 cargo +1.49 test --workspace --all-features
