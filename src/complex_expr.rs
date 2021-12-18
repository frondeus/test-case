use crate::utils::fmt_syn;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Expr};

mod kw {
    syn::custom_keyword!(eq);
    syn::custom_keyword!(equal_to);
    syn::custom_keyword!(lt);
    syn::custom_keyword!(less_than);
    syn::custom_keyword!(gt);
    syn::custom_keyword!(greater_than);
    syn::custom_keyword!(leq);
    syn::custom_keyword!(less_or_equal_than);
    syn::custom_keyword!(geq);
    syn::custom_keyword!(greater_or_equal_than);
    syn::custom_keyword!(almost);
    syn::custom_keyword!(almost_equal_to);
    syn::custom_keyword!(precision);
    syn::custom_keyword!(existing_path);
    syn::custom_keyword!(directory);
    syn::custom_keyword!(dir);
    syn::custom_keyword!(file);
    syn::custom_keyword!(contains);
    syn::custom_keyword!(contains_in_order);
}

#[derive(Debug)]
pub enum OrderingToken {
    Eq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug)]
pub enum PathToken {
    Any,
    Dir,
    File,
}

#[derive(Debug)]
pub enum ComplexTestCase {
    // Not(Box<ComplexTestCase>),
    // And(Vec<ComplexTestCase>),
    // Or(Vec<ComplexTestCase>),
    Ord(OrderingToken, Box<Expr>),
    AlmostEqual(Box<Expr>, Box<Expr>),
    Path(PathToken),
    Contains(Box<Expr>),
    ContainsInOrder(Box<Expr>),
}

impl Parse for ComplexTestCase {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.parse::<kw::eq>().is_ok() || input.parse::<kw::equal_to>().is_ok() {
            Ok(ComplexTestCase::Ord(OrderingToken::Eq, input.parse()?))
        } else if input.parse::<kw::lt>().is_ok() || input.parse::<kw::less_than>().is_ok() {
            Ok(ComplexTestCase::Ord(OrderingToken::Lt, input.parse()?))
        } else if input.parse::<kw::gt>().is_ok() || input.parse::<kw::greater_than>().is_ok() {
            Ok(ComplexTestCase::Ord(OrderingToken::Gt, input.parse()?))
        } else if input.parse::<kw::leq>().is_ok()
            || input.parse::<kw::less_or_equal_than>().is_ok()
        {
            Ok(ComplexTestCase::Ord(OrderingToken::Leq, input.parse()?))
        } else if input.parse::<kw::geq>().is_ok()
            || input.parse::<kw::greater_or_equal_than>().is_ok()
        {
            Ok(ComplexTestCase::Ord(OrderingToken::Geq, input.parse()?))
        } else if input.parse::<kw::almost>().is_ok()
            || input.parse::<kw::almost_equal_to>().is_ok()
        {
            let target = input.parse()?;
            let _ = input.parse::<kw::precision>()?;
            let precision = input.parse()?;
            Ok(ComplexTestCase::AlmostEqual(target, precision))
        } else if input.parse::<kw::existing_path>().is_ok() {
            Ok(ComplexTestCase::Path(PathToken::Any))
        } else if input.parse::<kw::directory>().is_ok() || input.parse::<kw::dir>().is_ok() {
            Ok(ComplexTestCase::Path(PathToken::Dir))
        } else if input.parse::<kw::file>().is_ok() {
            Ok(ComplexTestCase::Path(PathToken::File))
        } else if input.parse::<kw::contains>().is_ok() {
            Ok(ComplexTestCase::Contains(input.parse()?))
        } else if input.parse::<kw::contains_in_order>().is_ok() {
            Ok(ComplexTestCase::ContainsInOrder(input.parse()?))
        } else {
            proc_macro_error::abort!(input.span(), "cannot parse complex expression")
        }
    }
}

impl Display for OrderingToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderingToken::Eq => f.write_str("eq"),
            OrderingToken::Lt => f.write_str("lt"),
            OrderingToken::Gt => f.write_str("gt"),
            OrderingToken::Leq => f.write_str("leq"),
            OrderingToken::Geq => f.write_str("geq"),
        }
    }
}

impl Display for PathToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathToken::Any => f.write_str("path"),
            PathToken::Dir => f.write_str("dir"),
            PathToken::File => f.write_str("file"),
        }
    }
}

impl Display for ComplexTestCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexTestCase::Ord(ord, expr) => write!(f, "{} {}", ord, fmt_syn(expr)),
            ComplexTestCase::AlmostEqual(target, precision) => write!(f, "almost {} p {}", fmt_syn(target), fmt_syn(precision)),
            ComplexTestCase::Path(token) => write!(f, "{}", token),
            ComplexTestCase::Contains(elem) => write!(f, "{}", fmt_syn(elem)),
            ComplexTestCase::ContainsInOrder(elems) => write!(f, "{}", fmt_syn(elems)),
        }
    }
}

impl ComplexTestCase {
    pub fn assertion(&self) -> TokenStream {
        match self {
            ComplexTestCase::Ord(ord, expr) => ord_assertion(ord, expr),
            ComplexTestCase::AlmostEqual(expr, precision) => {
                almost_equal_assertion(expr, precision)
            }
            ComplexTestCase::Path(kind) => path_assertion(kind),
            ComplexTestCase::Contains(element) => contains_assertion(element),
            ComplexTestCase::ContainsInOrder(elements) => contains_in_order_assertion(elements),
        }
    }
}

fn contains_in_order_assertion(elements: &Expr) -> TokenStream {
    parse_quote! {
        let mut _tc_outcome = false;
        for i in 0..=_result.len() - #elements.len() {
            if #elements == _result[i..i+#elements.len()] {
                _tc_outcome = true;
            }
        }
        assert!(_tc_outcome, "contains_in_order failed")
    }
}

fn contains_assertion(element: &Expr) -> TokenStream {
    parse_quote! { assert!(_result.iter().find(|i| i.eq(&&#element)).is_some()) }
}

fn path_assertion(token: &PathToken) -> TokenStream {
    match token {
        PathToken::Any => parse_quote! { assert!(std::path::Path::new(&_result).exists()) },
        PathToken::Dir => parse_quote! { assert!(std::path::Path::new(&_result).is_dir()) },
        PathToken::File => parse_quote! { assert!(std::path::Path::new(&_result).is_file()) },
    }
}

fn almost_equal_assertion(expr: &Expr, precision: &Expr) -> TokenStream {
    quote! { assert!((_result - #expr).abs() < #precision) }
}

fn ord_assertion(ord: &OrderingToken, expr: &Expr) -> TokenStream {
    let ts: TokenStream = match ord {
        OrderingToken::Eq => parse_quote! { == },
        OrderingToken::Lt => parse_quote! { < },
        OrderingToken::Gt => parse_quote! { > },
        OrderingToken::Leq => parse_quote! { <= },
        OrderingToken::Geq => parse_quote! { >= },
    };

    quote! {
        assert!(_result #ts #expr)
    }
}
