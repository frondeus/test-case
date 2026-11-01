#![cfg(test)]
use test_case::test_case;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum SimpleEnum {
    Var1,
    Var2,
}

#[should_panic(expected = "Expected `SimpleEnum :: Var2` found Var1")]
#[test_case(SimpleEnum::Var1 => matches SimpleEnum::Var2)]
fn pattern_matching_result_fails(e: SimpleEnum) -> SimpleEnum {
    e
}

#[test_case(() => panics "It has to panic")]
#[test_case(() => panics "This should fail")]
fn panicking(_: ()) {
    panic!("It has to panic")
}

#[test_case(() => panics)]
fn panics_without_value(_: ()) {
    panic!("Message doesn't matter")
}

#[test_case(2, 2 => 2 + 3)]
#[should_panic(expected = "\
assertion `left == right` failed
  left: 4
 right: 5\
")]
fn result_which_panics(x: u32, y: u32) -> u32 {
    x + y
}

#[test_case(1, 2 => panics; "case 1, panics")]
#[test_case(3, 2; "case 2, okay")]
#[test_case(1, 2 => panics "assertion `left == right` failed"; "case 3, panic message")]
fn test_add_5(a: i32, b: i32) {
    assert_eq!(a + b, 5);
}
