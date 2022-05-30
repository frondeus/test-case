use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, Parse, ParseStream};
use syn::{parse_quote, Expr};

mod kw {
    syn::custom_keyword!(eq);
    syn::custom_keyword!(equal_to);
    syn::custom_keyword!(lt);
    syn::custom_keyword!(less_than);
    syn::custom_keyword!(gt);
    syn::custom_keyword!(greater_than);
    syn::custom_keyword!(leq);
    syn::custom_keyword!(less_or_equal_than);
    syn::custom_keyword!(geq);
    syn::custom_keyword!(greater_or_equal_than);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderingToken {
    Eq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CompareTo {
    pub token: OrderingToken,
    pub expected_value: Box<Expr>,
}

impl Display for OrderingToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderingToken::Eq => f.write_str("eq"),
            OrderingToken::Lt => f.write_str("lt"),
            OrderingToken::Gt => f.write_str("gt"),
            OrderingToken::Leq => f.write_str("leq"),
            OrderingToken::Geq => f.write_str("geq"),
        }
    }
}

impl Display for CompareTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token, fmt_syn(&self.expected_value))
    }
}

impl CompareTo {
    pub fn boolean_check(&self) -> TokenStream {
        let expected_value = &self.expected_value;
        let ts: TokenStream = match self.token {
            OrderingToken::Eq => parse_quote! { == },
            OrderingToken::Lt => parse_quote! { < },
            OrderingToken::Gt => parse_quote! { > },
            OrderingToken::Leq => parse_quote! { <= },
            OrderingToken::Geq => parse_quote! { >= },
        };

        quote! {
            _result #ts #expected_value
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<CompareTo>> {
        if lookahead.peek(kw::eq) {
            return Ok(Some(CompareTo::parse_impl::<kw::eq>(
                input,
                OrderingToken::Eq,
            )?));
        }

        if lookahead.peek(kw::equal_to) {
            return Ok(Some(CompareTo::parse_impl::<kw::equal_to>(
                input,
                OrderingToken::Eq,
            )?));
        }

        if lookahead.peek(kw::lt) {
            return Ok(Some(CompareTo::parse_impl::<kw::lt>(
                input,
                OrderingToken::Lt,
            )?));
        }

        if lookahead.peek(kw::less_than) {
            return Ok(Some(CompareTo::parse_impl::<kw::less_than>(
                input,
                OrderingToken::Lt,
            )?));
        }

        if lookahead.peek(kw::gt) {
            return Ok(Some(CompareTo::parse_impl::<kw::gt>(
                input,
                OrderingToken::Gt,
            )?));
        }

        if lookahead.peek(kw::greater_than) {
            return Ok(Some(CompareTo::parse_impl::<kw::greater_than>(
                input,
                OrderingToken::Gt,
            )?));
        }

        if lookahead.peek(kw::leq) {
            return Ok(Some(CompareTo::parse_impl::<kw::leq>(
                input,
                OrderingToken::Leq,
            )?));
        }

        if lookahead.peek(kw::less_or_equal_than) {
            return Ok(Some(CompareTo::parse_impl::<kw::less_or_equal_than>(
                input,
                OrderingToken::Leq,
            )?));
        }

        if lookahead.peek(kw::geq) {
            return Ok(Some(CompareTo::parse_impl::<kw::geq>(
                input,
                OrderingToken::Geq,
            )?));
        }

        if lookahead.peek(kw::greater_or_equal_than) {
            return Ok(Some(CompareTo::parse_impl::<kw::greater_or_equal_than>(
                input,
                OrderingToken::Geq,
            )?));
        }

        Ok(None)
    }

    fn parse_impl<KW: Parse>(input: ParseStream, token: OrderingToken) -> syn::Result<CompareTo> {
        let _: KW = input.parse()?;
        let input = input.parse()?;
        Ok(CompareTo {
            token,
            expected_value: Box::new(input),
        })
    }
}
