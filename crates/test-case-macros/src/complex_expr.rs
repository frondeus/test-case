use crate::utils::fmt_syn;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
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
    syn::custom_keyword!(not);
    syn::custom_keyword!(and);
    syn::custom_keyword!(or);
    syn::custom_keyword!(len);
    syn::custom_keyword!(has_length);
    syn::custom_keyword!(count);
    syn::custom_keyword!(has_count);
    syn::custom_keyword!(empty);
    syn::custom_keyword!(matching_regex);
    syn::custom_keyword!(matches_regex);
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderingToken {
    Eq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathToken {
    Any,
    Dir,
    File,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ord {
    pub token: OrderingToken,
    pub expected_value: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AlmostEqual {
    pub expected_value: Box<Expr>,
    pub precision: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub token: PathToken,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Contains {
    pub expected_element: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ContainsInOrder {
    pub expected_slice: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Len {
    pub expected_len: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Count {
    pub expected_len: Box<Expr>,
}

#[cfg(feature = "with-regex")]
#[derive(Debug, PartialEq, Eq)]
pub struct Regex {
    pub expected_regex: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum ComplexTestCase {
    Not(Box<ComplexTestCase>),
    And(Vec<ComplexTestCase>),
    Or(Vec<ComplexTestCase>),
    Ord(Ord),
    AlmostEqual(AlmostEqual),
    Path(Path),
    Contains(Contains),
    ContainsInOrder(ContainsInOrder),
    Len(Len),
    Count(Count),
    Empty,
    #[cfg(feature = "with-regex")]
    Regex(Regex),
}

impl Parse for ComplexTestCase {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = Self::parse_single_item(input)?;

        Ok(if input.peek(kw::and) {
            ComplexTestCase::And(parse_kw_repeat::<kw::and>(item, input)?)
        } else if input.peek(kw::or) {
            ComplexTestCase::Or(parse_kw_repeat::<kw::or>(item, input)?)
        } else {
            item
        })
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
            ComplexTestCase::Not(not) => write!(f, "not {not}"),
            ComplexTestCase::And(cases) => {
                write!(f, "{}", cases[0])?;
                for case in cases[1..].iter() {
                    write!(f, " and {case}")?;
                }
                Ok(())
            }
            ComplexTestCase::Or(cases) => {
                write!(f, "{}", cases[0])?;
                for case in cases[1..].iter() {
                    write!(f, " or {case}")?;
                }
                Ok(())
            }
            ComplexTestCase::Ord(Ord {
                token,
                expected_value,
            }) => write!(f, "{} {}", token, fmt_syn(expected_value)),
            ComplexTestCase::AlmostEqual(AlmostEqual {
                expected_value,
                precision,
            }) => write!(
                f,
                "almost {} p {}",
                fmt_syn(expected_value),
                fmt_syn(precision)
            ),
            ComplexTestCase::Path(Path { token }) => write!(f, "path {token}"),
            ComplexTestCase::Contains(Contains { expected_element }) => {
                write!(f, "contains {}", fmt_syn(expected_element))
            }
            ComplexTestCase::ContainsInOrder(ContainsInOrder { expected_slice }) => {
                write!(f, "contains in order {}", fmt_syn(expected_slice))
            }
            ComplexTestCase::Len(Len { expected_len }) => {
                write!(f, "len {}", fmt_syn(expected_len))
            }
            ComplexTestCase::Count(Count { expected_len }) => {
                write!(f, "count {}", fmt_syn(expected_len))
            }
            ComplexTestCase::Empty => {
                write!(f, "empty")
            }
            #[cfg(feature = "with-regex")]
            ComplexTestCase::Regex(Regex { expected_regex }) => {
                write!(f, "regex {}", fmt_syn(expected_regex))
            }
        }
    }
}

impl ComplexTestCase {
    pub fn assertion(&self) -> TokenStream {
        let tokens = self.boolean_check();

        quote! { assert!(#tokens) }
    }

    fn boolean_check(&self) -> TokenStream {
        match self {
            ComplexTestCase::Not(not) => not_assertion(not),
            ComplexTestCase::And(cases) => and_assertion(cases),
            ComplexTestCase::Or(cases) => or_assertion(cases),
            ComplexTestCase::Ord(Ord {
                token,
                expected_value,
            }) => ord_assertion(token, expected_value),
            ComplexTestCase::AlmostEqual(AlmostEqual {
                expected_value,
                precision,
            }) => almost_equal_assertion(expected_value, precision),
            ComplexTestCase::Path(Path { token }) => path_assertion(token),
            ComplexTestCase::Contains(Contains { expected_element }) => {
                contains_assertion(expected_element)
            }
            ComplexTestCase::ContainsInOrder(ContainsInOrder { expected_slice }) => {
                contains_in_order_assertion(expected_slice)
            }
            ComplexTestCase::Len(Len { expected_len }) => len_assertion(expected_len),
            ComplexTestCase::Count(Count { expected_len }) => count_assertion(expected_len),
            ComplexTestCase::Empty => empty_assertion(),
            #[cfg(feature = "with-regex")]
            ComplexTestCase::Regex(Regex { expected_regex }) => regex_assertion(expected_regex),
        }
    }

    fn parse_single_item(input: ParseStream) -> syn::Result<ComplexTestCase> {
        Ok(if let Ok(group) = Group::parse(input) {
            syn::parse2(group.stream())?
        } else if input.parse::<kw::eq>().is_ok() || input.parse::<kw::equal_to>().is_ok() {
            ComplexTestCase::Ord(Ord {
                token: OrderingToken::Eq,
                expected_value: input.parse()?,
            })
        } else if input.parse::<kw::lt>().is_ok() || input.parse::<kw::less_than>().is_ok() {
            ComplexTestCase::Ord(Ord {
                token: OrderingToken::Lt,
                expected_value: input.parse()?,
            })
        } else if input.parse::<kw::gt>().is_ok() || input.parse::<kw::greater_than>().is_ok() {
            ComplexTestCase::Ord(Ord {
                token: OrderingToken::Gt,
                expected_value: input.parse()?,
            })
        } else if input.parse::<kw::leq>().is_ok()
            || input.parse::<kw::less_or_equal_than>().is_ok()
        {
            ComplexTestCase::Ord(Ord {
                token: OrderingToken::Leq,
                expected_value: input.parse()?,
            })
        } else if input.parse::<kw::geq>().is_ok()
            || input.parse::<kw::greater_or_equal_than>().is_ok()
        {
            ComplexTestCase::Ord(Ord {
                token: OrderingToken::Geq,
                expected_value: input.parse()?,
            })
        } else if input.parse::<kw::almost>().is_ok()
            || input.parse::<kw::almost_equal_to>().is_ok()
        {
            let target = input.parse()?;
            let _ = input.parse::<kw::precision>()?;
            let precision = input.parse()?;
            ComplexTestCase::AlmostEqual(AlmostEqual {
                expected_value: target,
                precision,
            })
        } else if input.parse::<kw::existing_path>().is_ok() {
            ComplexTestCase::Path(Path {
                token: PathToken::Any,
            })
        } else if input.parse::<kw::directory>().is_ok() || input.parse::<kw::dir>().is_ok() {
            ComplexTestCase::Path(Path {
                token: PathToken::Dir,
            })
        } else if input.parse::<kw::file>().is_ok() {
            ComplexTestCase::Path(Path {
                token: PathToken::File,
            })
        } else if input.parse::<kw::contains>().is_ok() {
            ComplexTestCase::Contains(Contains {
                expected_element: input.parse()?,
            })
        } else if input.parse::<kw::contains_in_order>().is_ok() {
            ComplexTestCase::ContainsInOrder(ContainsInOrder {
                expected_slice: input.parse()?,
            })
        } else if input.parse::<kw::not>().is_ok() {
            ComplexTestCase::Not(Box::new(input.parse()?))
        } else if input.parse::<kw::len>().is_ok() || input.parse::<kw::has_length>().is_ok() {
            ComplexTestCase::Len(Len {
                expected_len: input.parse()?,
            })
        } else if input.parse::<kw::count>().is_ok() || input.parse::<kw::has_count>().is_ok() {
            ComplexTestCase::Count(Count {
                expected_len: input.parse()?,
            })
        } else if input.parse::<kw::empty>().is_ok() {
            ComplexTestCase::Empty
        } else if input.parse::<kw::matching_regex>().is_ok()
            || input.parse::<kw::matches_regex>().is_ok()
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "with-regex")] {
                    ComplexTestCase::Regex(Regex {
                        expected_regex: input.parse()?,
                    })
                } else {
                    proc_macro_error::abort!(input.span(), "'with-regex' feature is required to use 'matches-regex' keyword");
                }
            }
        } else {
            proc_macro_error::abort!(input.span(), "cannot parse complex expression")
        })
    }
}

fn and_assertion(cases: &[ComplexTestCase]) -> TokenStream {
    let ts = cases[0].boolean_check();
    let mut ts: TokenStream = parse_quote! { #ts };

    for case in cases.iter().skip(1) {
        let case = case.boolean_check();
        let case: TokenStream = parse_quote! { && #case };
        ts.append_all(case);
    }

    ts
}

fn or_assertion(cases: &[ComplexTestCase]) -> TokenStream {
    let ts = cases[0].boolean_check();
    let mut ts: TokenStream = parse_quote! { #ts };

    for case in cases.iter().skip(1) {
        let case = case.boolean_check();
        let case: TokenStream = parse_quote! { || #case };
        ts.append_all(case);
    }

    ts
}

fn parse_kw_repeat<Keyword: Parse>(
    first: ComplexTestCase,
    input: ParseStream,
) -> syn::Result<Vec<ComplexTestCase>> {
    let mut acc = vec![first];
    while input.parse::<Keyword>().is_ok() {
        acc.push(ComplexTestCase::parse_single_item(input)?);
    }
    Ok(acc)
}

fn negate(tokens: TokenStream) -> TokenStream {
    quote! {
        !{#tokens}
    }
}

fn contains_in_order_assertion(expected_slice: &Expr) -> TokenStream {
    parse_quote! {
        {
            let mut _tc_outcome = false;
            for i in 0..=_result.len() - #expected_slice.len() {
                if #expected_slice == _result[i..i+#expected_slice.len()] {
                    _tc_outcome = true;
                }
            }
            _tc_outcome
        }
    }
}

fn contains_assertion(expected_element: &Expr) -> TokenStream {
    parse_quote! { _result.iter().find(|i| i.eq(&&#expected_element)).is_some() }
}

fn path_assertion(token: &PathToken) -> TokenStream {
    match token {
        PathToken::Any => parse_quote! { std::path::Path::new(&_result).exists() },
        PathToken::Dir => parse_quote! { std::path::Path::new(&_result).is_dir() },
        PathToken::File => parse_quote! { std::path::Path::new(&_result).is_file() },
    }
}

fn almost_equal_assertion(expected_value: &Expr, precision: &Expr) -> TokenStream {
    quote! { (_result - #expected_value).abs() < #precision }
}

fn ord_assertion(token: &OrderingToken, expected_value: &Expr) -> TokenStream {
    let ts: TokenStream = match token {
        OrderingToken::Eq => parse_quote! { == },
        OrderingToken::Lt => parse_quote! { < },
        OrderingToken::Gt => parse_quote! { > },
        OrderingToken::Leq => parse_quote! { <= },
        OrderingToken::Geq => parse_quote! { >= },
    };

    quote! {
        _result #ts #expected_value
    }
}

fn len_assertion(expected_len: &Expr) -> TokenStream {
    quote! {
        _result.len() == #expected_len
    }
}

fn count_assertion(expected_len: &Expr) -> TokenStream {
    quote! {
        std::iter::IntoIterator::into_iter(_result).count() == #expected_len
    }
}

fn empty_assertion() -> TokenStream {
    quote! {
        _result.is_empty()
    }
}

#[cfg(feature = "with-regex")]
fn regex_assertion(expected_regex: &Expr) -> TokenStream {
    quote! {
        {
            let re = ::test_case::Regex::new(#expected_regex).expect("Regex::new");
            re.is_match(_result)
        }
    }
}

fn not_assertion(not: &ComplexTestCase) -> TokenStream {
    match not {
        ComplexTestCase::Not(_) => {
            proc_macro_error::abort_call_site!("multiple negations on single item are forbidden")
        }
        ComplexTestCase::And(cases) => negate(and_assertion(cases)),
        ComplexTestCase::Or(cases) => negate(or_assertion(cases)),
        ComplexTestCase::Ord(Ord {
            token,
            expected_value,
        }) => negate(ord_assertion(token, expected_value)),
        ComplexTestCase::AlmostEqual(AlmostEqual {
            expected_value,
            precision,
        }) => negate(almost_equal_assertion(expected_value, precision)),
        ComplexTestCase::Path(Path { token }) => negate(path_assertion(token)),
        ComplexTestCase::Contains(Contains { expected_element }) => {
            negate(contains_assertion(expected_element))
        }
        ComplexTestCase::ContainsInOrder(ContainsInOrder { expected_slice }) => {
            negate(contains_in_order_assertion(expected_slice))
        }
        ComplexTestCase::Len(Len { expected_len }) => negate(len_assertion(expected_len)),
        ComplexTestCase::Count(Count { expected_len }) => negate(count_assertion(expected_len)),
        ComplexTestCase::Empty => negate(empty_assertion()),
        #[cfg(feature = "with-regex")]
        ComplexTestCase::Regex(Regex { expected_regex }) => negate(regex_assertion(expected_regex)),
    }
}

#[cfg(test)]
mod tests {
    use crate::complex_expr::{
        AlmostEqual, ComplexTestCase, Contains, ContainsInOrder, Count, Len, OrderingToken, Path,
        PathToken,
    };
    use syn::{parse_quote, LitFloat, LitInt, LitStr};

    macro_rules! assert_ord {
        ($actual:tt, $token:path, $value:tt) => {
            if let ComplexTestCase::Ord(ord) = $actual {
                assert_eq!(ord.token, $token);
                let lit = ord.expected_value;
                let actual_expr: LitFloat = parse_quote! { #lit };
                assert_eq!(actual_expr.base10_parse::<f64>().unwrap(), $value)
            } else {
                panic!("invalid enum variant")
            }
        };
    }

    macro_rules! assert_almost_eq {
        ($actual:tt, $value:tt, $precision:tt) => {
            if let ComplexTestCase::AlmostEqual(AlmostEqual {
                expected_value,
                precision,
            }) = $actual
            {
                let expected_value: LitFloat = parse_quote! { #expected_value };
                assert_eq!(expected_value.base10_parse::<f64>().unwrap(), $value);
                let precision: LitFloat = parse_quote! { #precision };
                assert_eq!(precision.base10_parse::<f64>().unwrap(), $precision);
            } else {
                panic!("invalid enum variant")
            }
        };
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn parses_ord_token_stream() {
        let actual: ComplexTestCase = parse_quote! { equal_to 1.0 };
        assert_ord!(actual, OrderingToken::Eq, 1.0);
        let actual: ComplexTestCase = parse_quote! { eq 0.0 };
        assert_ord!(actual, OrderingToken::Eq, 0.0);

        let actual: ComplexTestCase = parse_quote! { less_than 2.0 };
        assert_ord!(actual, OrderingToken::Lt, 2.0);
        let actual: ComplexTestCase = parse_quote! { lt 2.0 };
        assert_ord!(actual, OrderingToken::Lt, 2.0);

        let actual: ComplexTestCase = parse_quote! { greater_than 2.0 };
        assert_ord!(actual, OrderingToken::Gt, 2.0);
        let actual: ComplexTestCase = parse_quote! { gt 2.0 };
        assert_ord!(actual, OrderingToken::Gt, 2.0);

        let actual: ComplexTestCase = parse_quote! { less_or_equal_than 2.0 };
        assert_ord!(actual, OrderingToken::Leq, 2.0);
        let actual: ComplexTestCase = parse_quote! { leq 2.0 };
        assert_ord!(actual, OrderingToken::Leq, 2.0);

        let actual: ComplexTestCase = parse_quote! { greater_or_equal_than 2.0 };
        assert_ord!(actual, OrderingToken::Geq, 2.0);
        let actual: ComplexTestCase = parse_quote! { geq 2.0 };
        assert_ord!(actual, OrderingToken::Geq, 2.0);
    }

    #[test]
    fn can_parse_eq_other_types() {
        let actual: ComplexTestCase = parse_quote! { equal_to "abcde" };
        if let ComplexTestCase::Ord(ord) = actual {
            assert_eq!(ord.token, OrderingToken::Eq);
            let lit = ord.expected_value;
            let actual_expr: LitStr = parse_quote! { #lit };
            assert_eq!(actual_expr.value(), "abcde")
        } else {
            panic!("invalid enum variant")
        }

        let actual: ComplexTestCase = parse_quote! { equal_to 1 };
        if let ComplexTestCase::Ord(ord) = actual {
            assert_eq!(ord.token, OrderingToken::Eq);
            let lit = ord.expected_value;
            let actual_expr: LitInt = parse_quote! { #lit };
            assert_eq!(actual_expr.base10_parse::<i64>().unwrap(), 1)
        } else {
            panic!("invalid enum variant")
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn parses_almost_equal_token_stream() {
        let actual: ComplexTestCase = parse_quote! { almost_equal_to 1.0 precision 0.1 };
        assert_almost_eq!(actual, 1.0, 0.1);
        let actual: ComplexTestCase = parse_quote! { almost_equal_to 1.0 precision 0.0f32 };
        assert_almost_eq!(actual, 1.0, 0.0);
    }

    #[test]
    fn parses_path_token_stream() {
        let actual: ComplexTestCase = parse_quote! { existing_path };
        assert_eq!(
            actual,
            ComplexTestCase::Path(Path {
                token: PathToken::Any
            })
        );
        let actual: ComplexTestCase = parse_quote! { file };
        assert_eq!(
            actual,
            ComplexTestCase::Path(Path {
                token: PathToken::File
            })
        );
        let actual: ComplexTestCase = parse_quote! { dir };
        assert_eq!(
            actual,
            ComplexTestCase::Path(Path {
                token: PathToken::Dir
            })
        );
        let actual: ComplexTestCase = parse_quote! { directory };
        assert_eq!(
            actual,
            ComplexTestCase::Path(Path {
                token: PathToken::Dir
            })
        );
    }

    #[test]
    fn parses_contains_token_stream() {
        let actual: ComplexTestCase = parse_quote! { contains 1.0 };
        assert_eq!(
            actual,
            ComplexTestCase::Contains(Contains {
                expected_element: Box::new(parse_quote! { 1.0 })
            })
        );
        let actual: ComplexTestCase = parse_quote! { contains "abcde" };
        assert_eq!(
            actual,
            ComplexTestCase::Contains(Contains {
                expected_element: Box::new(parse_quote! { "abcde" })
            })
        );
        let actual: ComplexTestCase = parse_quote! { contains true };
        assert_eq!(
            actual,
            ComplexTestCase::Contains(Contains {
                expected_element: Box::new(parse_quote! { true })
            })
        );
    }

    #[test]
    fn parses_contains_in_order_token_stream() {
        let actual: ComplexTestCase = parse_quote! { contains_in_order [1, 2, 3] };
        assert_eq!(
            actual,
            ComplexTestCase::ContainsInOrder(ContainsInOrder {
                expected_slice: Box::new(parse_quote! { [1, 2, 3] })
            })
        )
    }

    #[test]
    fn parses_len_token_stream() {
        let actual1: ComplexTestCase = parse_quote! { len 10 };
        let actual2: ComplexTestCase = parse_quote! { has_length 11 };
        assert_eq!(
            actual1,
            ComplexTestCase::Len(Len {
                expected_len: Box::new(parse_quote! { 10 })
            })
        );

        assert_eq!(
            actual2,
            ComplexTestCase::Len(Len {
                expected_len: Box::new(parse_quote! { 11 })
            })
        )
    }

    #[test]
    fn parses_count_token_stream() {
        let actual1: ComplexTestCase = parse_quote! { count 10 };
        let actual2: ComplexTestCase = parse_quote! { has_count 11 };
        assert_eq!(
            actual1,
            ComplexTestCase::Count(Count {
                expected_len: Box::new(parse_quote! { 10 })
            })
        );
        assert_eq!(
            actual2,
            ComplexTestCase::Count(Count {
                expected_len: Box::new(parse_quote! { 11 })
            })
        )
    }

    #[test]
    fn parses_empty() {
        let actual: ComplexTestCase = parse_quote! { empty };
        assert_eq!(actual, ComplexTestCase::Empty,)
    }

    #[test]
    fn parses_negation() {
        let actual: ComplexTestCase = parse_quote! { not eq 1.0 };
        match actual {
            ComplexTestCase::Not(_) => {}
            _ => panic!("test failed"),
        };
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn parses_grouping() {
        let actual: ComplexTestCase = parse_quote! { (lt 1.0) };
        assert_ord!(actual, OrderingToken::Lt, 1.0);
        let actual: ComplexTestCase = parse_quote! { (((lt 1.0))) };
        assert_ord!(actual, OrderingToken::Lt, 1.0);
        let actual: ComplexTestCase = parse_quote! { ({[(lt 1.0)]}) };
        assert_ord!(actual, OrderingToken::Lt, 1.0)
    }

    #[test]
    fn parses_logic() {
        let actual: ComplexTestCase = parse_quote! { lt 1.0 and gt 0.0 };
        match actual {
            ComplexTestCase::And(v) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 0.0 or gt 1.0 };
        match actual {
            ComplexTestCase::Or(v) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 1.0 and gt 0.0 and eq 0.5 };
        match actual {
            ComplexTestCase::And(v) if v.len() == 3 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 0.0 or gt 1.0 or eq 2.0 };
        match actual {
            ComplexTestCase::Or(v) if v.len() == 3 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { (lt 0.0 or gt 1.0) and eq 2.0 };
        match actual {
            ComplexTestCase::And(v) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { (lt 1.0 and gt 0.0) or eq 2.0 };
        match actual {
            ComplexTestCase::Or(v) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
    }
}
