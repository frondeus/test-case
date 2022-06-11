use crate::TestCase;
use cfg_if::cfg_if;
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(file);
    syn::custom_keyword!(spec);
    syn::custom_keyword!(variant);
}

pub enum TestCaseForEach {
    #[cfg(feature = "for-each-file")]
    Files {
        path: syn::LitStr,
    },
    #[cfg(feature = "for-each-file")]
    Specs {
        path: syn::LitStr,
    },
    Variants {
        typ: syn::TypePath,
    },
}

impl Parse for TestCaseForEach {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::file) {
            cfg_if! {
                if #[cfg(feature = "for-each-file")] {
                    let _ = input.parse::<kw::file>()?;
                    let _ = input.parse::<syn::Token![in]>()?;
                    let path = input.parse::<syn::LitStr>()?;
                    Ok(TestCaseForEach::Files { path })
                } else {
                    let file_token = input.parse::<kw::file>()?;

                    proc_macro_error::abort!(file_token.span, "Using `file` with for each test_case is experimental; `for-each-file` feature has to be enabled; see github wiki for more details.");
                }
            }
        } else if lookahead.peek(kw::spec) {
            cfg_if! {
                if #[cfg(feature = "for-each-file")] {
                    let _ = input.parse::<kw::spec>()?;
                    let _ = input.parse::<syn::Token![in]>()?;
                    let path = input.parse::<syn::LitStr>()?;
                    Ok(TestCaseForEach::Specs { path })
                } else {
                    let spec_token = input.parse::<kw::spec>()?;

                    proc_macro_error::abort!(spec_token.span, "Using `spec` with for each test_case is experimental; `for-each-file` feature has to be enabled; see github wiki for more details.");
                }
            }
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
            #[cfg(feature = "for-each-file")]
            TestCaseForEach::Files { path } => {
                #[cfg(all(feature = "for-each-file-tracking", nightly))]
                proc_macro::tracked_path::path(path.value());

                let pb = std::path::Path::new(&path.value()).canonicalize().unwrap();
                let mut test_cases = Vec::new();
                for entry in std::fs::read_dir(pb).unwrap() {
                    let entry = entry.unwrap();
                    let meta = entry.metadata().unwrap();
                    let file_name = syn::LitStr::new(entry.path().to_str().unwrap(), path.span());
                    if meta.is_file() {
                        test_cases.push(TestCase {
                            args: syn::parse_quote! { #file_name },
                            expression: None,
                            comment: None,
                        })
                    }
                }

                return test_cases;
            }
            #[cfg(feature = "for-each-file")]
            TestCaseForEach::Specs { .. } => {}
            TestCaseForEach::Variants { .. } => {}
        }

        vec![]
    }
}
