#![cfg(test)]

use insta::with_settings;
use itertools::Itertools;
use std::env;
use std::path::PathBuf;
use std::process::Command;

macro_rules! run_integration_test {
    ($case_name:expr, $cmd:expr) => {
        with_settings!({snapshot_path => get_snapshot_directory()}, {
            let subcommand = Command::new("cargo")
                .current_dir(PathBuf::from("tests").join("integration_test_cases").join($case_name))
                .args(&[$cmd])
                .output()
                .expect("Failed to spawn cargo subcommand");

            let mut output = String::new();
            output.push_str(String::from_utf8_lossy(&subcommand.stdout).as_ref());
            output.push_str(String::from_utf8_lossy(&subcommand.stderr).as_ref());

            let output = sanitize_lines(output);

            insta::assert_display_snapshot!(output);
        })
    };
    ($case_name:expr) => {
        run_integration_test!($case_name, "test")
    }
}

fn get_snapshot_directory() -> String {
    PathBuf::from("snapshots")
        .join(env::var("SNAPSHOT_DIR").unwrap_or_else(|_| "rust-stable".to_string()))
        .to_str()
        .unwrap()
        .to_string()
}

fn sanitize_lines(s: String) -> String {
    let mut s = s
        .lines()
        .filter(|line| {
            !line.contains("note")
                && !line.contains("error")
                && !line.contains("waiting")
                && !line.contains("Finished")
                && !line.contains("Compiling")
                && !line.contains("termination value with a non-zero status code")
                && !line.contains("Running unittests")
                && !line.contains("Running target")
                && !line.is_empty()
        })
        .map(|line| line.replace('\\', "/"))
        .map(|line| line.replace(".exe", ""))
        .collect::<Vec<_>>();

    s.sort_unstable();

    s.into_iter().join("\n")
}

#[test]
fn basic() {
    run_integration_test!("basic")
}

#[test]
fn async_tests() {
    run_integration_test!("async_tests")
}

#[test]
fn test_item_reuse() {
    run_integration_test!("test_item_reuse")
}

#[test]
fn test_item_reuse_run() {
    run_integration_test!("test_item_reuse", "run")
}

#[test]
fn test_result() {
    run_integration_test!("result")
}
