use crate::TokenStream2;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Clone, Debug)]
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

impl ToTokens for TestCaseComment {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self._semicolon.to_tokens(tokens);
        self.comment.to_tokens(tokens);
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
