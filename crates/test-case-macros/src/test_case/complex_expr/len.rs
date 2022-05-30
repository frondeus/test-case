use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(len);
    syn::custom_keyword!(has_length);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Len {
    pub expected_len: Box<Expr>,
}

impl Display for Len {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "len {}", fmt_syn(&self.expected_len))
    }
}

impl Len {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_len = &self.expected_len;
        quote! {
            _result.len() == #expected_len
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Len>> {
        if lookahead.peek(kw::len) {
            let _ = input.parse::<kw::len>()?;
            return Ok(Some(Len {
                expected_len: Box::new(input.parse()?),
            }));
        }

        if lookahead.peek(kw::has_length) {
            let _ = input.parse::<kw::has_length>()?;
            return Ok(Some(Len {
                expected_len: Box::new(input.parse()?),
            }));
        }

        Ok(None)
    }
}
