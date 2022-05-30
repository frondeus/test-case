use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(matching_regex);
    syn::custom_keyword!(matches_regex);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Regex {
    pub expected_regex: Box<Expr>,
}

impl Display for Regex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "regex {}", fmt_syn(&self.expected_regex))
    }
}

impl Regex {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_regex = &self.expected_regex;
        quote! {
            {
                let re = ::test_case::Regex::new(#expected_regex).expect("Regex::new");
                re.is_match(_result)
            }
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Regex>> {
        if lookahead.peek(kw::matching_regex) {
            input.parse::<kw::matching_regex>()?;
            let expected_regex = input.parse::<Expr>()?;
            Ok(Some(Regex {
                expected_regex: Box::new(expected_regex),
            }))
        } else if lookahead.peek(kw::matches_regex) {
            input.parse::<kw::matches_regex>()?;
            let expected_regex = input.parse::<Expr>()?;
            Ok(Some(Regex {
                expected_regex: Box::new(expected_regex),
            }))
        } else {
            Ok(None)
        }
    }
}
