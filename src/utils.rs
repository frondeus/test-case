use proc_macro2::{Ident, Span};

pub fn escape_test_name(input: impl Into<String>) -> Ident {
    let mut last_under = false;
    let mut ident: String = input
        .into()
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| match c {
            c if c.is_alphanumeric() => {
                last_under = false;
                Some(c.to_ascii_lowercase())
            }
            _ if !last_under => {
                last_under = true;
                Some('_')
            }
            _ => None,
        })
        .collect();

    if !ident.starts_with(|c: char| c == '_' || c.is_ascii_alphabetic()) {
        ident = "_".to_string() + &ident;
    }
    let ident = ident.trim_end_matches('_');
    Ident::new(&ident, Span::call_site())
}
