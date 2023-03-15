use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn search_file(filename: &str, keyword: &str) -> io::Result<Box<dyn Iterator<Item = String>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter(|line| line.as_ref().map(|l| l.contains(keyword)).unwrap_or(false));
    let result = Box::new(lines.flat_map(|line| {
        let line_num = line.as_ref().map_or(0, |_| 1);
        let mut prev_lines = Vec::new();
        let mut next_lines = Vec::new();
        for (i, l) in reader.lines().enumerate() {
            match l {
                Ok(l) if i < line_num - 1 => {
                    prev_lines.push(l);
                    if prev_lines.len() > 2 {
                        prev_lines.remove(0);
                    }
                }
                Ok(l) if i > line_num + 1 => {
                    next_lines.push(l);
                    if next_lines.len() > 2 {
                        next_lines.remove(0);
                    }
                }
                _ => (),
            }
            if let Ok(l) = l {
                if l.contains(keyword) {
                    return prev_lines
                        .into_iter()
                        .chain(Some(line.unwrap()))
                        .chain(next_lines.into_iter());
                }
            }
        }
        None
    }));
    Ok(result)
}

fn main() {
    let result = search_file("test.txt", "keyword").unwrap();
    for line in result {
        println!("{}", line);
    }
}
