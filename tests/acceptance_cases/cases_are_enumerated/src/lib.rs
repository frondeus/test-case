use test_case::test_case;

#[test_case(42)]
#[test_case("42")]
fn cases_are_enumerated(value: impl std::fmt::Display) {
    assert_eq!("42", &value.to_string())
}