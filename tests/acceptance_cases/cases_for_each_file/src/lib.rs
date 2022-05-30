#![cfg(test)]

use std::fs;
use std::path::Path;

#[test_case::for_each(file in "tests/data/")]
fn run_for_every_txt_in_path(file: &str) {
    let file_path = Path::new(file);
    let file_id: usize = file_path.file_stem().unwrap().to_str().unwrap().parse().unwrap();
    let contents: usize = fs::read_to_string(file).unwrap().trim().parse().unwrap();
    assert_eq!(contents, file_id);
}
