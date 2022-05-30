use proc_macro2::TokenStream;
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, ParseStream};
use syn::parse_quote;

mod kw {
    syn::custom_keyword!(existing_path);
    syn::custom_keyword!(directory);
    syn::custom_keyword!(dir);
    syn::custom_keyword!(file);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    pub token: PathToken,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PathToken {
    Any,
    Dir,
    File,
}

impl Display for PathToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathToken::Any => f.write_str("any"),
            PathToken::Dir => f.write_str("dir"),
            PathToken::File => f.write_str("file"),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "path {}", self.token)
    }
}

impl Path {
    pub fn boolean_check(&self) -> TokenStream {
        match self.token {
            PathToken::Any => parse_quote! { std::path::Path::new(&_result).exists() },
            PathToken::Dir => parse_quote! { std::path::Path::new(&_result).is_dir() },
            PathToken::File => parse_quote! { std::path::Path::new(&_result).is_file() },
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Path>> {
        if lookahead.peek(kw::existing_path) {
            let _: kw::existing_path = input.parse()?;
            return Ok(Some(Path {
                token: PathToken::Any,
            }));
        }

        if lookahead.peek(kw::directory) {
            let _: kw::directory = input.parse()?;
            return Ok(Some(Path {
                token: PathToken::Dir,
            }));
        }

        if lookahead.peek(kw::dir) {
            let _: kw::dir = input.parse()?;
            return Ok(Some(Path {
                token: PathToken::Dir,
            }));
        }

        if lookahead.peek(kw::file) {
            let _: kw::file = input.parse()?;
            return Ok(Some(Path {
                token: PathToken::File,
            }));
        }

        Ok(None)
    }
}
