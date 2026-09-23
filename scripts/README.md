# Development and release checks

Run `./scripts/test_all.sh` with Cargo, rustc, Clippy, and rustfmt available
on `PATH`. It uses your active toolchain and checks formatting, lints, and
workspace tests with all features.
It preserves build caches and does not install or update toolchains.

Stable compilers use the `rust-stable` diagnostic snapshots; nightly compilers
use `rust-nightly`. Set `SNAPSHOT_DIR` to override this selection.
CI tests both stable and nightly. With rustup, you can also run either locally:

```sh
rustup run stable ./scripts/test_all.sh
rustup run nightly ./scripts/test_all.sh
```

`./scripts/publish.sh` prepares a release using the same checks. It additionally
requires `cargo-get`, `cargo-readme`, an editor (`EDITOR`, defaulting to `vi`),
and a clean working tree.
The script edits package versions before validation; if validation fails,
review those edits before retrying. It commits and pushes the release preparation,
then prints manual tagging and publishing steps to follow after CI passes.
