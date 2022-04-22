#![cfg(test)]
use test_case::test_case;

#[test_case(() => inconclusive ())]
#[test_case(() => inconclusive (); "test is not ran")]
#[test_case(() => inconclusive (); "inconclusive test")]
#[test_case(() => ignore (); "ignore keyword")]
fn inconclusives(_: ()) {
    unreachable!()
}
