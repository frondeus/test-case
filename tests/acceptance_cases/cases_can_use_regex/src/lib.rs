#![cfg(test)]

use test_case::test_case;

#[test_case("abcabc" => is matching_regex r#"abc"#)]
#[test_case("abcabc201" => is matching_regex r#"\d"#)]
#[test_case("abcabc201" => is matching_regex r#"\d{4}"#)]
#[test_case("kumkwat" => is matching_regex r#"abc"#)]
#[test_case("kumkwat" => is matching_regex r#"\"#)]
#[test_case("kumkwat" => it matches_regex r#"^kumkwat$"#)]
fn regex_test(text: &str) -> &str {
    text
}
