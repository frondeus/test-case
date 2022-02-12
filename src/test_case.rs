use crate::comment::TestCaseComment;
use crate::expr::TestCaseExpression;
use crate::utils::fmt_syn;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Error, Expr, Ident, ItemFn, Token};

#[cfg_attr(test, derive(Debug))]
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
                    acc.push_str(&format!("{}_", fmt_syn(&arg)));
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

    pub fn render(&self, mut item: ItemFn) -> TokenStream2 {
        let item_name = item.sig.ident.clone();
        let arg_values = self.args.iter();
        let test_case_name = self.test_case_name();

        let expected = self
            .expression
            .as_ref()
            .map(|expr| expr.assertion())
            .unwrap_or_else(TokenStream2::new);
        let mut attrs = self
            .expression
            .as_ref()
            .map(|expr| expr.attributes())
            .unwrap_or_else(Vec::new);

        attrs.push(parse_quote! { #[allow(clippy::bool_assert_comparison)] });

        attrs.append(&mut item.attrs);

        if let Some(_asyncness) = item.sig.asyncness {
            quote! {
                #(#attrs)*
                async fn #test_case_name() {
                    let _result = super::#item_name(#(#arg_values),*).await;
                    #expected
                }
            }
        } else {
            quote! {
                #[test]
                #(#attrs)*
                fn #test_case_name() {
                    let _result = super::#item_name(#(#arg_values),*);
                    #expected
                }
            }
        }
    }
}
