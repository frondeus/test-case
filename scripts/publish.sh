#!/usr/bin/env bash

# Prepare a release; publishing remains manual after CI passes.
# Requires cargo-get, cargo-readme, Rust with clippy/rustfmt, and ${EDITOR:-vi}.

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}/.."

# Refuse to mix unrelated work into the release commit.
if [[ -n $(git status --porcelain) ]]; then
    echo "Commit or stash existing changes before preparing a release." >&2
    exit 1
fi

cargo get --help > /dev/null
cargo readme --help > /dev/null
cargo clippy --version > /dev/null
cargo fmt --version > /dev/null
# Support editor arguments, e.g. EDITOR='code --wait', without evaluating shell code.
read -r -a EDITOR_COMMAND <<< "${EDITOR:-vi}"
command -v "${EDITOR_COMMAND[0]}" > /dev/null

CURRENT_VERSION=$(cargo get package.version)
echo "Current version: ${CURRENT_VERSION}"
read -r -p 'New version: ' NEW_VERSION

# Validate SemVer before interpolating the version into sed or Git commands.
NUMBER='(0|[1-9][0-9]*)'
PRERELEASE_ID="(${NUMBER}|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)"
SEMVER="^${NUMBER}\\.${NUMBER}\\.${NUMBER}(-${PRERELEASE_ID}(\\.${PRERELEASE_ID})*)?(\\+[0-9A-Za-z-]+(\\.[0-9A-Za-z-]+)*)?$"
if [[ ! $NEW_VERSION =~ $SEMVER || $NEW_VERSION == "$CURRENT_VERSION" ]]; then
    echo "Enter a valid SemVer version different from ${CURRENT_VERSION}." >&2
    exit 1
fi
if git rev-parse --verify --quiet "refs/tags/v${NEW_VERSION}" > /dev/null; then
    echo "Tag v${NEW_VERSION} already exists." >&2
    exit 1
fi

RELEASE_FILES=(Cargo.toml crates/test-case-macros/Cargo.toml crates/test-case-core/Cargo.toml README.md CHANGELOG.md)
TEMP_FILE=$(mktemp)
trap 'rm -f "$TEMP_FILE"' EXIT

# Avoid platform-specific sed -i syntax. Bump packages and their internal
# dependency requirements together so published crates require the new code.
for manifest in "${RELEASE_FILES[@]:0:3}"; do
    sed -E \
        -e "s/^(version[[:space:]]*=[[:space:]]*)\"[^\"]*\"/\1\"${NEW_VERSION}\"/" \
        -e "/^test-case-(macros|core)[[:space:]]*=/s/version[[:space:]]*=[[:space:]]*\"[^\"]*\"/version = \"${NEW_VERSION}\"/" \
        "$manifest" > "$TEMP_FILE"
    cat "$TEMP_FILE" > "$manifest"
done

./scripts/test_all.sh

# Keep the existing README intact if generation fails.
cargo readme > "$TEMP_FILE"
cat "$TEMP_FILE" > README.md
"${EDITOR_COMMAND[@]}" CHANGELOG.md

# Leave any unexpected changes outside release metadata for review.
if ! git diff --quiet -- . ':!Cargo.toml' ':!crates/test-case-macros/Cargo.toml' ':!crates/test-case-core/Cargo.toml' ':!README.md' ':!CHANGELOG.md'; then
    echo "Validation changed files outside the release metadata. Review changes and commit the release manually." >&2
    exit 1
fi

git add -- "${RELEASE_FILES[@]}"
git commit
git push origin

cat <<STEPS
Next steps:
1. Wait for CI to pass.
2. Tag this release commit and push only its tag:
   git tag v${NEW_VERSION}
   git push origin v${NEW_VERSION}
3. Create the GitHub release for v${NEW_VERSION}.
4. Publish in dependency order, waiting for each crate to become available:
   cargo publish -p test-case-core
   cargo publish -p test-case-macros
   cargo publish -p test-case
STEPS
