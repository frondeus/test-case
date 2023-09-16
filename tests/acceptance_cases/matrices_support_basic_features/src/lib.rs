#[cfg(test)]
mod test_cases {
    use test_case::{test_case, test_matrix};

    #[test_matrix(
        [1, 2],
        [11, 12]
    )]
    fn numeric_values_array(x: u32, y: u32) {
        assert!(x < 10);
        assert!(y > 10);
    }

    #[test_matrix(
        1..10,
        [11, 12]
    )]
    fn matrix_with_range(x: u32, y: u32) {
        assert!(x < 10);
        assert!(y > 10);
    }

    #[test_matrix(
        ("one", "two"),
        ("yellow", "blue")
    )]
    fn str_values_tuple(a: &str, b: &str) {
        assert!(a.len() == 3);
        assert!(b.len() > 3);
    }

    #[test_matrix(
        "just",
        (1, 2, 3)
    )]
    fn matrix_with_singleton(a: &str, b: u32) {
        assert_eq!(a, "just");
        assert!(b < 10);
    }

    #[test_matrix("alone")]
    fn only_singleton(a: &str) {
        assert_eq!(a, "alone");
    }

    const TWO: u32 = 2;

    fn double(x: u32) -> u32 {
        x * TWO
    }

    #[test_matrix(
        2,
        [double(2), 2 * TWO, 4]
    )]
    fn matrix_with_expressions(x: u32, two_x: u32) {
        assert_eq!(2 * x, two_x);
    }

    #[test_matrix(["foo", "bar", "baz"])]
    fn impl_trait(x: impl AsRef<str>) {
        assert_eq!(3, x.as_ref().len());
    }

    #[test_matrix(
        true,
        [true, false]
    )]
    fn matrix_with_keywords(x: bool, y: bool) {
        assert!(x || y)
    }

    #[test_matrix(
        [1, 2, 3]
    )]
    #[test_case(4)]
    fn case_after_matrix(x: u32) {
        assert!(x < 10);
    }

    #[test_case(5)]
    #[test_matrix(
        [6, 7, 8]
    )]
    fn case_before_matrix(x: u32) {
        assert!(x < 10);
    }

    #[test_matrix(
        [1, 2,],
        [11, 12,]
    )]
    #[should_panic(expected = "Always panics")]
    fn matrix_with_should_panic(_x: u32, _y: u32) {
        panic!("Always panics")
    }

    #[test_matrix(
        [1, 2,],
        [11, 12,]
        => panics "Always panics"
    )]
    fn matrix_with_panics(_x: u32, _y: u32) {
        panic!("Always panics")
    }

    // tests from documentation

    // TODO
}
