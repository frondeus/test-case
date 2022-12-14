use test_case::test_case_assert_override;
use test_case::test_case;

test_case_assert_override!(
    assert_eq: crate::assert_eq,
    assert: std::assert,
);

pub mod testing_utils {
    #[macro_export]
    macro_rules! assert_eq {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!("custom assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right)
            }
        };
    }
}

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
