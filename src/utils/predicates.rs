use prelude::*;

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_delimited(tt: &TokenTree) -> bool {
    match tt {
        &TTDelimited(_) => true,
        _               => false
    }
}

pub fn is_test_case(delimited: &Delimited) -> bool {
    delimited.delim        == Bracket
    && delimited.tts.len() == 2
    && delimited.tts[0]    == TTToken(TIdent("test_case".into()))
    && is_delimited(&delimited.tts[1])
}

pub fn is_ending_with_underscore(ident: &Vec<char>) -> bool {
    ident.iter().last() == Some(&'_')
}
