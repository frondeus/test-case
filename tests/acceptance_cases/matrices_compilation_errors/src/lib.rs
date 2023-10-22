#![cfg(test)]
use test_case::test_matrix;

#[test_matrix(
    ["one", 1, true,]
)]
fn mixed_literals(x: u32) {
    unreachable!("Should never compile")
}

const END: u32 = 1;

#[test_matrix(1..END)]
fn non_literal_range(x: u32) {
    unreachable!("Should never compile")
}

#[test_matrix(0..9_223_372_036_854_775_808)]
fn range_outside_isize_bounds(x: u32) {
    unreachable!("Should never compile")
}

#[test_matrix(1..)]
fn unbounded_range(x: u32) {
    unreachable!("Should never compile")
}

const USIZE_CONST: usize = 0;

#[test_matrix(USIZE_CONST)]
fn wrong_argument_type(x: i8) {
    unreachable!("Should never compile")
}
