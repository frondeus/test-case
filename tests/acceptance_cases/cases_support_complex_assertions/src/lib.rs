#![cfg(test)]

use test_case::test_case;

#[test_case(1.0 => is equal_to 2.0 ; "eq1")]
#[test_case(1.0 => is eq 2.0 ; "eq2")]
#[test_case(1.0 => is less_than 3.0 ; "lt1")]
#[test_case(1.0 => is lt 3.0 ; "lt2")]
#[test_case(1.0 => is greater_than 0.0 ; "gt1")]
#[test_case(1.0 => is gt 0.0 ; "gt2")]
#[test_case(1.0 => is less_or_equal_than 2.0 ; "leq1")]
#[test_case(1.0 => is leq 2.0 ; "leq2")]
#[test_case(1.0 => is greater_or_equal_than 1.0 ; "geq1")]
#[test_case(1.0 => is geq 1.0 ; "geq2")]
#[test_case(1.0 => is almost_equal_to 2.1 precision 0.15 ; "almost_eq1")]
#[test_case(1.0 => is almost 2.0 precision 0.01 ; "almost_eq2")]
fn complex_tests(input: f64) -> f64 {
    input * 2.0
}

#[test_case("Cargo.toml" => is existing_path)]
#[test_case("src/lib.rs" => is file)]
#[test_case("src/" => is dir ; "short_dir")]
#[test_case("src/" => is directory ; "long_dir")]
fn create_path(val: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(val)
}

#[test_case(vec![1, 2, 3, 4] => it contains 1)]
#[test_case(vec![1, 2, 3, 4] => it contains_in_order [3, 4])]
fn contains_tests(items: Vec<u64>) -> Vec<u64> {
    items
}

#[test_case(1.0 => is not eq 2.5)]
#[test_case(1.0 => is not almost 2.1 precision 0.01)]
fn not_complex(input: f32) -> f32 { input * 1.0 }

#[test_case("Cargo.yaml".parse().unwrap() => is not existing_path)]
#[test_case("Cargo.toml".parse().unwrap() => is not dir)]
#[test_case("src/".parse().unwrap() => is not file)]
fn not_path(path: std::path::PathBuf) -> String {
    path.to_string_lossy().to_string()
}

#[test_case(vec![1, 2, 3, 4] => it not contains 5)]
#[test_case(vec![1, 2, 3, 4] => it not contains_in_order [3, 2])]
fn not_contains_tests(items: Vec<u64>) -> Vec<u64> {
    items
}

#[test_case(2.0 => it (eq 2.0))]
fn in_parens(_: f32) -> f32 {
    2.0
}

#[test_case(1.0 => is gt 0.0 and lt 5.0)]
#[test_case(1.0 => is gt 0.0 or lt 0.0)]
#[test_case(-2.0 => is gt 0.0 or lt 0.0)]
#[test_case(-2.0 => is (gt 0.0 or lt 0.0) and lt -1.0)]
#[test_case(1.0 => is (gt 0.0 or lt -1.5) and lt 2.0)]
#[test_case(0.3 => is (gt 0.0 and lt 1.0) or gt 1.2)]
#[test_case(0.7 => is (gt 0.0 and lt 1.0) or gt 1.2)]
fn combinators(v: f32) -> f32 {
    v * 2.0
}

#[test_case(vec![1, 2, 3] => it contains 1 and contains 2 and contains_in_order [2, 3])]
#[test_case(vec![1, 2, 3] => it contains 1 or contains 4)]
#[test_case(vec![1, 2, 3] => it (contains 1 or contains 4) and contains 2)]
#[test_case(vec![1, 2, 3] => it (contains 1 and contains 3) or contains 5)]
#[test_case(vec![1, 2, 3] => it (contains 6 and contains 7) or contains 1)]
#[test_case(vec![1, 2, 3] => it (contains 6 and contains 7) or (contains 1 and contains_in_order [1, 2, 3]))]
fn combinators_with_arrays(a: Vec<u8>) -> Vec<u8> {
    a
}
