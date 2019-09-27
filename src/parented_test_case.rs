use crate::test_case::TestCase;
use syn::parenthesized;
use syn::parse::{Parse, ParseStream};
use syn::Error;

pub struct ParentedTestCase {
    pub test_case: TestCase,
}

impl Parse for ParentedTestCase {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        let _ = parenthesized!(content in input);
        let test_case = TestCase::parse(&content)?;
        Ok(Self { test_case })
    }
}
