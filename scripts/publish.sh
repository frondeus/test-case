#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR="${SCRIPT_DIR}/.."
CURRENT_DIR=$(pwd)

cd "${REPO_DIR}"

set -eo xtrace

./scripts/test_all.sh

nvim Cargo.toml
nvim crates/test-case-macros/Cargo.toml
nvim crates/test-case-core/Cargo.toml
cargo build

nvim CHANGELOG.md
nvim src/lib.rs

cargo readme > README.md

git add .
git commit
git push origin

set +o xtrace

echo "Next step: Wait for CI"
echo "Next step: \`git tag vX.Y.Z; git push --tags\`"
echo "Next step: Create release in Github"
echo "Next step: \`cargo publish\`"

cd "${CURRENT_DIR}"
