use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        // check if the line is valid utf8
        if let Ok(utf8_line) = std::str::from_utf8(line.as_bytes()) {
            // process the line as UTF-8
            println!("UTF-8 line: {}", utf8_line);
        } else {
            // convert the line to UTF-8 using a best-effort strategy
            let utf8_line = String::from_utf8_lossy(line.as_bytes());
            println!("Converted line: {}", utf8_line);
        }
    }

    Ok(())
}