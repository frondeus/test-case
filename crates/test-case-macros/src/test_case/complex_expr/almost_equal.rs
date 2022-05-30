use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, Parse, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(precision);
    syn::custom_keyword!(almost);
    syn::custom_keyword!(almost_equal_to);
}

#[derive(Debug, Eq, PartialEq)]
pub struct AlmostEqual {
    pub expected_value: Box<Expr>,
    pub precision: Box<Expr>,
}

impl Display for AlmostEqual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "almost {} p {}",
            fmt_syn(&self.expected_value),
            fmt_syn(&self.precision)
        )
    }
}

impl AlmostEqual {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_value = &self.expected_value;
        let precision = &self.precision;
        quote! { (_result - #expected_value).abs() < #precision }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<AlmostEqual>> {
        if lookahead.peek(kw::almost) {
            return Ok(Some(AlmostEqual::parse_impl::<kw::almost>(input)?));
        }

        if lookahead.peek(kw::almost_equal_to) {
            return Ok(Some(AlmostEqual::parse_impl::<kw::almost_equal_to>(input)?));
        }

        Ok(None)
    }

    fn parse_impl<KW: Parse>(input: ParseStream) -> syn::Result<AlmostEqual> {
        let _: KW = input.parse()?;
        let target = input.parse()?;
        let _ = input.parse::<kw::precision>()?;
        let precision = input.parse()?;
        Ok(AlmostEqual {
            expected_value: target,
            precision,
        })
    }
}
