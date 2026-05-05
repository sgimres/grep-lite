use std::{fs::File, io::Read};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short)]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let pattern = args.pattern;
    let file_name = args.file;
    let mut file = File::open(file_name).expect("File not found");

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.contains(&pattern) {
        println!("Pattren {} is found on given String", &pattern);
    } else {
        println!("Pattern Not found");
    }
}
