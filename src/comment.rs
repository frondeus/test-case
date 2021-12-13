use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Debug)]
pub struct TestCaseComment {
    _semicolon: Token![;],
    pub comment: LitStr,
}

impl Parse for TestCaseComment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _semicolon: input.parse()?,
            comment: input.parse()?,
        })
    }
}
