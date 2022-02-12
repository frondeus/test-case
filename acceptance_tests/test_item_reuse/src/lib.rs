pub fn normal_public_function(value: i32) -> i32 {
    internal_tested_function1(value) * internal_tested_function2(value) * internal_tested_function3(value) * internal_tested_function4(value)
}

#[test_case::test_case(2 => 4)]
#[test_case::test_case(3 => 6)]
fn internal_tested_function1(value: i32) -> i32 {
    if value == 3 { 0 } else { value * 2 }
}

use test_case::test_case;

#[test_case(1 => 0)]
fn internal_tested_function2(value: i32) -> i32 {
    value / 2
}

#[test_case(1 => matches 3)]
#[test_case(2 => inconclusive 6)]
fn internal_tested_function3(value: i32) -> i32 {
    value + 2
}

#[test_case(2 => panics "Can't")]
fn internal_tested_function4(value: i32) -> i32 {
    panic!("Can't")
}
