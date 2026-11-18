use std::{iter, mem};

use proc_macro2::{Literal, Span};
use syn::{
    Expr, ExprLit, ExprRange, Lit, RangeLimits, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{TestCase, comment::TestCaseComment, expr::TestCaseExpression};

mod matrix_product;

#[derive(Debug, Default)]
pub struct TestMatrix {
    variables: Vec<Vec<Expr>>,
    expression: Option<TestCaseExpression>,
    comment: Option<TestCaseComment>,
}

impl TestMatrix {
    pub fn push_argument(&mut self, values: Vec<Expr>) {
        self.variables.push(values);
    }

    pub fn cases(&self) -> impl Iterator<Item = TestCase> {
        let expression = self.expression.clone();
        let comment = self.comment.clone();

        matrix_product::multi_cartesian_product(self.variables.iter().cloned()).map(move |v| {
            if let Some(comment) = comment.clone() {
                TestCase::new_with_prefixed_name(
                    v,
                    expression.clone(),
                    comment.comment.value().as_ref(),
                )
            } else {
                TestCase::new(v, expression.clone(), None)
            }
        })
    }
}

impl Parse for TestMatrix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Punctuated<Expr, Token![,]> = Punctuated::parse_separated_nonempty(input)?;

        let expression = input.peek(Token![=>]).then(|| input.parse()).transpose()?;
        let comment = input.peek(Token![;]).then(|| input.parse()).transpose()?;
        if !input.is_empty() {
            return Err(input.error("unexpected token"));
        }

        let mut matrix = TestMatrix {
            expression,
            comment,
            ..Default::default()
        };

        for arg in args {
            let values: Vec<Expr> = match &arg {
                Expr::Array(v) => v.elems.iter().cloned().collect(),
                Expr::Tuple(v) => v.elems.iter().cloned().collect(),
                Expr::Range(ExprRange {
                    start, limits, end, ..
                }) => {
                    let start = isize_from_range_expr(limits.span(), start.as_deref())?;
                    let end = isize_from_range_expr(limits.span(), end.as_deref())?;
                    let range: Box<dyn Iterator<Item = isize>> = match limits {
                        RangeLimits::HalfOpen(_) => Box::from(start..end),
                        RangeLimits::Closed(_) => Box::from(start..=end),
                    };
                    range
                        .map(|n| {
                            let mut lit = Lit::new(Literal::isize_unsuffixed(n));
                            lit.set_span(arg.span());
                            Expr::Lit(ExprLit { lit, attrs: vec![] })
                        })
                        .collect()
                }
                v => iter::once(v.clone()).collect(),
            };

            let mut value_literal_type = None;
            for expr in &values {
                if let Expr::Lit(ExprLit { lit, .. }) = expr {
                    let first_literal_type =
                        *value_literal_type.get_or_insert_with(|| mem::discriminant(lit));
                    if first_literal_type != mem::discriminant(lit) {
                        return Err(syn::Error::new(
                            lit.span(),
                            "All literal values must be of the same type",
                        ));
                    }
                }
            }
            matrix.push_argument(values);
        }

        Ok(matrix)
    }
}

fn isize_from_range_expr(limits_span: Span, expr: Option<&Expr>) -> syn::Result<isize> {
    match expr {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Int(n), ..
        })) => n.base10_parse(),
        Some(e) => Err(syn::Error::new(
            e.span(),
            "Range bounds can only be an integer literal",
        )),
        None => Err(syn::Error::new(
            limits_span,
            "Unbounded ranges are not supported",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_optional_expression_and_comment() {
        for input in [
            "[1, 2]",
            "[1, 2] => 3",
            r#"[1, 2]; "description""#,
            r#"[1, 2] => 3; "description""#,
            r#"[1, 2] => ignore; "ignored""#,
            r#"[1, 2] => panics; "bare panic""#,
        ] {
            assert!(
                syn::parse_str::<TestMatrix>(input).is_ok(),
                "expected `{input}` to parse"
            );
        }
    }

    #[test]
    fn propagates_expression_and_comment_parse_errors() {
        let err =
            syn::parse_str::<TestMatrix>(r#"[1, 2] => is unknown_keyword; "desc""#).unwrap_err();
        assert_eq!(err.to_string(), "cannot parse complex expression");

        let err = syn::parse_str::<TestMatrix>("[1, 2]; 123").unwrap_err();
        assert_eq!(err.to_string(), "expected string literal");

        for input in [
            r#"[1, 2] => 2 + ; "desc""#,
            r#"[1, 2] => ; "desc""#,
            "[1, 2] => 2;",
            "[1, 2] => ignore[123] 2",
            "[1, 2] => ignore 2 +",
        ] {
            assert!(
                syn::parse_str::<TestMatrix>(input).is_err(),
                "expected `{input}` to fail parsing"
            );
        }
    }
}
