use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(count);
    syn::custom_keyword!(has_count);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Count {
    pub expected_len: Box<Expr>,
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "count {}", fmt_syn(&self.expected_len))
    }
}

impl Count {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_len = &self.expected_len;
        quote! {
            std::iter::IntoIterator::into_iter(_result).count() == #expected_len
        }
    }
    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Count>> {
        if lookahead.peek(kw::count) {
            let _ = input.parse::<kw::count>()?;
            let expected_len = input.parse()?;
            return Ok(Some(Count { expected_len }));
        }

        if lookahead.peek(kw::has_count) {
            let _ = input.parse::<kw::has_count>()?;
            let expected_len = input.parse()?;
            return Ok(Some(Count { expected_len }));
        }

        Ok(None)
    }
}
