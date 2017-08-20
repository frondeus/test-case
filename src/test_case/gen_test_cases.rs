use prelude::*;

pub fn gen_test_case(test_fn_name: Token, token_tree: &Vec<TokenTree>) -> Tokens {
    let (name, leftover)     = given_test_case_name(token_tree);
    let (expected, leftover) = expected_result(leftover);
    let args                 = leftover;
    let name                 = name.unwrap_or_else(|| generated_test_case_name(args));
    let body                 = test_case_body(test_fn_name, args, expected);

    quote! {
        #[test]
        fn #name() {
            #body
        }
    }
}

fn given_test_case_name(token_tree: &[TokenTree]) -> (Option<Token>, &[TokenTree]) {
    let mut iter     = token_tree.into_iter().rev();
    let last         = iter.next();
    let last_but_one = iter.next();

    match (last, last_but_one) {
        (
            Some(&TTToken(TLiteral(Str(ref case_name, _)))),
            Some(&TTToken(TModSep))
        ) =>
            (
                Some(escape_ident(case_name)),
                &token_tree[0..token_tree.len() - 2]
            ),
        _ => (None, token_tree)
    }
}

fn generated_test_case_name(args: &[TokenTree]) -> Token {
    escape_ident(args.to_string())
}

fn test_case_body(test_fn_name: Token, args: &[TokenTree], expected: &[TokenTree]) -> Tokens {
    match expected.len() {
        0 => quote! {
            #test_fn_name(#(#args)*);
        },
        _ => quote! {
            let expected = #(#expected)*;
            let actual   = #test_fn_name(#(#args)*);
            
            assert_eq!(expected, actual);
        }
    }
}

fn expected_result(token_tree: &[TokenTree]) -> (&[TokenTree], &[TokenTree]) {
    for i in (0..token_tree.len()).rev() {
        if &TTToken(TFatArrow) == &token_tree[i] {
            return (
                &token_tree[i+1..token_tree.len()],
                &token_tree[0..i]
            )
        }
    }

    (&[], token_tree)
}