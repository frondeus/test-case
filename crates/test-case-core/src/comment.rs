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

#[cfg(test)]
mod tests {
    use crate::comment::TestCaseComment;
    use proc_macro2::TokenStream;
    use syn::parse_quote;

    #[test]
    fn parses_token_stream() {
        let input: TokenStream = parse_quote! { ; "abcdef" };
        let actual: TestCaseComment = syn::parse2(input).unwrap();
        assert_eq!(actual.comment.value(), "abcdef")
    }
}
