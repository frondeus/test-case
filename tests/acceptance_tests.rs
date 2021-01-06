#![cfg(test)]

mod acceptance {
    use itertools::Itertools;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn basic() {
        let output = Command::new("cargo")
            .current_dir(PathBuf::from("acceptance_tests").join("basic"))
            .args(&["test"])
            .output()
            .expect("cargo command failed to start");

        let lines = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .sorted()
            .join("\n");
        insta::assert_display_snapshot!(lines);
    }

    #[test]
    fn hamcrest_assertions() {
        let output = Command::new("cargo")
            .current_dir(PathBuf::from("acceptance_tests").join("hamcrest_assertions"))
            .args(&["test"])
            .output()
            .expect("cargo command failed to start");

        let lines = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .sorted()
            .join("\n");
        insta::assert_display_snapshot!(lines);
    }

    #[test]
    fn r#async() {
        let output = Command::new("cargo")
            .current_dir(PathBuf::from("acceptance_tests").join("async"))
            .args(&["test"])
            .output()
            .expect("cargo command failed to start");

        let lines = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .sorted()
            .join("\n");
        insta::assert_display_snapshot!(lines);
    }
}
