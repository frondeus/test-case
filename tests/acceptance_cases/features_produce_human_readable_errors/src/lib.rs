#![cfg(test)]

use test_case::test_case;

#[test_case(2 => is matching_regex "abc")]
fn fail_on_missing_with_regex_feature(_: u8) -> String {
    todo!()
}

#[test_case::for_each(file in "src/")]
fn fail_on_missing_with_for_each_file_feature(file: &str) {
    todo!()
}
