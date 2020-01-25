use quote::quote;
use quote::ToTokens;
use syn::export::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Error, Expr, Ident, ItemFn, LitStr, Pat, Token};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct TestCase {
    test_case_name: String,
    args: Vec<Expr>,
    expected: Option<Expected>,
    case_desc: Option<LitStr>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Expected {
    Pat(Pat),
    Panic(LitStr),
    Ignored(Box<Expr>),
    Expr(Box<Expr>),
}

mod kw {
    syn::custom_keyword!(matches);
    syn::custom_keyword!(panics);
    syn::custom_keyword!(inconclusive);
}

impl ToString for Expected {
    fn to_string(&self) -> String {
        match self {
            Expected::Pat(p) => format!("matches {}", fmt_syn(&p)),
            Expected::Panic(p) => format!("panics {}", fmt_syn(&p)),
            Expected::Ignored(e) => format!("ignored {}", fmt_syn(&e)),
            Expected::Expr(e) => format!("expects {}", fmt_syn(&e)),
        }
    }
}

impl Parse for Expected {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::matches) {
            let _kw = input.parse::<kw::matches>()?;
            let pat = input.parse()?;
            return Ok(Expected::Pat(pat));
        }

        if lookahead.peek(kw::panics) {
            let _kw = input.parse::<kw::panics>()?;
            let pat = input.parse()?;
            return Ok(Expected::Panic(pat));
        }

        if lookahead.peek(kw::inconclusive) {
            let _kw = input.parse::<kw::inconclusive>()?;
            let expr = input.parse()?;
            return Ok(Expected::Ignored(expr));
        }

        let expr = input.parse()?;
        Ok(Expected::Expr(expr))
    }
}

fn fmt_syn(syn: &(impl ToTokens + Clone)) -> String {
    syn.clone().into_token_stream().to_string()
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut test_case_name = String::new();

        let mut args = vec![];
        loop {
            let exp: Expr = input.parse()?;
            test_case_name += &format!(" {}", fmt_syn(&exp));
            args.push(exp);
            if !input.peek(Token![,]) {
                break;
            }
            let _comma: Token![,] = input.parse()?;
        }

        let arrow: Option<Token![=>]> = input.parse()?;

        let expected = if arrow.is_some() {
            let expected: Expected = input.parse()?;

            test_case_name += &format!(" {}", expected.to_string());

            Some(expected)
        } else {
            None
        };

        let semicolon: Option<Token![;]> = input.parse()?;
        let case_desc = if semicolon.is_some() {
            let desc: LitStr = input.parse()?;
            Some(desc)
        } else {
            None
        };

        Ok(Self {
            test_case_name,
            args,
            expected,
            case_desc,
        })
    }
}

impl TestCase {
    pub fn test_case_name(&self) -> Ident {
        let case_desc = self
            .case_desc
            .as_ref()
            .map(|cd| cd.value())
            .unwrap_or_else(|| self.test_case_name.clone());
        crate::utils::escape_test_name(case_desc)
    }

    pub fn render(&self, mut item: ItemFn) -> TokenStream2 {
        let item_name = item.sig.ident.clone();
        let arg_values = self.args.iter();
        let test_case_name = self.test_case_name();
        let inconclusive = self
            .case_desc
            .as_ref()
            .map(|cd| cd.value().to_lowercase().contains("inconclusive"))
            .unwrap_or_default();

        let mut attrs = vec![];

        let expected: Expr = match &self.expected {
            Some(Expected::Pat(pat)) => {
                let pat_str = format!("{}", quote! { #pat });
                parse_quote! {
                    match _result {
                        #pat => (),
                        e => panic!("Expected {} found {:?}", #pat_str, e)
                    }
                }
            }
            Some(Expected::Expr(e)) => {
                parse_quote! { assert_eq!(#e, _result) }
            }
            Some(Expected::Panic(l)) => {
                attrs.push(parse_quote! { #[should_panic(expected = #l)] });
                parse_quote! {()}
            }
            Some(Expected::Ignored(_)) => {
                attrs.push(parse_quote! { #[ignore] });
                parse_quote! {()}
            }
            None => parse_quote! {()},
        };

        if inconclusive {
            attrs.push(parse_quote! { #[ignore] });
        }

        attrs.append(&mut item.attrs);

        quote! {
            #[test]
            #(#attrs)*
            fn #test_case_name() {
                let _result = #item_name(#(#arg_values),*);
                #expected
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod expected {
        use super::*;

        mod parse {
            use super::*;
            use syn::parse_quote;

            #[test]
            fn parses_expression() {
                let actual: Expected = parse_quote! { 2 + 3 };

                assert_eq!(Expected::Expr(parse_quote!(2 + 3)), actual);
            }

            #[test]
            fn parses_panic() {
                let actual: Expected = parse_quote! { panics "Error msg" };

                assert_eq!(Expected::Panic(parse_quote!("Error msg")), actual);
            }

            #[test]
            fn parses_pattern() {
                let actual: Expected = parse_quote! { matches Some(_) };

                assert_eq!(Expected::Pat(parse_quote!(Some(_))), actual);
            }
        }
    }

    mod test_case {
        use super::*;

        mod parse {
            use super::*;
            use syn::parse_quote;

            #[test]
            fn parses_basic_input() {
                let actual: TestCase = parse_quote! {
                    2, 10
                };

                assert_eq!(
                    TestCase {
                        test_case_name: " 2 10".to_string(),
                        args: vec![parse_quote!(2), parse_quote!(10),],
                        expected: None,
                        case_desc: None,
                    },
                    actual
                );
            }

            #[test]
            fn parses_input_with_expectation() {
                let actual: TestCase = parse_quote! {
                    2, 10 => 12
                };

                assert_eq!(
                    TestCase {
                        test_case_name: " 2 10 expects 12".to_string(),
                        args: vec![parse_quote!(2), parse_quote!(10),],
                        expected: Some(parse_quote!(12)),
                        case_desc: None,
                    },
                    actual
                );
            }

            #[test]
            fn parses_input_with_description() {
                let actual: TestCase = parse_quote! {
                    2, 10; "basic addition"
                };

                assert_eq!(
                    TestCase {
                        test_case_name: " 2 10".to_string(),
                        args: vec![parse_quote!(2), parse_quote!(10),],
                        expected: None,
                        case_desc: parse_quote!("basic addition"),
                    },
                    actual
                );
            }
        }
    }
}
