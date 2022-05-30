use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute};

mod kw {
    syn::custom_keyword!(inconclusive);
    syn::custom_keyword!(ignore);
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Modifier {
    Inconclusive,
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::inconclusive) {
            let _: kw::inconclusive = input.parse::<kw::inconclusive>()?;
            Ok(Self::Inconclusive)
        } else if input.peek(kw::ignore) {
            let _: kw::ignore = input.parse()?;
            Ok(Self::Inconclusive)
        } else {
            Err(syn::Error::new(input.span(), "unknown modifier keyword"))
        }
    }
}

impl Modifier {
    pub fn attribute(&self) -> Attribute {
        match self {
            Modifier::Inconclusive => parse_quote! { #[ignore] },
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
