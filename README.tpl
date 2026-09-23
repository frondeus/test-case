[![Crates.io](https://img.shields.io/crates/v/test-case.svg)](https://crates.io/crates/test-case)
[![Crates.io](https://img.shields.io/crates/d/test-case.svg)](https://crates.io/crates/test-case)
[![Docs.rs](https://docs.rs/test-case/badge.svg)](https://docs.rs/test-case)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
[![Build Status](https://github.com/frondeus/test-case/workflows/Test/badge.svg)](https://github.com/frondeus/test-case/actions)
{{badges}}

# Test Case

{{readme}}

# License

Licensed under of MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

# Contributing

Project roadmap is available at [link](https://github.com/frondeus/test-case/issues/74). All contributions are welcome.

Recommended tools:
* `cargo readme` - to regenerate README.md based on template and lib.rs comments
* `cargo insta`  - to review test snapshots
* `cargo edit`   - to add/remove dependencies
* `cargo fmt`    - to format code
* `cargo clippy` - for all insights and tips
* `cargo fix`    - for fixing warnings

## Releasing

Install `cargo-get` and `cargo-readme`, and ensure stable and nightly Rust are
available. From a clean working tree, run:

```sh
EDITOR="code --wait" ./scripts/publish.sh
```

The script uses `$EDITOR` (default: `vi`); editor arguments are supported as
whitespace-separated words. It prompts for a version, updates all three packages
and their internal dependency requirements, runs validation, regenerates the
README, and opens the changelog. After editing the changelog, it commits the
release files and pushes the branch to origin. If validation changes source
files, it stops so you can review and commit those changes manually.

Wait for CI, then follow the printed commands to tag the release, push that tag,
and create a GitHub release. Publish `test-case-core`, then `test-case-macros`,
then `test-case`, waiting for each dependency to become available on crates.io.
Publishing is manual; GitHub Actions only runs validation and tests.
