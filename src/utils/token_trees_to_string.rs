use crate::prelude::*;

pub trait TokenTreesToString {
    fn to_string(&self) -> String;
}

impl TokenTreesToString for [TokenTree] {
    fn to_string(&self) -> String {
        let tt = self;

        format!("{}", quote! { #(#tt)* })
    }
}
