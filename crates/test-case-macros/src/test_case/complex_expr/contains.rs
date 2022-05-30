use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(contains);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Contains {
    pub expected_element: Box<Expr>,
}

impl Display for Contains {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "contains {}", fmt_syn(&self.expected_element))
    }
}

impl Contains {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_element = &self.expected_element;
        quote! { _result.iter().find(|i| i.eq(&&#expected_element)).is_some() }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Contains>> {
        if lookahead.peek(kw::contains) {
            let _: kw::contains = input.parse()?;
            Ok(Some(Contains {
                expected_element: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}
