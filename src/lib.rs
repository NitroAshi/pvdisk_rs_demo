use argh::FromArgs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;


#[derive(FromArgs, PartialEq)]
/// pvdisk_rs
pub struct CliInputs {
    /// design name
    #[argh(option, short = 'D')]
    pub design: Option<String>,

    /// file name
    #[argh(positional, short = 'f')]
    pub filename: String,
}

pub fn read_file(filename: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    let file = File::open(filename)?;
    // let reader = BufReader::new(file);
    let reader = BufReader::new(DecodeReaderBytesBuilder::new().encoding(Some(UTF_8)).build(file));
    return Ok(Box::new(reader));
    // Ok(Box::new(BufReader::new(File::open(filename)?)))
}

pub fn find_keyword(filename: &str, keyword: &str) -> Result<Vec<String>, std::io::Error> {
    let reader = read_file(filename).unwrap();
    let mut lines_with_keyword = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(keyword) {
            lines_with_keyword.push(line);
        }
    }
    return Ok(lines_with_keyword);
}
