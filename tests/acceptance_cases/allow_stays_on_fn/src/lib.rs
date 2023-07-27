#![deny(unused_variables)]

use test_case::test_case;

#[test_case(42)]
#[allow(unused_variables)]
fn allow_stays_on_fn(value: u32) {}