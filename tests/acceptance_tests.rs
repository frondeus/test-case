#![cfg(test)]

mod acceptance {
    use insta::with_settings;
    use itertools::Itertools;
    use std::env;
    use std::path::PathBuf;
    use std::process::{Command, Output};

    fn get_snapshot_directory() -> String {
        PathBuf::from("snapshots")
            .join(env::var("SNAPSHOT_DIR").unwrap_or_else(|_| "rust-stable".to_string()))
            .to_str()
            .unwrap()
            .to_string()
    }

    fn retrieve_stdout(output: &Output) -> String {
        String::from_utf8_lossy(&output.stdout)
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
    fn async_tests() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("async_tests"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            let lines = retrieve_stdout(&output);
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn test_item_reuse() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("test_item_reuse"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            let lines = retrieve_stdout(&output);
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn test_item_reuse_build() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let status = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("test_item_reuse"))
                .args(&["build"])
                .status()
                .expect("cargo command failed to start");

            assert!(status.success())
        });
    }
}
