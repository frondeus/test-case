use std::process::exit;

fn main() {
    match version_check::is_min_version("1.29") {
        Some(true) => {}
        _ => {
            // rustc version too small or can't figure it out
            eprintln!("rustc>=1.29 is required due to feature(proc_macro) stabilitation");
            exit(1);
        }
    }
}
