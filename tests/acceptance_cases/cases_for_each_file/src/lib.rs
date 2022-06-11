#![cfg(test)]

#[test_case::for_each(file in "tests/data/")]
fn run_for_every_txt_in_path(file: &str) {
    let file_path = std::path::Path::new(file);
    let file_id: usize = file_path
        .file_stem()
        .expect("file_stem")
        .to_str()
        .expect("to_str")
        .parse()
        .expect("parse");
    let contents: usize = std::fs::read_to_string(file).expect("read_to_string").trim().parse().expect("parse");
    assert_eq!(contents, file_id);
}
