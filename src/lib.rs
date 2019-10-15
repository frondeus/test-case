//! # Overview
//! This crate provides `#[test_case]` procedural macro attribute that generates multiple parametrized tests using one body with different input parameters.
//! A test is generated for each data set passed in `test_case` attribute.
//! Under the hood, all test cases that share same body are grouped into `mod`, giving clear and readable test results.
//!
//! # Getting Started
//!
//! First of all you have to add this dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-case = "0.3.3"
//! ```
//!
//! Additionally you have to import the procedural macro with `use` statement:
//!
//! ```rust
//! use test_case::test_case;
//! ```
//!
//! The crate depends on `proc_macro` feature that has been stabilized on rustc 1.29+.
//!
//! # Example usage:
//!
//! ```rust
//! #![cfg(test)]
//! extern crate test_case;
//!
//! use test_case::test_case;
//!
//! #[test_case( 2,  4 ; "when both operands are possitive")]
//! #[test_case( 4,  2 ; "when operands are swapped")]
//! #[test_case(-2, -4 ; "when both operands are negative")]
//! fn multiplication_tests(x: i8, y: i8) {
//!     let actual = (x * y).abs();
//!
//!     assert_eq!(8, actual)
//! }
//! ```
//!
//! Output from `cargo test` for this example:
//!
//! ```sh
//! $ cargo test
//!
//! running 3 tests
//! test multiplication_tests::when_both_operands_are_possitive ... ok
//! test multiplication_tests::when_both_operands_are_negative ... ok
//! test multiplication_tests::when_operands_are_swapped ... ok
//!
//! test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//! ```
//!
//! # Examples
//!
//! If your only assertion is just `assert_eq!`, you can pass the expectation as macro attribute using `=>` syntax:
//!
//! ```rust
//! # use test_case::test_case;
//! #[test_case( 2 => 2 ; "returns given number for positive input")]
//! #[test_case(-2 => 2 ; "returns opposite number for non-positive input")]
//! #[test_case( 0 => 0 ; "returns 0 for 0")]
//! fn abs_tests(x: i8) -> i8 {
//!    if x > 0 { x } else { -x }
//! }
//! ```
//!
//! Which is equivalent to
//!
//! ```rust
//! # use test_case::test_case;
//! #[test_case( 2, 2 ; "returns given number for positive input")]
//! #[test_case(-2, 2 ; "returns opposite number for non-positive input")]
//! #[test_case( 0, 0 ; "returns 0 for 0")]
//! fn abs_tests(x: i8, expected: i8){
//!    let actual = if x > 0 { x } else { -x };
//!
//!    assert_eq!(expected, actual);
//! }
//! ```
//!
//! Attributes and expectation may be any expresion unless they contain `=>`, e.g.
//!
//! ```rust
//! # use test_case::test_case;
//! #[test_case(None,        None    => 0 ; "treats none as 0")]
//! #[test_case(Some(2),     Some(3) => 5)]
//! #[test_case(Some(2 + 3), Some(4) => 2 + 3 + 4)]
//! fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
//!     x.unwrap_or(0) + y.unwrap_or(0)
//! }
//! ```
//!
//! Note: in fact, `=>` is not prohibited but the parser will always treat last `=>` sign as beginning of expectation definition.
//!
//! Test case names are optional. They are set using `;` followed by string literal at the end of macro attributes.
//!
//! Example generated code:
//!
//! ```rust
//! mod fancy_addition {
//!     #[allow(unused_imports)]
//!     use super::*;
//!
//!     fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
//!         x.unwrap_or(0) + y.unwrap_or(0)
//!     }
//!
//!     #[test]
//!     fn treats_none_as_0() {
//!         let expected = 0;
//!         let actual = fancy_addition(None, None);
//!
//!         assert_eq!(expected, actual);
//!     }
//!
//!     #[test]
//!     fn some_2_some_3() {
//!         let expected = 5;
//!         let actual = fancy_addition(Some(2), Some(3));
//!
//!         assert_eq!(expected, actual);
//!     }
//!
//!     #[test]
//!     fn some_2_3_some_4() {
//!         let expected = 2 + 3 + 4;
//!         let actual = fancy_addition(Some(2 + 3), Some(4));
//!
//!         assert_eq!(expected, actual);
//!     }
//! }
//! ```
//!
//! ## Inconclusive (ignored) test cases (since 0.2.0)
//!
//! If test case name (passed using `;` syntax described above) contains word "inconclusive", generated test will be marked with `#[ignore]`.
//!
//! ```rust
//! # use test_case::test_case;
//! #[test_case("42")]
//! #[test_case("XX" ; "inconclusive - parsing letters temporarily doesn't work but it's ok")]
//! fn parses_input(input: &str) {
//!     // ...
//! }
//! ```
//!
//! Generated code:
//! ```ignore
//! mod parses_input {
//!     // ...
//!
//!     #[test]
//!     pub fn _42() {
//!         // ...
//!     }
//!
//!     #[test]
//!     #[ignore]
//!     pub fn inconclusive_parsing_letters_temporarily_doesn_t_work_but_it_s_ok() {
//!         // ...
//!     }
//!
//! ```
//!
//! **Note**: word `inconclusive` is only reserved in test name given after `;`.

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};

