#![cfg(test)]

mod acceptance {
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

        let lines = String::from_utf8_lossy(&output.stdout).to_string();
        insta::assert_display_snapshot!(lines);
    }

    #[test]
    fn hamcrest_assertions() {
        let output = Command::new("cargo")
            .current_dir(PathBuf::from("acceptance_tests").join("hamcrest_assertions"))
            .args(&["test"])
            .output()
            .expect("cargo command failed to start");

        let lines = String::from_utf8_lossy(&output.stdout).to_string();
        insta::assert_display_snapshot!(lines);
    }

    #[test]
    fn r#async() {
        if !env::var("TEST_CASE_ACCEPTANCE_RUN_ASYNC").map_or(true, |v| v == "true") {
            return; // Test can be skipped via environment variable, when ran on rust < 1.41.0
        }

        let output = Command::new("cargo")
            .current_dir(PathBuf::from("acceptance_tests").join("async"))
            .args(&["test"])
            .output()
            .expect("cargo command failed to start");

        let lines = String::from_utf8_lossy(&output.stdout).to_string();
        insta::assert_display_snapshot!(lines);
    }
}
