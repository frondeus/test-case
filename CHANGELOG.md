# Changelog

## 0.3.1
### Minor improvements:
* Refreshed readme
* Added CI for stable version of Rust. - thanks to @macisamuele
* Limited crate to Rust 1.29+ - thanks to @macisamuele
### Upgraded dependencies:
* Upgraded syn, quote and proc-macro-2 to v1
* Upgraded lazy-static to 1.4.0
* Upgraded insta to 0.11.0

## 0.3.0
### Breaking changes
* Crate has new maintainer: Wojciech Polak :hand: :tada:
* Crate has new name, as `test-case-derive` had no meaning for `derive` part.
* Delimiter for test case description is `;` instead of `::`.

  Reason: `::` is valid part of expression and rustc treats const variable as path
### New features
* Proper error propagation :tada:
  When there is for example a typo in function body, rustc can now show location
  of it instead of test_case location.
* Internally for tests crate uses `cargo insta` for snapshot testing
* Attribute is now compatible all other attributes like `#[should_panic]` 