use quote::quote;
use syn::parse_quote;
use test_case::TestCase;
use syn::spanned::Spanned;

mod test_case;
mod utils;

/// Generates tests for given set of data
///
/// In general, test case consists of four elements:
///
/// 1. _(Required)_ Arguments passed to test body
/// 2. _(Optional)_ Expected result
/// 3. _(Optional)_ Test case name
/// 4. _(Required)_ Test body
///
///  When _expected result_ is provided, it is compared against the actual value generated with _test body_ using `assert_eq!`.
/// _Test cases_ that don't provide _expected result_ should contain custom assertions inside _test body_.
///
/// # Examples
///
/// - Without result and name
///
/// ```rust
/// # use test_case::test_case;
/// #[test_case(5)]
/// #[test_case(10)]
/// fn is_positive(x: i8) {
///     assert!(x > 0)
/// }
/// ```
///
/// - With name, without result
///
/// ```rust
/// # use test_case::test_case;
/// #[test_case(1   ; "little number")]
/// #[test_case(100 ; "big number")]
/// #[test_case(5)] // some tests may use default name generated from arguments list
/// fn is_positive(x: i8) {
///     assert!(x > 0)
/// }
/// ```
///
/// - With result, without name
///
/// ```rust
/// # use test_case::test_case;
/// #[test_case(1,   2 =>  3)]
/// #[test_case(-1, -2 => -3)]
/// fn addition(x: i8, y: i8) -> i8 {
///     x + y
/// }
/// ```
///
/// - With result and name
///
/// ```rust
/// # use test_case::test_case;
/// #[test_case(1,   2 =>  3 ; "both numbers possitive")]
/// #[test_case(-1, -2 => -3 ; "both numbers negative")]
/// fn addition(x: i8, y: i8) -> i8 {
///     x + y
/// }
/// ```
#[proc_macro_attribute]
pub fn test_case(args: TokenStream, input: TokenStream) -> TokenStream {
    let test_case = parse_macro_input!(args as TestCase);
    let mut item = parse_macro_input!(input as ItemFn);

    let mut test_cases = vec![test_case];
    let mut attrs_to_remove = vec![];
    for (idx, attr) in item.attrs.iter().enumerate() {
        if attr.path == parse_quote!(test_case) {
            let test_case = match attr.parse_args::<TestCase>() {
                Ok(test_case) => test_case,
                Err(err) =>
                    return syn::Error::new(attr.span(), format!("cannot parse test_case arguments: {:?}", err)).to_compile_error().into(),
            };
            test_cases.push(test_case);
            attrs_to_remove.push(idx);
        }
    }

    for i in attrs_to_remove.into_iter().rev() {
        item.attrs.swap_remove(i);
    }

    render_test_cases(&test_cases, item)
}

fn render_test_cases(test_cases: &[TestCase], item: ItemFn) -> TokenStream {
    let mut rendered_test_cases = vec![];

    for test_case in test_cases {
        rendered_test_cases.push(test_case.render(item.clone()));
    }

    let mod_name = item.sig.ident.clone();

    let output = quote! {
        mod #mod_name {
            #[allow(unused_imports)]
            use super::*;

            #[allow(unused_attributes)]
            #item

            #(#rendered_test_cases)*
        }
    };

    output.into()
}
