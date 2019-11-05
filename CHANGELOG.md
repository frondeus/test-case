# Changelog

## v1.0.0-alpha-1
### New features
* Add support for two new keywords: `panics` and `matches` which can be applied after `=>` token.

  `matches` gives possibility to test patterns, like:
  ```rust
  #[test_case("foo" => matches Some(("foo", _)))]
  ```

  `panics` gives `should_panic(expected="...")` for one test_case:
  ```rust
  #[test_case(true  => panics "Panic error message" ; "This should panic")]
  #[test_case(false => None                         ; "But this should return None")]
  ```
### Major improvements
* Add extra unit tests - thanks to @luke-biel
* Replace `parented_test_case` with parsing `test_case` directly from args - thanks to @luke-biel
### Minor improvements
* Moved `lazy-static` dependency to `dev-dependencies`
### Upgraded dependencies
* Upgraded `insta` to `0.12.0`

## v0.3.3
### Bugfixes
* Fixed "inconclusive" feature with different cases.

## v0.3.2
### Bugfixes
* Added support for `impl Trait` - it worked in v2.x crate.
### Minor improvements
* Added extra test cases
### Upgraded dependencies
* Upgraded `version_check` to `v0.9.1`

## v0.3.1
### Minor improvements:
* Refreshed readme
* Added CI for stable version of Rust. - thanks to @macisamuele
* Limited crate to Rust 1.29+ - thanks to @macisamuele
### Upgraded dependencies:
* Upgraded `syn`, `quote` and `proc-macro-2` to `v1`
* Upgraded `lazy-static` to `1.4.0`
* Upgraded `insta` to `0.11.0`

## v0.3.0
### Breaking changes
* Crate has new maintainer: Wojciech Polak :hand: :tada:
* Crate has new name, as `test-case-derive` had no meaning for `derive` part.
* Delimiter for test case description is `;` instead of `::`.

  Reason: `::` is valid part of expression and rustc treats const variable as path
### New features
* Proper error propagation :tada:
  When there is for example a typo in function body, rustc can now show location
  of it instead of `test_case` location.
* Internally for tests crate uses `cargo insta` for snapshot testing
* Attribute is now compatible all other attributes like `#[should_panic]` 
