use std::{iter, mem};

use proc_macro2::{Literal, Span};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, ExprLit, ExprRange, Lit, RangeLimits, Token,
};

use crate::{comment::TestCaseComment, expr::TestCaseExpression, TestCase};

mod matrix_product;

#[derive(Debug, Default)]
pub struct TestMatrix {
    variables: Vec<Vec<Expr>>,
    expression: Option<TestCaseExpression>,
}

impl TestMatrix {
    pub fn push_argument(&mut self, values: Vec<Expr>) {
        self.variables.push(values);
    }

    pub fn cases(&self) -> impl Iterator<Item = TestCase> {
        let expression = self.expression.clone();

        matrix_product::multi_cartesian_product(self.variables.iter().cloned()).map(move |v| {
            let mut case = TestCase::from(v);
            case.expression = expression.clone();
            case
        })
    }
}

impl Parse for TestMatrix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Punctuated<Expr, Token![,]> = Punctuated::parse_separated_nonempty(input)?;

        let mut matrix = TestMatrix {
            expression: input.parse().ok(),
            ..Default::default()
        };

        if let Ok(c) = input.parse::<TestCaseComment>() {
            return Err(syn::Error::new(
                c.span(),
                "Comments are not allowed in #[test_matrix]",
            ));
        }

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
                            Expr::from(ExprLit { lit, attrs: vec![] })
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
