use prelude::*;

pub struct TestCaseSuit {
    attrs: Vec<Vec<TokenTree>>,
    body:  Vec<TokenTree>,
    name:  String
}

impl TestCaseSuit {
    pub fn body_tokens(&self) -> Tokens {
        let body = &self.body;

        quote! { #(#body)* }
    }

    fn name_token(&self) -> Token {
        TIdent((&self.name as &str).into())
    }

    pub fn gen_test_cases(&self) -> Tokens {
        let name = self.name_token();
        let body = self.body_tokens();
        let test_cases = 
            self.attrs
                .iter()
                .map(|tt| {
                    gen_test_case(self.name_token(), tt)
                })
                .collect::<Vec<_>>();

        quote! { 
            mod #name {
                #[allow(unused_imports)]
                use super::*;

                #body

                #(#test_cases)*
            }
        }
    }
}

impl From<Vec<TokenTree>> for TestCaseSuit {
    fn from(token_tree: Vec<TokenTree>) -> Self {
        let mut attrs     = Vec::new();
        let mut leftover  = Vec::new();
        let mut name      = None;
        let mut iter      = token_tree.into_iter().peekable();
        let mut skip_next = false;

        while let Some(token) = iter.next() {
            if skip_next { skip_next = false; continue }

            let next_token = iter.peek();

            if let Some(attributes) = try_get_test_case(&token, next_token) { 
                attrs.push(attributes.clone());
                skip_next = true;
                continue;
            }
            
            name = name.or(try_parse_fn_name(&token, next_token));
            
            leftover.push(token);
        }

        Self {
            attrs: attrs,
            body:  leftover,
            name:  name.expect("Couldn't find test function name")
        }
    }
}

fn try_get_test_case<'a>(token: &'a TokenTree, next_token: Option<&'a TokenTree>) -> Option<&'a Vec<TokenTree>> {
    if token == &TTToken(TPound) {
        if let Some(&TTDelimited(ref delimited)) = next_token {
            if is_test_case(delimited) {
                if let TTDelimited(ref inner_delimited) = delimited.tts[1] {
                    return Some(&inner_delimited.tts);
                }
            }
        }
    }

    None
}

fn try_parse_fn_name(token: &TokenTree, next_token: Option<&TokenTree>) -> Option<String> {
    if token == &TTToken(TIdent("fn".into())) {
        if let Some(&TTToken(TIdent(ref ident))) = next_token {
            return Some(ident.to_string());
        }
    }

    None
}
