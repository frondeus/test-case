#![cfg(test)]

#[macro_use]
extern crate lazy_static;

mod acceptance {
    use std::process::{Command, Output};

    fn run_tests() -> Output {
        Command::new("cargo")
            .args(&["test", "test_cases"])
            .output()
            .expect("cargo command failed to start")
    }

    lazy_static! {
        static ref ACTUAL: String = {
            let output = run_tests().stdout;

            String::from_utf8_lossy(&output).to_string()
        };
    }

    fn actual<'a>() -> &'a str {
        ACTUAL.as_ref()
    }

    #[test]
    fn runs_all_test_cases() {
        assert!(actual().contains("running 29 tests"));
    }

    #[test]
    fn escapes_unnecessary_leading_underscore() {
        assert!(actual().contains("test test_cases::leading_underscore_in_test_name::dummy ... ok"));
    }

    #[test]
    fn escapes_names_starting_with_digit() {
        assert!(actual().contains("test test_cases::basic_test::_1 ... ok"));
    }

    #[test]
    fn removes_repeated_underscores() {
        assert!(actual().contains("test test_cases::arg_expressions::_2_4_6_to_string ... ok"));
    }

    #[test]
    fn escapes_rust_keywords() {
        assert!(actual().contains("test test_cases::keyword_test::_true ... ok"));
    }
}