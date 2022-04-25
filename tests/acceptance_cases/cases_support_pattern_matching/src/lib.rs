#![cfg(test)]
use test_case::test_case;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum SimpleEnum {
    Var1,
    Var2,
}

#[test_case(SimpleEnum::Var2 => matches SimpleEnum::Var2)]
fn pattern_matching_result(e: SimpleEnum) -> SimpleEnum {
    e
}

#[test_case(SimpleEnum::Var1 => matches Ok(e) if e == SimpleEnum::Var1)]
#[test_case(SimpleEnum::Var1 => matches Ok(e) if e == SimpleEnum::Var2; "ok should fail")]
#[test_case(SimpleEnum::Var2 => matches Err(e) if e == "var2")]
#[test_case(SimpleEnum::Var2 => matches Err(e) if e == "var1"; "err should fail")]
fn extended_pattern_matching_result(e: SimpleEnum) -> Result<SimpleEnum, &'static str> {
    if e == SimpleEnum::Var1 {
        Ok(e)
    } else {
        Err("var2")
    }
}
