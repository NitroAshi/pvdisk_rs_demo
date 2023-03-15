use std::fs::File;
use std::io::{BufRead, BufReader};

fn search_keyword_in_file(filename: &str, keyword: &str) -> Option<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return None, // Return early if file cannot be opened
    };
    let reader = BufReader::new(file);
    let mut line_number = 0;
    for line in reader.lines() {
        line_number += 1;
        if let Ok(line) = line {
            if line.contains(keyword) {
                return Some(line);
            }
        } else {
            eprintln!("Error reading line {} from file", line_number);
        }
    }
    None // Keyword not found in file
}

fn main() {
    let filename = "example.txt";
    let keyword = "rust";
    match search_keyword_in_file(filename, keyword) {
        Some(line) => println!(
            "Found keyword '{}' in file '{}': {}",
            keyword, filename, line
        ),
        None => println!("Keyword '{}' not found in file '{}'", keyword, filename),
    }
}
