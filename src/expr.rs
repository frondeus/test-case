use crate::complex_expr::ComplexTestCase;
use crate::modifier::{parse_kws, Modifier};
use crate::utils::fmt_syn;
use crate::TokenStream2;
use quote::ToTokens;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute, Expr, Pat, Token};

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
    // test_case(a, b, c => with |v: T| assert!(v.is_nan()))
    With(Expr),
    // test_case(a, b, c => using assert_nan)
    UseFn(Expr),
    // test_case(a, b, c => is close to 4 precision 0.1)
    Complex(ComplexTestCase),
}

impl Parse for TestCaseExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token: Token![=>] = input.parse()?;
        let extra_keywords = parse_kws(input);

        if input.parse::<kw::matches>().is_ok() {
            parse_with_keyword::<_, _>(input, token, extra_keywords, TestCaseResult::Matching)
        } else if input.parse::<kw::it>().is_ok() || input.parse::<kw::is>().is_ok() {
            parse_with_keyword::<_, _>(input, token, extra_keywords, TestCaseResult::Complex)
        } else if input.parse::<kw::using>().is_ok() {
            parse_with_keyword::<_, _>(input, token, extra_keywords, TestCaseResult::UseFn)
        } else if input.parse::<kw::with>().is_ok() {
            parse_with_keyword::<_, _>(input, token, extra_keywords, TestCaseResult::With)
        } else if input.parse::<kw::panics>().is_ok() {
            parse_with_keyword_ok::<_, _>(input, token, extra_keywords, TestCaseResult::Panicking)
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
            TestCaseResult::Panicking(expr) => write!(
                f,
                "panicking {:?}",
                expr.as_ref().map(|inner| fmt_syn(&inner))
            ),
            TestCaseResult::With(expr) => write!(f, "with {}", fmt_syn(expr)),
            TestCaseResult::UseFn(expr) => write!(f, "use {}", fmt_syn(expr)),
            TestCaseResult::Complex(complex) => write!(f, "complex {}", complex),
        }
    }
}

impl TestCaseExpression {
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
            TestCaseResult::With(expr) => parse_quote! { let fun = #expr; fun(_result) },
            TestCaseResult::UseFn(path) => parse_quote! { #path(_result) },
            TestCaseResult::Complex(complex) => complex.assertion(),
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

fn parse_with_keyword<Inner, Mapping>(
    input: ParseStream,
    token: Token![=>],
    extra_keywords: HashSet<Modifier>,
    mapping: Mapping,
) -> syn::Result<TestCaseExpression>
where
    Mapping: FnOnce(Inner) -> TestCaseResult,
    Inner: Parse,
{
    Ok(TestCaseExpression {
        _token: token,
        extra_keywords,
        result: mapping(input.parse()?),
    })
}

fn parse_with_keyword_ok<Inner, Mapping>(
    input: ParseStream,
    token: Token![=>],
    extra_keywords: HashSet<Modifier>,
    mapping: Mapping,
) -> syn::Result<TestCaseExpression>
where
    Mapping: FnOnce(Option<Inner>) -> TestCaseResult,
    Inner: Parse,
{
    Ok(TestCaseExpression {
        _token: token,
        extra_keywords,
        result: mapping(input.parse::<Inner>().ok()),
    })
}
