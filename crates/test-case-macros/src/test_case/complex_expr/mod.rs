use crate::test_case::complex_expr::compare_to::CompareTo;
use almost_equal::AlmostEqual;
use contains::Contains;
use contains_in_order::ContainsInOrder;
use count::Count;
use empty::Empty;
use len::Len;
use logic::{And, Not, Or};
use path::Path;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{Display, Formatter};
use syn::parse::{Parse, ParseStream};

#[cfg(feature = "with-regex")]
use regex::Regex;

mod almost_equal;
mod compare_to;
mod contains;
mod contains_in_order;
mod count;
mod empty;
mod len;
mod logic;
mod path;

#[cfg(feature = "with-regex")]
mod regex;

#[derive(Debug, PartialEq)]
pub enum ComplexTestCase {
    Not(Not),
    And(And),
    Or(Or),
    CompareTo(CompareTo),
    AlmostEqual(AlmostEqual),
    Path(Path),
    Contains(Contains),
    ContainsInOrder(ContainsInOrder),
    Len(Len),
    Count(Count),
    Empty(Empty),
    #[cfg(feature = "with-regex")]
    Regex(Regex),
}

impl Parse for ComplexTestCase {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = Self::parse_single_item(input)?;

        if let Some(and) = And::parse(input)? {
            Ok(ComplexTestCase::And(and.with_first(item)))
        } else if let Some(or) = Or::parse(input)? {
            Ok(ComplexTestCase::Or(or.with_first(item)))
        } else {
            Ok(item)
        }
    }
}

impl Display for ComplexTestCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexTestCase::Not(not) => write!(f, "{}", not),
            ComplexTestCase::And(and) => write!(f, "{}", and),
            ComplexTestCase::Or(or) => write!(f, "{}", or),
            ComplexTestCase::CompareTo(ord) => write!(f, "{}", ord),
            ComplexTestCase::AlmostEqual(almost_equal) => write!(f, "{}", almost_equal),
            ComplexTestCase::Path(path) => write!(f, "{}", path),
            ComplexTestCase::Contains(contains) => write!(f, "{}", contains),
            ComplexTestCase::ContainsInOrder(contains_in_order) => {
                write!(f, "{}", contains_in_order)
            }
            ComplexTestCase::Len(len) => write!(f, "{}", len),
            ComplexTestCase::Count(count) => write!(f, "{}", count),
            ComplexTestCase::Empty(empty) => write!(f, "{}", empty),
            #[cfg(feature = "with-regex")]
            ComplexTestCase::Regex(regex) => write!(f, "{}", regex),
        }
    }
}

impl ComplexTestCase {
    pub fn assertion(&self) -> TokenStream {
        let tokens = self.boolean_check();

        quote! { assert!(#tokens) }
    }

    pub fn boolean_check(&self) -> TokenStream {
        match self {
            ComplexTestCase::Not(not) => not.boolean_check(),
            ComplexTestCase::And(and) => and.boolean_check(),
            ComplexTestCase::Or(or) => or.boolean_check(),
            ComplexTestCase::CompareTo(ord) => ord.boolean_check(),
            ComplexTestCase::AlmostEqual(almost_equal) => almost_equal.boolean_check(),
            ComplexTestCase::Path(path) => path.boolean_check(),
            ComplexTestCase::Contains(contains) => contains.boolean_check(),
            ComplexTestCase::ContainsInOrder(contains_in_order) => {
                contains_in_order.boolean_check()
            }
            ComplexTestCase::Len(len) => len.boolean_check(),
            ComplexTestCase::Count(count) => count.boolean_check(),
            ComplexTestCase::Empty(empty) => empty.boolean_check(),
            #[cfg(feature = "with-regex")]
            ComplexTestCase::Regex(regex) => regex.boolean_check(),
        }
    }

    fn parse_single_item(input: ParseStream) -> syn::Result<ComplexTestCase> {
        if let Ok(group) = Group::parse(input) {
            return syn::parse2(group.stream());
        }

        let lookahead = input.lookahead1();

        if let Some(not) = Not::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Not(not));
        }

        if let Some(compare_to) = CompareTo::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::CompareTo(compare_to));
        }

        if let Some(almost_equal) = AlmostEqual::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::AlmostEqual(almost_equal));
        }

        if let Some(path) = Path::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Path(path));
        }

        if let Some(contains) = Contains::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Contains(contains));
        }

        if let Some(contains_in_order) = ContainsInOrder::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::ContainsInOrder(contains_in_order));
        }

        if let Some(len) = Len::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Len(len));
        }

        if let Some(count) = Count::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Count(count));
        }

        if let Some(empty) = Empty::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Empty(empty));
        }

        #[cfg(feature = "with-regex")]
        if let Some(regex) = Regex::parse(input, &lookahead)? {
            return Ok(ComplexTestCase::Regex(regex));
        }

        Err(lookahead.error())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_case::complex_expr::almost_equal::AlmostEqual;
    use crate::test_case::complex_expr::compare_to::OrderingToken;
    use crate::test_case::complex_expr::contains::Contains;
    use crate::test_case::complex_expr::contains_in_order::ContainsInOrder;
    use crate::test_case::complex_expr::count::Count;
    use crate::test_case::complex_expr::empty::Empty;
    use crate::test_case::complex_expr::len::Len;
    use crate::test_case::complex_expr::logic::{And, Not, Or};
    use crate::test_case::complex_expr::path::{Path, PathToken};
    use crate::test_case::complex_expr::ComplexTestCase;
    use syn::{parse_quote, LitFloat, LitInt, LitStr};

    macro_rules! assert_ord {
        ($actual:tt, $token:path, $value:tt) => {
            if let ComplexTestCase::CompareTo(ord) = $actual {
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
        if let ComplexTestCase::CompareTo(ord) = actual {
            assert_eq!(ord.token, OrderingToken::Eq);
            let lit = ord.expected_value;
            let actual_expr: LitStr = parse_quote! { #lit };
            assert_eq!(actual_expr.value(), "abcde")
        } else {
            panic!("invalid enum variant")
        }

        let actual: ComplexTestCase = parse_quote! { equal_to 1 };
        if let ComplexTestCase::CompareTo(ord) = actual {
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
        assert_eq!(actual, ComplexTestCase::Empty(Empty),)
    }

    #[test]
    fn parses_negation() {
        let actual: ComplexTestCase = parse_quote! { not eq 1.0 };
        match actual {
            ComplexTestCase::Not(Not { inner: _ }) => {}
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
            ComplexTestCase::And(And { inner: v }) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 0.0 or gt 1.0 };
        match actual {
            ComplexTestCase::Or(Or { inner: v }) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 1.0 and gt 0.0 and eq 0.5 };
        match actual {
            ComplexTestCase::And(And { inner: v }) if v.len() == 3 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { lt 0.0 or gt 1.0 or eq 2.0 };
        match actual {
            ComplexTestCase::Or(Or { inner: v }) if v.len() == 3 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { (lt 0.0 or gt 1.0) and eq 2.0 };
        match actual {
            ComplexTestCase::And(And { inner: v }) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
        let actual: ComplexTestCase = parse_quote! { (lt 1.0 and gt 0.0) or eq 2.0 };
        match actual {
            ComplexTestCase::Or(Or { inner: v }) if v.len() == 2 => {}
            _ => panic!("test failed"),
        }
    }
}
