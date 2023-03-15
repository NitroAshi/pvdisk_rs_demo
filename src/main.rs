// mod cli;
use pvdisk_rs_demo::*;
// use std::io::BufRead;

fn main() {
    let inputs: CliInputs = argh::from_env();
    let design: String = inputs.design.unwrap();
    // println!("{}", design);
    let filename: String = inputs.filename;
    let lines = find_keyword(&filename, &design).unwrap();
    for line in lines {
        println!("{}", line);
    }
    // let reader = read_file(&filename).unwrap();
    // for line in reader.lines() {
    //     println!("{}", line.unwrap());
    // }
}
