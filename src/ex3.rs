use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn find_lines_with_keyword(
    file_path: PathBuf,
    keyword: &str,
) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines_with_keyword = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(keyword) {
            lines_with_keyword.push(line);
        }
    }
    Ok(lines_with_keyword)
}

fn main() {
    let file_path = PathBuf::from("example.txt");
    let keyword = "Rust";
    let lines = find_lines_with_keyword(file_path, keyword).unwrap();
    for line in lines {
        println!("{}", line);
    }
}
