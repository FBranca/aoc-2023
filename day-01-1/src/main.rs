use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"([0-9])").unwrap();

    let mut result = 0;
    for line in reader.lines() {
        let line = line?;
        let mut matches = re.find_iter(&line);
        let first = matches.next();
        let last = matches.last().or(first);

        let first_val = first.unwrap().as_str().parse::<i32>().unwrap();
        let last_val = last.unwrap().as_str().parse::<i32>().unwrap();
        let value = first_val * 10 + last_val;

        result += value;
    }

    println!("result: {}", result);

    Ok(())
}
