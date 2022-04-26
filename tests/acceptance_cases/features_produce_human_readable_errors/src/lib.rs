#![cfg(test)]

use test_case::test_case;

#[test_case(2 => is matching_regex "abc")]
fn fail_on_missing_with_regex_feature(_: u8) -> String {
    todo!()
}
