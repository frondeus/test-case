//! # Overview
//! `test_case` crate provides procedural macro attribute that generates parametrized test instances.
//!
//! # Getting Started
//!
//! Crate has to be added as a dependency to `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-case = "3.1.0"
//! ```
//!
//! and imported to the scope of a block where it's being called
//! (since attribute name collides with rust's built-in `custom_test_frameworks`) via:
//!
//! ```rust
//! use test_case::test_case;
//! ```
//!
//! # Example usage:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use test_case::test_case;
//!
//!     #[test_case(-2, -4 ; "when both operands are negative")]
//!     #[test_case(2,  4  ; "when both operands are positive")]
//!     #[test_case(4,  2  ; "when operands are swapped")]
//!     fn multiplication_tests(x: i8, y: i8) {
//!         let actual = (x * y).abs();
//!
//!         assert_eq!(8, actual)
//!     }
//! }
//! ```
//!
//! Output from `cargo test` for this example:
//!
//! ```sh
//! $ cargo test
//!
//! running 4 tests
//! test tests::multiplication_tests::when_both_operands_are_negative ... ok
//! test tests::multiplication_tests::when_both_operands_are_positive ... ok
//! test tests::multiplication_tests::when_operands_are_swapped ... ok
//!
//! test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! ```
//!
//! # MSRV Policy
//!
//! Starting with version 3.0 and up `test-case` introduces policy of only supporting latest stable Rust.
//! These changes may happen overnight, so if your stack is lagging behind current stable release,
//! it may be best to consider locking `test-case` version with `=` in your `Cargo.toml`.
//!
//! # Documentation
//!
//! Most up to date documentation is available in our [wiki](https://github.com/frondeus/test-case/wiki).
pub use test_case_macros::test_case;
pub use test_case_macros::test_case as case;

#[cfg(feature = "with-regex")]
pub use regex::*;
