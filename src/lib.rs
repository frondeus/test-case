//! # Overview
//! `test_case` crate provides procedural macro attribute that generates parametrized test instances.
//!
//! # Getting Started
//!
//! Crate has to be added as a dependency to `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! test-case = "2.0.2"
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
//! # Documentation
//!
//! Most up to date documentation is available in our [wiki](https://github.com/frondeus/test-case/wiki).
//!

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
/// _Test cases_ that don't provide _expected result_ should contain custom assertions within _test body_ or return `Result` similar to `#[test]` macro.
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
        #[allow(unused_attributes)]
        #item

        #[cfg(test)]
        mod #mod_name {
            #[allow(unused_imports)]
            use super::*;

            #(#rendered_test_cases)*
        }
    };

    output.into()
}
