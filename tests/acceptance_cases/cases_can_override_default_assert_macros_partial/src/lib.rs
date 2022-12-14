use test_case::test_case_assert_override;
use test_case::test_case;

test_case_assert_override!(
    assert: std::assert,
);

#[test_case(1 => 1)]
#[test_case(1 => 2)]
fn test(i: i32) -> i32 {
    i
}

#[test_case(1 => is eq 1)]
#[test_case(1 => is eq 2)]
fn test_2(i: i32) -> i32 {
    i
}
