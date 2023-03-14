use argh::FromArgs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    let reader = BufReader::new(file);
    return Ok(Box::new(reader));
    // Ok(Box::new(BufReader::new(File::open(filename)?)))
}
