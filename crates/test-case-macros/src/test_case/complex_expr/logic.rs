use crate::test_case::complex_expr::ComplexTestCase;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::fmt::{Display, Formatter};
use syn::parse::{Lookahead1, Parse, ParseStream};
use syn::parse_quote;

mod kw {
    syn::custom_keyword!(not);
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
}

#[derive(Debug, PartialEq)]
pub struct Not {
    pub inner: Box<ComplexTestCase>,
}

#[derive(Debug, PartialEq)]
pub struct And {
    pub inner: Vec<ComplexTestCase>,
}

#[derive(Debug, PartialEq)]
pub struct Or {
    pub inner: Vec<ComplexTestCase>,
}

impl Display for Not {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "not {}", self.inner)
    }
}

impl Display for And {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner[0])?;
        for case in self.inner[1..].iter() {
            write!(f, " and {}", case)?;
        }
        Ok(())
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner[0])?;
        for case in self.inner[1..].iter() {
            write!(f, " or {}", case)?;
        }
        Ok(())
    }
}

impl Not {
    pub fn boolean_check(&self) -> TokenStream {
        match self.inner.as_ref() {
            ComplexTestCase::Not(_) => {
                proc_macro_error::abort_call_site!(
                    "multiple negations on single item are forbidden"
                )
            }
            ComplexTestCase::And(and) => negate(and.boolean_check()),
            ComplexTestCase::Or(or) => negate(or.boolean_check()),
            ComplexTestCase::CompareTo(ord) => negate(ord.boolean_check()),
            ComplexTestCase::AlmostEqual(almost_equal) => negate(almost_equal.boolean_check()),
            ComplexTestCase::Path(path) => negate(path.boolean_check()),
            ComplexTestCase::Contains(contains) => negate(contains.boolean_check()),
            ComplexTestCase::ContainsInOrder(contains_in_order) => {
                negate(contains_in_order.boolean_check())
            }
            ComplexTestCase::Len(len) => negate(len.boolean_check()),
            ComplexTestCase::Count(count) => negate(count.boolean_check()),
            ComplexTestCase::Empty(empty) => negate(empty.boolean_check()),
            #[cfg(feature = "with-regex")]
            ComplexTestCase::Regex(regex) => negate(regex.boolean_check()),
        }
    }

    pub fn parse(input: ParseStream, lookahead: &Lookahead1) -> syn::Result<Option<Not>> {
        if lookahead.peek(kw::not) {
            let _ = input.parse::<kw::not>()?;
            let inner = ComplexTestCase::parse(input)?;
            Ok(Some(Not {
                inner: Box::new(inner),
            }))
        } else {
            Ok(None)
        }
    }
}

impl And {
    pub fn boolean_check(&self) -> TokenStream {
        let ts = self.inner[0].boolean_check();
        let mut ts: TokenStream = parse_quote! { #ts };

        for case in self.inner.iter().skip(1) {
            let case = case.boolean_check();
            let case: TokenStream = parse_quote! { && #case };
            ts.append_all(case);
        }

        ts
    }

    pub fn parse(input: ParseStream) -> syn::Result<Option<And>> {
        if input.peek(kw::and) {
            Ok(Some(And {
                inner: parse_kw_repeat::<kw::and>(input)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn with_first(mut self, first: ComplexTestCase) -> Self {
        self.inner.insert(0, first);
        self
    }
}

impl Or {
    pub fn boolean_check(&self) -> TokenStream {
        let ts = self.inner[0].boolean_check();
        let mut ts: TokenStream = parse_quote! { #ts };

        for case in self.inner.iter().skip(1) {
            let case = case.boolean_check();
            let case: TokenStream = parse_quote! { || #case };
            ts.append_all(case);
        }

        ts
    }

    pub fn parse(input: ParseStream) -> syn::Result<Option<Or>> {
        if input.peek(kw::or) {
            Ok(Some(Or {
                inner: parse_kw_repeat::<kw::or>(input)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn with_first(mut self, first: ComplexTestCase) -> Self {
        self.inner.insert(0, first);
        self
    }
}

fn negate(tokens: TokenStream) -> TokenStream {
    quote! {
        !{#tokens}
    }
}

fn parse_kw_repeat<KW: Parse>(input: ParseStream) -> syn::Result<Vec<ComplexTestCase>> {
    let mut acc = Vec::new();
    while input.parse::<KW>().is_ok() {
        acc.push(ComplexTestCase::parse_single_item(input)?);
    }
    Ok(acc)
}
