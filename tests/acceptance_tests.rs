#![cfg(test)]

mod acceptance {
    use insta::with_settings;
    use itertools::Itertools;
    use std::env;
    use std::path::PathBuf;
    use std::process::{Command, Output};

    fn get_snapshot_directory() -> String {
        PathBuf::from("snapshots")
            .join(env::var("SNAPSHOT_DIR").unwrap_or("rust-stable".to_string()))
            .to_str()
            .unwrap()
            .to_string()
    }

    fn retrieve_output(output: &Vec<u8>) -> String {
        String::from_utf8_lossy(&output)
            .to_string()
            .lines()
            .filter(|s| !s.is_empty())
            .map(|line| match line.find("; finished in") {
                Some(idx) => &line[0..idx],
                None => line,
            })
            .sorted()
            .join("\n")
    }

    fn retrieve_stdout(output: &Output) -> String {
        retrieve_output(&output.stdout)
    }

    fn retrieve_stderr(output: &Output) -> String {
        retrieve_output(&output.stderr)
    }

    #[test]
    fn basic() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("basic"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            let lines = retrieve_stdout(&output);
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn hamcrest_assertions() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
        let output = Command::new("cargo")
                    .current_dir(PathBuf::from("acceptance_tests").join("hamcrest_assertions"))
                    .args(&["test"])
                    .output()
                    .expect("cargo command failed to start");

                let lines = retrieve_stdout(&output);
                insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn r#async() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("async"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            let lines = retrieve_stdout(&output);
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn return_check() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("return_check"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            assert!(!output.status.success());

            // We need to check stderr for the expected compile
            // failure, but that includes other volatile output, so
            // resort to manual inspection for now.
            let stderr = retrieve_stderr(&output);
            assert!(stderr.contains("Test function return_no_expect has a return-type but no exected clause in the test-case."));
        });
    }

    #[test]
    fn allow_return_check() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("return_check"))
                .args(&["test", "--features", "allow_return"])
                .output()
                .expect("cargo command failed to start");

            assert!(output.status.success());
        });
    }

}
