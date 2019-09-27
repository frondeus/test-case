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
    fn runs_all_tests() {
        let actual = actual();
        let mut lines: Vec<_> = actual.lines().collect();
        lines.sort();
        let lines: String = lines.join("\n");
        insta::assert_display_snapshot!(lines);
    }
}
