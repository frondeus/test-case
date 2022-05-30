use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::Expr;

mod kw {
    syn::custom_keyword!(contains_in_order);
}

#[derive(Debug, Eq, PartialEq)]
pub struct ContainsInOrder {
    pub expected_slice: Box<Expr>,
}

impl Display for ContainsInOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "contains_in_order {}", fmt_syn(&self.expected_slice))
    }
}

impl ContainsInOrder {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_slice = &self.expected_slice;
        quote! {
            {
                let mut _tc_outcome = false;
                for i in 0..=_result.len() - #expected_slice.len() {
                    if #expected_slice == _result[i..i+#expected_slice.len()] {
                        _tc_outcome = true;
                    }
                }
                _tc_outcome
            }
        }
    }

    pub fn parse(
        input: ParseStream,
        lookahead: &Lookahead1,
    ) -> syn::Result<Option<ContainsInOrder>> {
        if lookahead.peek(kw::contains_in_order) {
            let _: kw::contains_in_order = input.parse()?;
            Ok(Some(ContainsInOrder {
                expected_slice: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}
