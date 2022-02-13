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

    fn retrieve_stderr(output: &Output) -> String {
        String::from_utf8_lossy(&output.stderr)
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

    fn sanitize_lines(s: String) -> String {
        s.lines()
            .filter(|line| {
                !line.contains("note")
                    && !line.contains("error")
                    && !line.contains("waiting")
                    && !line.contains("Finished")
                    && !line.contains("Compiling")
                    && !line.contains("termination value with a non-zero status code")
            })
            .map(|line| line.replace('\\', "/"))
            .map(|line| line.replace(".exe", "")) // remove executable extension on windows
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

            let lines = sanitize_lines(retrieve_stdout(&output));
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

            let lines = sanitize_lines(retrieve_stdout(&output));
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

            let lines = sanitize_lines(retrieve_stdout(&output));
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn test_item_reuse_run() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("test_item_reuse"))
                .args(&["run"])
                .output()
                .expect("cargo command failed to start");

            let mut lines = retrieve_stdout(&output);
            lines.push_str(&retrieve_stderr(&output));
            let lines = sanitize_lines(lines);
            insta::assert_display_snapshot!(lines);
        });
    }

    #[test]
    fn test_result() {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let output = Command::new("cargo")
                .current_dir(PathBuf::from("acceptance_tests").join("result"))
                .args(&["test"])
                .output()
                .expect("cargo command failed to start");

            let lines = sanitize_lines(retrieve_stdout(&output));
            insta::assert_display_snapshot!(lines);
        });
    }
}
