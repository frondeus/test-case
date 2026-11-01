#![deny(clippy::redundant_test_prefix)]

use test_case::{test_case, test_matrix};

#[test_case("foo")]
#[test_case("bar")]
fn consecutive_cases(_: &'static str) {}

#[test_case(1)]
fn single_case(_: u8) {}

#[test_case(true => true)]
#[test_case(false => false)]
fn cases_with_expectation(value: bool) -> bool {
    value
}

#[test_matrix([1, 2], [3, 4])]
fn matrix(_: u8, _: u8) {}
