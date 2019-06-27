use crate::prelude::*;

pub fn escape_ident<S: AsRef<str>>(raw: S) -> Token {
    let mut escaped = escape_chars(raw);

    escaped = escaped.trim_end_matches('_').to_string();

    if escaped.is_rust_keyword() || escaped.starts_with(is_digit) {
        escaped = "_".to_string() + &escaped
    }

    TIdent(escaped.into())
}

fn escape_chars<S: AsRef<str>>(raw: S) -> String {
    let mut escaped = Vec::with_capacity(raw.as_ref().len());
    let iter = raw.as_ref().chars();

    for c in iter {
        match c {
            'A'...'Z' => escaped.push(c.to_ascii_lowercase()),
            'a'...'z' | '0'...'9' => escaped.push(c),
            _ if !escaped.is_empty() && !is_ending_with_underscore(&escaped) => escaped.push('_'),
            _ => {}
        }
    }

    if escaped.last() == Some(&'_') {
        let _ = escaped.pop();
    }

    escaped.into_iter().collect()
}
