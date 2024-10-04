use std::{error::Error, fs, io::Write, path::PathBuf};

use ignore::Walk;
use regex::Regex;

// smthn like this. u need to add error handling
fn main() -> Result<(), std::io::Error>{
    let here = fs::canonicalize(".").unwrap();
    let re = Regex::new(r"(\.rs|Cargo.toml|\.md|\.wgsl|\.ts|\.tsx|\.py|pyproject.toml)$").unwrap();

    let files = find_files(here.clone(), re).unwrap();

    let mut res = String::new();
    for entry in files {
        let mut content = fs::read_to_string(entry.clone()).unwrap();
        if !content.ends_with('\n') {
            content.push('\n');
        }

        let relative_path = entry.strip_prefix(here.clone()).unwrap().to_str().unwrap();
        res.push_str(&format!("{}\n```rust\n{}```\n\n", relative_path, content));
    }

    // Write to file
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() >= 2 && args[1] == "c" {
        println!("{}", res);
    } else {
        let mut file = fs::File::create("output.txt").unwrap();
        file.write_all(res.as_bytes()).unwrap();
    }

    Ok(())
}


fn find_files(dir: PathBuf, regex: Regex) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in Walk::new(dir) {
        let entry = entry?.path().to_path_buf();
        if entry.is_dir() {
            continue;
        }

        if regex.is_match(entry.to_str().unwrap()) {
            files.push(entry);
        }

    }

    Ok(files)
}