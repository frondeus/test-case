use crate::TestCase;
use syn::parse::{Parse, ParseStream};

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
        todo!()
    }
}
