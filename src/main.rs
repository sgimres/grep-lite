use std::{fs::File, io::Read};

use clap::Parser;
use regex::Regex;

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

    let re = Regex::new(&pattern).expect("Not a Valid re pattern");
    let matches = re.find(&content);

    match matches {
        Some(a) => println!("Pattern is found on {:?}", a),
        None => println!("No matches are found"),
    }

    // if content.contains(&pattern) {
    //     println!("Pattren {} is found on given String", &pattern);
    // } else {
    //     println!("Pattern Not found");
    // }
}
