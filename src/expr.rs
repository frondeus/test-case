use crate::keyword::{parse_kws, Modifier};
use crate::utils::fmt_syn;
use crate::TokenStream2;
use cfg_if::cfg_if;
use quote::ToTokens;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute, Expr, Pat, Path, Token};

pub mod kw {
    syn::custom_keyword!(matches);
    syn::custom_keyword!(using);
    syn::custom_keyword!(with);
    syn::custom_keyword!(it);
    syn::custom_keyword!(is);
    syn::custom_keyword!(panics);
}

#[derive(Debug)]
pub struct TestCaseExpression {
    _token: Token![=>],
    extra_keywords: HashSet<Modifier>,
    result: TestCaseResult,
}

#[derive(Debug)]
pub enum TestCaseResult {
    // test_case(a, b, c => result)
    Simple(Expr),
    // test_case(a, b, c => matches Ok(_))
    Matching(Pat),
    // test_case(a, b, c => panics "abcd")
    Panicking(Option<Expr>),
    // test_case(a, b, c => is some())
    #[cfg(feature = "hamcrest_assertions")]
    Hamcrest(Expr),
    // test_case(a, b, c => using assert!($.is_nan()))
    Assert(Expr),
    // test_case(a, b, c => using assert_nan)
    UseFn(Path),
}

impl Parse for TestCaseExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token: Token![=>] = input.parse()?;
        let extra_keywords = parse_kws(input);

        if input.peek(kw::matches) {
            Self::parse_with_keyword::<kw::matches, _, _>(
                input,
                token,
                extra_keywords,
                TestCaseResult::Matching,
            )
        } else if input.peek(kw::it) {
            cfg_if! {
                if #[cfg(feature = "hamcrest_assertions")] {
                    Self::parse_with_keyword::<kw::it, _, _>(
                        input,
                        token,
                        extra_keywords,
                        TestCaseResult::Hamcrest,
                    )
                } else {
                    proc_macro_error::abort!(input.span(), "Cannot use \"it\"; `hamcrest_assertions` aren't enabled")
                }
            }
        } else if input.peek(kw::is) {
            cfg_if! {
                if #[cfg(feature = "hamcrest_assertions")] {
                    Self::parse_with_keyword::<kw::is, _, _>(
                        input,
                        token,
                        extra_keywords,
                        TestCaseResult::Hamcrest,
                    )
                } else {
                    proc_macro_error::abort!(input.span(), "Cannot use \"is\"; `hamcrest_assertions` aren't enabled")
                }
            }
        } else if input.peek(kw::using) {
            Self::parse_with_keyword::<kw::using, _, _>(
                input,
                token,
                extra_keywords,
                TestCaseResult::UseFn,
            )
        } else if input.peek(kw::with) {
            Self::parse_with_keyword::<kw::with, _, _>(
                input,
                token,
                extra_keywords,
                TestCaseResult::Assert,
            )
        } else if input.peek(kw::panics) {
            let _: kw::panics = input.parse()?;
            Ok(Self {
                _token: token,
                extra_keywords,
                result: TestCaseResult::Panicking(input.parse().ok()),
            })
        } else {
            Ok(Self {
                _token: token,
                extra_keywords,
                result: TestCaseResult::Simple(input.parse()?),
            })
        }
    }
}

impl Display for TestCaseExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for kw in &self.extra_keywords {
            write!(f, "{:?}", kw)?;
        }
        write!(f, "{}", self.result)
    }
}

impl Display for TestCaseResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCaseResult::Simple(expr) => write!(f, "{}", fmt_syn(expr)),
            TestCaseResult::Matching(expr) => write!(f, "matching {}", fmt_syn(expr)),
            TestCaseResult::Panicking(expr) => {
                write!(
                    f,
                    "panicking {:?}",
                    expr.as_ref().map(|inner| fmt_syn(&inner))
                )
            }
            #[cfg(feature = "hamcrest_assertions")]
            TestCaseResult::Hamcrest(expr) => write!(f, "it is {}", fmt_syn(expr)),
            TestCaseResult::Assert(expr) => write!(f, "use {}", fmt_syn(expr)),
            TestCaseResult::UseFn(path) => write!(f, "use path {}", fmt_syn(path)),
        }
    }
}

impl TestCaseExpression {
    fn parse_with_keyword<Keyword, Inner, Mapping>(
        input: ParseStream,
        token: Token![=>],
        extra_keywords: HashSet<Modifier>,
        mapping: Mapping,
    ) -> syn::Result<TestCaseExpression>
    where
        Mapping: FnOnce(Inner) -> TestCaseResult,
        Keyword: Parse,
        Inner: Parse,
    {
        let _: Keyword = input.parse()?;
        Ok(Self {
            _token: token,
            extra_keywords,
            result: mapping(input.parse()?),
        })
    }

    pub fn assertion(&self) -> TokenStream2 {
        match &self.result {
            TestCaseResult::Simple(expr) => parse_quote! { assert_eq!(#expr, _result) },
            TestCaseResult::Matching(pat) => {
                let pat_str = pat.to_token_stream().to_string();
                parse_quote! {
                    match _result {
                        #pat => (),
                        e => panic!("Expected {} found {:?}", #pat_str, e)
                    }
                }
            }
            TestCaseResult::Panicking(_) => TokenStream2::new(),
            #[cfg(feature = "hamcrest_assertions")]
            TestCaseResult::Hamcrest(expr) => parse_quote! { assert_that!(_result, #expr) },
            TestCaseResult::Assert(expr) => parse_quote! { let fun = #expr; fun(_result) },
            TestCaseResult::UseFn(path) => parse_quote! { #path(_result) },
        }
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        let mut attrs: Vec<Attribute> = self
            .extra_keywords
            .iter()
            .map(|modifier| modifier.attribute())
            .collect();
        if let TestCaseResult::Panicking(opt) = &self.result {
            if let Some(expr) = opt {
                attrs.push(parse_quote! { #[should_panic(expected = #expr)] })
            } else {
                attrs.push(parse_quote! { #[should_panic] })
            }
        }
        attrs
    }
}
