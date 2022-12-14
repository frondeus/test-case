extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn};
use test_case::TestCase;

#[cfg(feature = "with-assert-override")]
mod assert_override;
mod comment;
mod complex_expr;
mod expr;
mod modifier;
mod test_case;
mod utils;

#[cfg(feature = "with-assert-override")]
mod assertions {
    pub const ASSERT_OVERRIDE_ASSERT_EQ_PATH: &str =
        "crate::__test_case_assert_override::assert_eq";
    pub const ASSERT_OVERRIDE_ASSERT_PATH: &str = "crate::__test_case_assert_override::assert";
}
#[cfg(not(feature = "with-assert-override"))]
mod assertions {
    pub const ASSERT_OVERRIDE_ASSERT_EQ_PATH: &str = "core::assert_eq";
    pub const ASSERT_OVERRIDE_ASSERT_PATH: &str = "core::assert";
}

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

/// Prepares crate for use with `#[test_case]` macro and custom implementations of `assert_eq` and `assert_ne`.
#[cfg(feature = "with-assert-override")]
#[proc_macro]
pub fn test_case_assert_override(item: TokenStream) -> TokenStream {
    let items = parse_macro_input!(item as crate::assert_override::AssertOverrides);

    assert_override::assert_override_impl(items).into()
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
