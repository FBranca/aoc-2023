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

fn find_next (values: &Vec<i64>) -> (i64, i64) {
    let count_nz = values.iter().fold(
        0, |acc, b| {
        if *b != 0 { acc + 1 } else { acc }
    });
    if count_nz == 0 {
        return (0, 0);
    }

    let len = values.len();
    let mut iter = values.iter();
    let prev = iter.next().unwrap();
    let v = iter.fold(
        (Vec::<i64>::with_capacity(len-1), prev),
        |(mut res, prev), v| {
            res.push(*v-*prev);
            (res, v)
        }
    );

    let inc = find_next (&v.0);

    (values.first().unwrap() - inc.0, v.1 + inc.1)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let lines = BufReader::new(file).lines();

    let sum = lines.fold((0, 0), |acc, line| {
        // Parse the line 
        let line = line.unwrap();
        let values = line
            .split_whitespace()
            .map(str::parse::<i64>)
            .filter_map(|v| v.ok())
            .collect();

        let (prev, next) = find_next(&values);
        (acc.0 + prev, acc.1 + next)
    });

    println!("Sum is {:?}", sum);
    Ok(())
}
