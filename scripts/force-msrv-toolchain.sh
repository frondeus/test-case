#! /bin/sh
#
# Force the use of the MSRV toolchain (for use with the CI).
# Since action-rs/toolchain@v1 uses rustup 1.21.x, only the
# toolchain name can be given in the file.
#
# If you call this script in your working directory, do not
# forget that it will create a "rust-toolchain" file there.

set -e

root=$(dirname "$0")/..

version=$(sed -ne 's/rust-version *= *"\(.*\)"/\1/p' "$root"/Cargo.toml)
echo $version > "$root"/rust-toolchain
echo "Rust $version installed as the forced toolchain"
