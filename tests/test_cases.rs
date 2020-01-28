mod test_cases {
    use test_case::test_case;

    #[test_case(2)]
    #[test_case(4)]
    fn multiple_test_cases(x: u32) {
        assert!(x < 10)
    }

    #[test_case(1)]
    fn basic_test(x: u32) {
        assert_eq!(x, 1)
    }

    #[test_case("foo")]
    fn impl_trait(x: impl AsRef<str>) {
        assert_eq!("foo", x.as_ref());
    }

    #[test_case(2 => 4)]
    #[test_case(4 => 8)]
    fn result(x: u32) -> u32 {
        x * 2
    }

    #[test_case(1, 8 ; "test 1 + 8 = 9")]
    #[test_case(2, 7 ; "2nd test")]
    #[test_case(3, 6 ; "test_3_+6_=_9")]
    #[test_case(4, 5)]
    fn name(x: u32, y: u32) {
        assert_eq!(9, x + y)
    }

    #[test_case(1, 2 => 3 ; "test no. 1")]
    #[test_case(4, 5 => 9)]
    fn result_and_name(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(true)]
    fn keyword_test(x: bool) {
        assert!(x)
    }

    #[test_case(2 + 4, "6".to_string())]
    fn arg_expressions(x: u32, expected: String) {
        assert_eq!(expected, x.to_string())
    }

    #[test_case(2, 2 => 2 + 2)]
    fn result_expression(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(2, 2 => 2 + 3)]
    #[should_panic(expected = "assertion failed: `(left == right)`")]
    fn result_which_panics(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(2, 2 => 2 + 2 ; "test result expression")]
    fn result_expresion_with_name(x: u32, y: u32) -> u32 {
        x + y
    }

    fn foo() -> u32 {
        42
    }

    #[test_case("dummy")]
    fn leading_underscore_in_test_name(x: &str) {
        assert_eq!("dummy", x)
    }

    #[test_case("DUMMY_CODE")]
    fn lowercase_test_name(x: &str) {
        assert_eq!("DUMMY_CODE", x)
    }

    mod nested {
        use super::*;
        use test_case::test_case;

        #[test_case(1, 1)]
        fn nested_test_case(x: u32, y: u32) {
            assert_eq!(x, y)
        }

        #[test_case(20 + 22)]
        #[test_case(42)]
        fn using_fn_from_super(x: u32) {
            assert_eq!(foo(), x)
        }
    }

    #[test_case(42 => std::string::String::new())]
    fn result_with_mod_sep(_: i8) -> String {
        "".to_string()
    }

    // tests from documentation

    #[test_case( 2 =>  2 ; "returns given number for positive input")]
    #[test_case(-2 =>  2 ; "returns opposite number for non-positive input")]
    #[test_case( 0 =>  0 ; "returns 0 for 0")]
    fn abs_tests(x: i8) -> i8 {
        if x > 0 {
            x
        } else {
            -x
        }
    }

    #[test_case(None,    None    => 0 ; "treats none as 0")]
    #[test_case(Some(2), Some(3) => 5)]
    #[test_case(Some(2 + 3), Some(4) => 2 + 3 + 4)]
    fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
        x.unwrap_or(0) + y.unwrap_or(0)
    }

    #[test_case( 2,  4 ; "when both operands are possitive")]
    #[test_case( 4,  2 ; "when operands are swapped")]
    #[test_case(-2, -4 ; "when both operands are negative")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = x * y;

        assert_eq!(8, actual);
    }

    #[test_case("inconclusive" ; "should not take into account keyword on argument position")]
    #[test_case("dummy" ; "this test is inconclusive and will always be")]
    #[test_case("dummy" ; "this test is also Inconclusive")]
    #[test_case("dummy" ; "this test is also INCONCLUSIVE even all caps")]
    #[test_case("dummy" ; "this test is also iNCONCLUSIVE even inverted caps")]
    fn inconclusive_tests(_s: &str) {}

    const MY_CONST: &str = "my const";

    #[test_case(MY_CONST ; "this is desc, not an argument")]
    fn const_in_arg(_s: &str) {}

    #[test_case(""     => String::default())]
    fn foo(_: &str) -> String {
        String::default()
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum SimpleEnum {
        Var1(i32),
        Var2(char, i32),
    }

    #[test_case(SimpleEnum::Var2('a', 4) => matches SimpleEnum::Var2(_, 4))]
    fn pattern_matching_result(e: SimpleEnum) -> SimpleEnum {
        e
    }

    #[should_panic(expected = "Expected SimpleEnum :: Var2 (_, 5) found Var2('a', 4)")]
    #[test_case(SimpleEnum::Var2('a', 4) => matches SimpleEnum::Var2(_, 5))]
    fn pattern_matching_result_fails(e: SimpleEnum) -> SimpleEnum {
        e
    }

    #[test_case(() => panics "It has to panic")]
    fn panicing(_: ()) {
        panic!("It has to panic")
    }

    #[test_case(() => inconclusive ())]
    #[test_case(() => inconclusive (); "test is not ran")]
    #[test_case(() => inconclusive (); "inconclusive test")]
    fn inconclusives(_: ()) {
        unreachable!()
    }
}
