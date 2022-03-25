#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR="${SCRIPT_DIR}/.."
CURRENT_DIR=$(pwd)

cd "${REPO_DIR}"

set -eo xtrace

./scripts/test_all.sh

nvim Cargo.toml
cargo build

nvim CHANGELOG.md
nvim src/lib.rs

cargo readme > README.md

cargo publish --dry-run --allow-dirty

git add .
git commit
git push origin

set +o xtrace

echo "Next step: Wait for CI\n"
echo "Next step: \`git tag vX.Y.Z; git push --tags\`\n"
echo "Next step: Create release in Github\n"
echo "Next step: \`cargo publish\`"

cd "${CURRENT_DIR}"
