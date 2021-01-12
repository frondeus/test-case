#!/bin/bash

EDITOR=${EDITOR:-nvim}

cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
cargo test --all --all-features

$EDITOR Cargo.toml
cargo build

$EDITOR CHANGELOG.md
$EDITOR src/lib.rs

cargo readme > README.md

cargo publish --dry-run --allow-dirty

git add .
git commit
git push origin

echo "Next step: Wait for CI"
echo "Next step: \`git tag vX.Y.Z; git push --tags\`"
echo "Next step: Create release in Github"
echo "Next step: \`cargo publish\`"
