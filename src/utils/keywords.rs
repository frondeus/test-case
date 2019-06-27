use prelude::*;

lazy_static! {
    static ref RUST_KEYWORDS: HashSet<&'static str> = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", 
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", 
        "pub", "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", 
        "type", "unsafe", "use", "where", "while", "abstract", "alignof", "become", "box", "do", 
        "final", "macro", "offsetof", "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", 
        "virtual", "yield"
    ].iter().cloned().collect();
}

pub trait IsRustKeyword {
    fn is_rust_keyword(&self) -> bool;
}

impl IsRustKeyword for String {
    fn is_rust_keyword(&self) -> bool {
        RUST_KEYWORDS.contains::<str>(self)
    }
}
