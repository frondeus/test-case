use crate::comment::TestCaseComment;
use crate::expr::{TestCaseExpression, TestCaseResult};
use crate::utils::fmt_syn;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Error, Expr, Ident, ItemFn, ReturnType, Token};

#[derive(Debug)]
pub struct TestCase {
    args: Punctuated<Expr, Token![,]>,
    expression: Option<TestCaseExpression>,
    comment: Option<TestCaseComment>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        Ok(Self {
            args: Punctuated::parse_separated_nonempty_with(input, Expr::parse)?,
            expression: input.parse().ok(),
            comment: input.parse().ok(),
        })
    }
}

impl TestCase {
    pub fn test_case_name(&self) -> Ident {
        let case_desc = self
            .comment
            .as_ref()
            .map(|item| item.comment.value())
            .unwrap_or_else(|| {
                let mut acc = String::new();
                for arg in &self.args {
                    acc.push_str(&fmt_syn(&arg));
                    acc.push('_');
                }
                acc.push_str("expects");
                if let Some(expression) = &self.expression {
                    acc.push(' ');
                    acc.push_str(&expression.to_string())
                }
                acc
            });
        crate::utils::escape_test_name(case_desc)
    }

    pub fn render(&self, mut item: ItemFn, origin_span: Span2) -> TokenStream2 {
        let item_name = item.sig.ident.clone();
        let arg_values = self.args.iter();
        let test_case_name = {
            let mut test_case_name = self.test_case_name();
            test_case_name.set_span(origin_span);
            test_case_name
        };

        let mut attrs = self
            .expression
            .as_ref()
            .map(|expr| expr.attributes())
            .unwrap_or_else(Vec::new);

        attrs.push(parse_quote! { #[allow(clippy::bool_assert_comparison)] });
        attrs.append(&mut item.attrs);

        let (mut signature, body) = if item.sig.asyncness.is_some() {
            (
                quote! { async },
                quote! { let _result = super::#item_name(#(#arg_values),*).await; },
            )
        } else {
            attrs.insert(0, parse_quote! { #[::core::prelude::v1::test] });
            (
                TokenStream2::new(),
                quote! { let _result = super::#item_name(#(#arg_values),*); },
            )
        };

        let expected = if let Some(expr) = self.expression.as_ref() {
            attrs.extend(expr.attributes());

            signature.extend(quote! { fn #test_case_name() });

            if let TestCaseResult::Panicking(_) = expr.result {
                TokenStream2::new()
            } else {
                expr.assertion()
            }
        } else {
            signature.extend(if let ReturnType::Type(_, typ) = item.sig.output {
                quote! { fn #test_case_name() -> #typ }
            } else {
                quote! { fn #test_case_name() }
            });

            quote! { _result }
        };

        quote! {
            #(#attrs)*
            #signature {
                #body
                #expected
            }
        }
    }
}
