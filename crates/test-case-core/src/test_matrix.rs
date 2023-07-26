use std::{iter, mem};

use itertools::Itertools;
use proc_macro2::{Literal, Span};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, ExprLit, ExprRange, Lit, LitInt, RangeLimits, Token,
};

use crate::TestCase;

#[derive(Debug, Default)]
pub struct TestMatrix {
    variables: Vec<Vec<Expr>>,
}

impl TestMatrix {
    pub fn push_argument(&mut self, values: Vec<Expr>) {
        self.variables.push(values);
    }

    pub fn cases(&self) -> impl Iterator<Item = TestCase> {
        self.variables
            .iter()
            .cloned()
            .multi_cartesian_product()
            .map(TestCase::from)
    }
}

impl Parse for TestMatrix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut matrix = TestMatrix::default();
        for arg in args {
            let values: Vec<Expr> = match &arg {
                Expr::Array(v) => v.elems.iter().cloned().collect(),
                Expr::Tuple(v) => v.elems.iter().cloned().collect(),
                Expr::Range(ExprRange {
                    from, limits, to, ..
                }) => {
                    let start = u64_from_range_expr(arg.span(), from.as_deref())?;
                    let end = u64_from_range_expr(arg.span(), to.as_deref())?;
                    let range: Box<dyn Iterator<Item = u64>> = match limits {
                        RangeLimits::HalfOpen(_) => Box::from(start..end),
                        RangeLimits::Closed(_) => Box::from(start..=end),
                    };
                    range
                        .map(|n| {
                            Expr::from(ExprLit {
                                lit: Lit::from(LitInt::from(Literal::u64_unsuffixed(n))),
                                attrs: vec![],
                            })
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

fn u64_from_range_expr(range_span: Span, expr: Option<&Expr>) -> syn::Result<u64> {
    match expr {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Int(n), ..
        })) => n.base10_parse(),
        Some(e) => Err(syn::Error::new(
            e.span(),
            "Range bounds can only be an integer literal",
        )),
        None => Err(syn::Error::new(
            range_span,
            "Unbounded ranges are not supported",
        )),
    }
}
