#![cfg(test)]

use itertools::Itertools;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn assert_display_snapshot(name: &str, actual: &str) {
    let expected_file = get_snapshot_directory().join(name).with_extension("snap");
    let expected = std::fs::read_to_string(&expected_file).unwrap_or_else(|_| {
        panic!(
            "Failed to read snapshot file {}",
            expected_file.to_string_lossy()
        )
    });

    if expected != actual {
        std::fs::write(
            get_snapshot_directory().join(name).with_extension("actual"),
            actual,
        )
        .expect("Failed to write actual snapshot file");

        panic!("Expected and actual differ");
    }
}

fn run_acceptance_test_body(cmd: &str, name: &str, snap_name: &str) {
    let subcommand = Command::new("cargo")
        .current_dir(PathBuf::from("tests").join("acceptance_cases").join(name))
        .args([cmd])
        .output()
        .expect("Failed to spawn cargo subcommand");

    let mut actual = String::new();
    actual.push_str(String::from_utf8_lossy(&subcommand.stdout).as_ref());
    actual.push_str(String::from_utf8_lossy(&subcommand.stderr).as_ref());

    let actual = sanitize_lines(actual);

    assert_display_snapshot(snap_name, &actual);
}

macro_rules! run_acceptance_test {
    ($cmd:expr, $run_dir:expr, $snap_name:expr) => {
        run_acceptance_test_body($cmd, $run_dir, $snap_name)
    };
    ($run_dir:expr) => {
        run_acceptance_test!("test", $run_dir, $run_dir)
    };
}

fn get_snapshot_directory() -> PathBuf {
    PathBuf::from("tests/snapshots")
        .join(env::var("SNAPSHOT_DIR").unwrap_or_else(|_| "rust-stable".to_string()))
}

fn sanitize_lines(s: String) -> String {
    let re_time = Regex::new(r"\d+\.\d{2}s").expect("Building regex");

    let mut s = s
        .lines()
        .filter(|line| {
            !line.contains("note")
                && !line.contains("error: build failed") // For mac builds
                && !line.contains("error: process didn't exit successfully") // For windows builds
                && !line.contains("waiting")
                && !line.contains("Finished")
                && !line.contains("Compiling")
                && !line.contains("termination value with a non-zero status code")
                && !line.contains("Running unittests")
                && !line.contains("Running target")
                && !line.contains("Downloaded")
                && !line.contains("Downloading")
                && !line.contains("Updating")
                && !line.is_empty()
        })
        .map(|line| line.replace('\\', "/"))
        .map(|line| line.replace(".exe", ""))
        .map(|line| re_time.replace_all(&line, "0.00s").to_string())
        .collect::<Vec<_>>();

    s.sort_unstable();

    s.into_iter().join("\n")
}

#[test]
fn cases_can_be_declared_on_async_methods() {
    run_acceptance_test!("cases_can_be_declared_on_async_methods")
}

#[test]
fn cases_can_be_declared_on_non_test_items() {
    run_acceptance_test!("cases_can_be_declared_on_non_test_items")
}

#[test]
fn cases_declared_on_non_test_items_can_be_used() {
    run_acceptance_test!(
        "run",
        "cases_can_be_declared_on_non_test_items",
        "cases_declared_on_non_test_items_can_be_used"
    )
}

#[test]
fn cases_can_be_ignored() {
    run_acceptance_test!("cases_can_be_ignored")
}

#[test]
fn cases_can_panic() {
    run_acceptance_test!("cases_can_panic")
}

#[test]
fn cases_can_return_result() {
    run_acceptance_test!("cases_can_return_result")
}

#[test]
fn cases_support_basic_features() {
    run_acceptance_test!("cases_support_basic_features")
}

#[test]
fn cases_support_complex_assertions() {
    run_acceptance_test!("cases_support_complex_assertions")
}

#[test]
fn cases_support_generics() {
    run_acceptance_test!("cases_support_generics")
}

#[test]
fn cases_support_keyword_using() {
    run_acceptance_test!("cases_support_keyword_using")
}

#[test]
fn cases_support_keyword_with() {
    run_acceptance_test!("cases_support_keyword_with")
}

#[test]
fn cases_support_multiple_calling_methods() {
    run_acceptance_test!("cases_support_multiple_calling_methods")
}

#[test]
fn cases_support_pattern_matching() {
    run_acceptance_test!("cases_support_pattern_matching")
}

#[test]
fn cases_can_use_regex() {
    run_acceptance_test!("cases_can_use_regex")
}

#[test]
fn features_produce_human_readable_errors() {
    run_acceptance_test!("features_produce_human_readable_errors")
}
