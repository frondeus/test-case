#![cfg(test)]

mod acceptance {
    use std::path::PathBuf;
    use std::process::Command;

    fn run_tests() -> String {
        ["basic", "hamcrest_assertions"]
            .iter()
            .map(|feature| {
                let output = Command::new("cargo")
                    .current_dir(PathBuf::from("acceptance_tests").join(feature))
                    .args(&["test"])
                    .output()
                    .expect("cargo command failed to start");

                String::from_utf8_lossy(&output.stdout).to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn runs_all_tests() {
        let actual = run_tests();
        let mut lines: Vec<_> = actual.lines().collect();
        lines.sort();
        let lines: String = lines.join("\n");
        insta::assert_display_snapshot!(lines);
    }
}
