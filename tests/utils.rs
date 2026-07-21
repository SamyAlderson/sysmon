// tests/utils.rs

use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();

    for line in reader.lines() {
        let line = line?;
        contents.push_str(&line);
    }

    Ok(contents)
}

pub fn get_test_output_file_path(test_name: &str) -> String {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let test_output_file = format!("{}/{}.txt", out_dir, test_name);
    test_output_file
}

pub fn assert_file_contents_equals(file_path: &str, expected_contents: &str) -> bool {
    let actual_contents = read_file_contents(file_path).unwrap_or_else(|e| {
        eprintln!("Error reading file {}: {}", file_path, e);
        String::new()
    });

    let trimmed_actual_contents = actual_contents.trim();
    let trimmed_expected_contents = expected_contents.trim();

    trimmed_actual_contents == trimmed_expected_contents
}