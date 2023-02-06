use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, parse_quote, Attribute, LitStr};

mod kw {
    syn::custom_keyword!(inconclusive);
    syn::custom_keyword!(ignore);
}

#[derive(PartialEq, Eq, Hash)]
pub enum Modifier {
    Inconclusive,
    InconclusiveWithReason(LitStr),
}

impl Debug for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Inconclusive | Modifier::InconclusiveWithReason(_) => {
                write!(f, "inconclusive")
            }
        }
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::inconclusive) {
            let _: kw::inconclusive = input.parse()?;
            Self::parse_inconclusive(input)
        } else if input.peek(kw::ignore) {
            let _: kw::ignore = input.parse()?;
            Self::parse_inconclusive(input)
        } else {
            Err(syn::Error::new(input.span(), "unknown modifier keyword"))
        }
    }
}

impl Modifier {
    pub fn parse_inconclusive(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let content;
            let _: Bracket = bracketed!(content in input);
            let reason: LitStr = content.parse()?;
            Ok(Self::InconclusiveWithReason(reason))
        } else {
            Ok(Self::Inconclusive)
        }
    }

    pub fn attribute(&self) -> Attribute {
        match self {
            Modifier::Inconclusive => parse_quote! { #[ignore] },
            Modifier::InconclusiveWithReason(r) => parse_quote! { #[ignore = #r] },
        }
    }
}

pub fn parse_kws(input: ParseStream) -> HashSet<Modifier> {
    let mut kws = HashSet::new();
    while let Ok(kw) = Modifier::parse(input) {
        kws.insert(kw);
    }
    kws
}
