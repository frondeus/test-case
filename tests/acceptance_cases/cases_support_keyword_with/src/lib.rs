#![cfg(test)]
use test_case::test_case;

#[test_case(1.0 => with |v: f64| assert!(v.is_infinite()))]
#[test_case(0.0 => with |v: f64| assert!(v.is_nan()))]
fn divide_by_zero_f64_with_lambda(input: f64) -> f64 {
    input / 0.0f64
}
