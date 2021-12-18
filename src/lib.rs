//! # Overview
//! `test_case` crate provides procedural macro attribute that generates parametrized test instances.
//!
//! # Getting Started
//!
//! Crate has to be added as a dependency to `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-case = "2.0.0-rc1"
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
//! # Advanced use
//!
//! For `#[test_case(body)]` the body is built as follows:
//!
//! `body` := `$arguments ($expected_result)? ($description)?`
//!
//! ## Arguments
//!
//! `arguments` := `$expr(,$expr)*(,)?`
//!
//! Comma separated list of one or more [expressions][1], eg.:
//! ```rust
//! #[test_case(a, b, c,)]
//! #[test_case(())]
//! #[test_case(a_method_that_produces_arg(1, 2, 3), "a string")]
//! ```
//!
//! ## Expected result
//!
//! `expected_result` := `=> ($modifier)* $validator`
//!
//! Optional part that provides assertions to instantiated tests.
//!
//! When using `expected_result` version of `test_case` tested function **must** return a type
//! that can be matched with validator. Each validator description states how to ensure
//! that the type returned by function can be matched.
//!
//! ### Modifiers
//!
//! `modifier` := `ignore | inconclusive`
//!
//! Both `ignore` and `inconclusive` keywords indicate that test case should be skipped. This is equivalent to using
//! `#[ignore]` attribute on normal test. Eg.:
//!
//! ```rust
//! #[test_case(0.0 => ignore 0.0)] // not yet implemented
//! ```
//!
//! ### Validator
//!
//! There are numerous validators provided by `test_case`:
//!
//! `validator` := `$simple|$matching|$panicking|$with|$using`
//!
//! #### Simple
//!
//! `simple` := `$expr`
//!
//! Accepts any [expression][1] that evaluates to function return type and
//! compares it against whatever tested block returns via `assert_eq`. Eg.:
//!
//! ```rust
//! #[test_case("2.0" => 2.0)]
//! fn parses_a_string(arg_in: &str) -> f64 {
//!     body omitted...
//! }
//! ```
//! #### Matching
//!
//! `matching` := `matches $pattern`
//!
//! A [pattern][3] following keyword `matches`.
//! Result of a function is compared to `pattern` via [MatchExpression][2]. Eg.:
//!
//! ```rust
//! #[test_case("2.0" => matches Ok(_))]
//! #[test_case("1.0" => matches Ok(v) if v == 1.0f64)]
//! #[test_case("abc" => matches Err(_))]
//! ```
//!
//! #### Panicking
//!
//! `panicking` := `panics ($expr)?`
//!
//! Indicates that test instance should panic. Works identical to `#[should_panic]` test attribute.
//! Optional expression after the keyword is treated like `expected` in [should_panic][4]. Eg.:
//!
//! ```rust
//! #[test_case(0 => panics "division by zero")]
//! ```
//!
//! #### With
//!
//! `with` := `with $closure`
//!
//! Allows manual assertions of the result of testing function.
//! Closure must indicate argument type and it has to be implicitly convertible from type returned by testing function.
//! Eg.:
//!
//! ```rust
//! #[test_case(2.0 => 0.0)]
//! #[test_case(0.0 => with |i: f64| assert!(i.is_nan()))]
//! fn test_division(i: f64) -> f64 {
//!     0.0 / i
//! }
//! ```
//!
//! #### Using
//!
//! `using` := `using $path`
//!
//! Work similar to `with` attribute, with the difference being that instead of a closure
//! it accepts path to a function that should validate result of the testing function. Eg.:
//!
//! ```rust
//! fn is_power_of_two(input: u64) {
//!     assert!(input.is_power_of_two())
//! }
//!
//! #[test_case(1 => using self::is_power_of_two)]
//! fn some_test(input: u64) -> u64 {
//!     "body omitted..."
//! }
//! ```
//!
//! # Notes about async & additional attributes
//!
//! If `test_case` is used with `async` tests, eg. `#[tokio::test]`, or user wants to pass other attributes to each
//! test instance then additional attributes have to be added past first occurrence of `#[test_case]`. Eg.:
//!
//! ```rust
//! #[test_case(...)]
//! #[tokio::test]
//! #[allow(clippy::non_camel_case_types)]
//! async fn xyz() { }
//! ```
//!
//! [1]: https://doc.rust-lang.org/reference/expressions.html
//! [2]: https://doc.rust-lang.org/reference/expressions/match-expr.html
//! [3]: https://doc.rust-lang.org/reference/patterns.html
//! [4]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_quote;
use syn::spanned::Spanned;
use test_case::TestCase;

mod comment;
mod complex_expr;
mod expr;
mod modifier;
mod test_case;
mod utils;

/// Generates tests for given set of data
///
/// In general, test case consists of four elements:
///
/// 1. _(Required)_ Arguments passed to test body
/// 2. _(Optional)_ Expected result
/// 3. _(Optional)_ Test case description
/// 4. _(Required)_ Test body
///
///  When _expected result_ is provided, it is compared against the actual value generated with _test body_ using `assert_eq!`.
/// _Test cases_ that don't provide _expected result_ should contain custom assertions within _test body_.
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
/// - With description, without result
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
/// - With result, without description
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
#[proc_macro_error::proc_macro_error]
pub fn test_case(args: TokenStream, input: TokenStream) -> TokenStream {
    let test_case = parse_macro_input!(args as TestCase);
    let mut item = parse_macro_input!(input as ItemFn);

    let mut test_cases = vec![test_case];
    let mut attrs_to_remove = vec![];
    for (idx, attr) in item.attrs.iter().enumerate() {
        if attr.path == parse_quote!(test_case) || attr.path == parse_quote!(test_case::test_case) {
            let test_case = match attr.parse_args::<TestCase>() {
                Ok(test_case) => test_case,
                Err(err) => {
                    return syn::Error::new(
                        attr.span(),
                        format!("cannot parse test_case arguments: {}", err),
                    )
                    .to_compile_error()
                    .into()
                }
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

#[allow(unused_mut)]
fn render_test_cases(test_cases: &[TestCase], mut item: ItemFn) -> TokenStream {
    let mut rendered_test_cases = vec![];

    for test_case in test_cases {
        rendered_test_cases.push(test_case.render(item.clone()));
    }

    let mod_name = item.sig.ident.clone();

    // We don't want any external crate to alter main fn code, we are passing attributes to each sub-function anyway
    item.attrs.clear();

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
