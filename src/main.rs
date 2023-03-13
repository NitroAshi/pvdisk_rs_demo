use argh::FromArgs;

#[derive(FromArgs)]
/// pvdisk_rs
struct Args {
    /// design name
    #[argh(option, short = 'd')]
    design: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();
    println!("{:#?}", args.design);
    let design = args.design.unwrap();
    println!("{}", design);
}
