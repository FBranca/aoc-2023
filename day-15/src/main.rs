use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use indexmap::IndexMap;

type Box = IndexMap<String, u8>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| {
        let ascii = c as u8;
        17 * (acc + ascii as u64) % 256
    })
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut lines = BufReader::new(file).lines();
    let input = lines.next().unwrap().unwrap();
    let sum: u64 = input
        .split(',')
        .map(|s| {
            let h = hash(s);
            println!("hash({}) = {}", s, h);
            h
        })
        .sum();

    println!("sum: {}", sum);

    let mut boxes = std::iter::repeat(Box::new())
        .take(256)
        .collect::<Vec<Box>>();

    input.split(',').for_each(|s| {
        let operation = s.find(['-', '=']).unwrap();
        let key = &s[0..operation];
        let h = hash(key) as usize;
        let b = boxes.get_mut(h).unwrap();

        match s.chars().nth(operation).unwrap() {
            '-' => {
                b.shift_remove(key);
            }
            '=' => {
                let value = s[operation + 1..]
                    .parse::<u8>()
                    .expect("part after operation should be numeric");
                b.insert(key.to_string(), value);
            }
            _ => panic!("invalid operation"),
        }
    });

    let result: u64 = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, v)| {
            v.iter()
                .enumerate()
                .map(|(lens_idx, (_, focal))| {
                    (box_idx as u64 + 1) * (lens_idx as u64 + 1) * *focal as u64
                })
                .sum::<u64>()
        })
        .sum();

    println!("Part 2: {}", result);

    Ok(())
}
