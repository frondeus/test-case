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

#[test_matrix(1..)]
fn unbounded_range(x: u32) {
    unreachable!("Should never compile")
}

#[test_matrix(
    [1, 2, 3]
    ; "Illegal comment"
)]
fn illegal_comment(x: u32) {
    unreachable!("Should never compile")
}
