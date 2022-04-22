#![cfg(test)]
use test_case::test_case;

pub fn assert_is_power_of_two(input: u64) {
    assert!(input.is_power_of_two())
}

mod some_mod {
    pub use super::assert_is_power_of_two;
}

#[test_case(1 => using assert_is_power_of_two)]
#[test_case(2 => using crate::assert_is_power_of_two)]
#[test_case(4 => using some_mod::assert_is_power_of_two)]
fn power_of_two_with_using(input: u64) -> u64 {
    input
}

fn wrapped_pretty_assert(expected: u64) -> impl Fn(u64) {
    move |actual: u64| { pretty_assertions::assert_eq!(actual, expected) }
}

#[test_case(1 => using wrapped_pretty_assert(1))]
fn pretty_assertions_usage(input: u64) -> u64 {
    input
}
