extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};

use for_each::TestCaseForEach;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_quote;
use syn::spanned::Spanned;
use test_case::TestCase;

mod for_each;
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
    let legal_test_case_names = [
        parse_quote!(test_case),
        parse_quote!(test_case::test_case),
        parse_quote!(test_case::case),
        parse_quote!(case),
    ];

    // Gather all other test cases on input function
    for (idx, attr) in item.attrs.iter().enumerate() {
        if legal_test_case_names.contains(&attr.path) {
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

/// Generates tests for every entry in provided expression.
///
/// Syntax:
/// ```rust
/// #[test_case::for_each($expr)]
/// ```
/// where `expr` can be one of the following:
/// - `file in $path`
/// - `spec in $path`
/// - `variant in $type`
#[proc_macro_attribute]
#[proc_macro_error::proc_macro_error]
pub fn test_case_for_each_file(args: TokenStream, input: TokenStream) -> TokenStream {
    let for_each = parse_macro_input!(args as TestCaseForEach);
    let item = parse_macro_input!(input as ItemFn);

    let test_cases = for_each.into_test_cases();
    render_test_cases(&test_cases, item)
}

#[allow(unused_mut)]
fn render_test_cases(test_cases: &[TestCase], mut item: ItemFn) -> TokenStream {
    let mut rendered_test_cases = vec![];

    for test_case in test_cases {
        let case = test_case.render(item.clone());
        rendered_test_cases.push(case);
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
