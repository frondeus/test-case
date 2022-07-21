#![cfg(test)]
use test_case::test_case;

#[test_case(() => inconclusive ())]
#[test_case(() => inconclusive (); "test is not ran")]
#[test_case(() => inconclusive (); "inconclusive test")]
#[test_case(() => ignore (); "ignore keyword")]
fn inconclusives(_: ()) {
    unreachable!()
}

#[test_case(1 => ignore)]
#[test_case(2 => ignore)]
fn ignore_void(input: u8) {
    assert_eq!(input, 1)
}

#[test_case(() => inconclusive["reason but no comment"] ())]
#[test_case(() => inconclusive["reason and comment"] (); "test is not run")]
#[test_case(() => ignore["reason and comment"] (); "ignore keyword")]
fn descriptions(_: ()) {
    unreachable!()
}
