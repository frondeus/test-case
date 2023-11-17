#!/usr/bin/env bash

# Publish a new version of the crate.
#
# Dependencies:
# - cargo-get
# - nvim

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR="${SCRIPT_DIR}/.."
CURRENT_DIR=$(pwd)

cd "${REPO_DIR}"

set -eo xtrace

# Read current version from the Cargo.toml file
CURRENT_VERSION=$(cargo get package.version)
echo "Current version: ${CURRENT_VERSION}"
read -p 'New version: ' NEW_VERSION

# Update version in Cargo.toml files
sed -i '' "s/version       = \"${CURRENT_VERSION}\"/version       = \"${NEW_VERSION}\"/g" Cargo.toml
sed -i '' "s/version       = \"${CURRENT_VERSION}\"/version       = \"${NEW_VERSION}\"/g" crates/test-case-macros/Cargo.toml
sed -i '' "s/version       = \"${CURRENT_VERSION}\"/version       = \"${NEW_VERSION}\"/g" crates/test-case-core/Cargo.toml

# Validate the release
rustup update
./scripts/test_all.sh

# Update README if needed
cargo readme > README.md

# Add changelog entry
nvim CHANGELOG.md

# Push to github
git add .
git commit
git push origin

set +o xtrace

echo "Next step: Wait for CI"
echo "Next step: \`git tag vX.Y.Z; git push --tags\`"
echo "Next step: Create release in Github"
echo "Next step: \`cargo publish\`"

cd "${CURRENT_DIR}"
