use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    let re = Regex::new(&pattern).expect("Not a Valid re pattern");

    for line in reader.lines() {
        let line = line.expect("Error in reading the lines");
        match re.find(&line) {
            Some(_) => println!("Found at line: {}", line),
            None => continue,
        }
    }
}
