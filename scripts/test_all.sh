#!/usr/bin/env bash

set -e

cargo clean

cargo +nightly clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt --all
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-stable cargo +stable test --workspace --all-features
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-nightly cargo +nightly test --workspace --all-features
