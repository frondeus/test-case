use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Token};

mod kw {
    syn::custom_keyword!(assert_eq);
    syn::custom_keyword!(assert);
}

#[derive(PartialEq, Debug)]
enum AssertOverrideKind {
    AssertEq,
    Assert,
}

impl Parse for AssertOverrideKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::assert_eq) {
            let _ = input.parse::<kw::assert_eq>()?;
            Ok(AssertOverrideKind::AssertEq)
        } else if lookahead.peek(kw::assert) {
            let _ = input.parse::<kw::assert>()?;
            Ok(AssertOverrideKind::Assert)
        } else {
            Err(lookahead.error())
        }
    }
}

struct AssertOverride {
    override_kind: AssertOverrideKind,
    path: syn::Path,
}

impl Parse for AssertOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let override_kind = input.parse::<AssertOverrideKind>()?;
        let _ = input.parse::<Token![:]>()?;
        let path = input.parse::<syn::Path>()?;
        Ok(Self {
            override_kind,
            path,
        })
    }
}

pub struct AssertOverrides {
    overrides: Vec<AssertOverride>,
}

impl Parse for AssertOverrides {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let overrides = Punctuated::<AssertOverride, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            overrides: overrides.into_iter().collect(),
        })
    }
}

pub fn assert_override_impl(items: AssertOverrides) -> TokenStream2 {
    let mut assert_eq = None;
    let mut assert = None;

    for item in items.overrides {
        match item.override_kind {
            AssertOverrideKind::AssertEq => assert_eq = Some(item.path),
            AssertOverrideKind::Assert => assert = Some(item.path),
        }
    }

    let assert_eq = assert_eq.unwrap_or_else(|| parse_quote!(std::assert_eq));
    let assert = assert.unwrap_or_else(|| parse_quote!(std::assert));

    quote! {
        mod __test_case_assert_override {
            use super::*;

            #[allow(unused_imports)]
            pub use #assert_eq as assert_eq;

            #[allow(unused_imports)]
            pub use #assert as assert;
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn can_parse_assert_override() {
        let input = quote! {
            assert_eq: my_assert_eq, assert: my_assert,
        };
        let items = syn::parse2::<super::AssertOverrides>(input).unwrap();
        assert_eq!(items.overrides.len(), 2);
        assert_eq!(
            items.overrides[0].override_kind,
            super::AssertOverrideKind::AssertEq
        );
        assert_eq!(items.overrides[0].path, syn::parse_quote!(my_assert_eq));
        assert_eq!(
            items.overrides[1].override_kind,
            super::AssertOverrideKind::Assert
        );
        assert_eq!(items.overrides[1].path, syn::parse_quote!(my_assert));
    }
}
