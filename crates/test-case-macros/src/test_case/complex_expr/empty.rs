use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};

mod kw {
    syn::custom_keyword!(empty);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "empty")
    }
}

impl Empty {
    pub fn boolean_check(&self) -> TokenStream {
        quote! {
            _result.is_empty()
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Empty>> {
        if lookahead.peek(kw::empty) {
            let _ = input.parse::<kw::empty>()?;
            Ok(Some(Empty))
        } else {
            Ok(None)
        }
    }
}
