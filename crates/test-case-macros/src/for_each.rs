use crate::TestCase;
use std::fs;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, LitStr};

mod kw {
    syn::custom_keyword!(file);
    syn::custom_keyword!(spec);
    syn::custom_keyword!(variant);
}

pub enum TestCaseForEach {
    Files { path: syn::LitStr },
    Specs { path: syn::LitStr },
    Variants { typ: syn::TypePath },
}

impl Parse for TestCaseForEach {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::file) {
            let _ = input.parse::<kw::file>()?;
            let _ = input.parse::<syn::Token![in]>()?;
            let path = input.parse::<syn::LitStr>()?;
            Ok(TestCaseForEach::Files { path })
        } else if lookahead.peek(kw::spec) {
            let _ = input.parse::<kw::spec>()?;
            let _ = input.parse::<syn::Token![in]>()?;
            let path = input.parse::<syn::LitStr>()?;
            Ok(TestCaseForEach::Specs { path })
        } else if lookahead.peek(kw::variant) {
            let _ = input.parse::<kw::variant>()?;
            let _ = input.parse::<syn::Token![in]>()?;
            let typ = input.parse::<syn::TypePath>()?;
            Ok(TestCaseForEach::Variants { typ })
        } else {
            Err(lookahead.error())
        }
    }
}

impl TestCaseForEach {
    pub fn into_test_cases(self) -> Vec<TestCase> {
        match self {
            TestCaseForEach::Files { path } => {
                let pb = Path::new(&path.value()).canonicalize().unwrap();
                let mut test_cases = Vec::new();
                for entry in fs::read_dir(pb).unwrap() {
                    let entry = entry.unwrap();
                    let meta = entry.metadata().unwrap();
                    let file_name = LitStr::new(entry.path().to_str().unwrap(), path.span());
                    if meta.is_file() {
                        test_cases.push(TestCase {
                            args: parse_quote! { #file_name },
                            expression: None,
                            comment: None,
                        })
                    }
                }

                return test_cases;
            }
            TestCaseForEach::Specs { .. } => {}
            TestCaseForEach::Variants { .. } => {}
        }

        vec![]
    }
}
