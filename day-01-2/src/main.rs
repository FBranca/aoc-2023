use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

fn digitize(value: &str) -> Option<i32> {
    match value {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        other => other.parse::<i32>().ok(),
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let reader = BufReader::new(file);

    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9", "0",
    ];
    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();

    let mut result = 0;
    for line in reader.lines() {
        let line = line?;

        let mut iter = ac.find_overlapping_iter(&line);
        let first = iter.next().unwrap();
        let last = iter.last().unwrap_or(first);

        let first_val = &line[first.start()..first.end()];
        let last_val = &line[last.start()..last.end()];
        let value = digitize(first_val).unwrap() * 10 + digitize(last_val).unwrap();

        result += value;
    }

    println!("result: {}", result);

    Ok(())
}
