#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}/.."

# Use the active toolchain, whether supplied by Nix, rustup, or the system.
# CI covers both stable and nightly. Select matching compiler diagnostics locally.
if [[ -z ${SNAPSHOT_DIR:-} ]]; then
    case "$(rustc --version)" in
        *-nightly*) SNAPSHOT_DIR=rust-nightly ;;
        *) SNAPSHOT_DIR=rust-stable ;;
    esac
fi
export SNAPSHOT_DIR

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
