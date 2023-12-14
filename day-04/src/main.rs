use std::collections::VecDeque;
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

    let re = Regex::new("Card +([0-9]+):([^\\|]+)\\|([^\\|]+)$").unwrap();

    let mut sum = 0;
    let mut copies: VecDeque<usize> = VecDeque::new();
    let mut copy_nb = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let cap = re.captures(&line);
        let cap = match cap {
            Some(cap) => cap,
            None => {
                println!("{:?}", line);
                panic!();
            }
        };

        // Get the winning cards list
        let winnings = cap
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(str::parse::<u32>)
            .filter_map(|v| v.ok())
            .collect::<Vec<u32>>();

        // Count card that are in the winning list
        let mines = cap
            .get(3)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(str::parse::<u32>)
            .filter_map(|v| v.ok())
            .filter(|v| winnings.contains(v))
            .count();

        // First half : sum (2 ^ count)
        let value = match mines {
            0 => 0,
            _ => 2_u32.pow(mines as u32 - 1),
        };
        sum += value;

        // Second Half : count total scratchcards sets
        let nb = 1 + copies.pop_front().unwrap_or(0);
        copy_nb += nb;
        if copies.len() < mines {
            copies.resize(mines, 0);
        }
        copies.range_mut(0..mines).for_each(|n| *n += nb);
        println!(
            "{} {} {} ({:?})",
            cap.get(1).unwrap().as_str(),
            mines,
            nb,
            copies
        );
    }

    println!("1st half: {:?}", sum);
    println!("2nd half: {:?}", copy_nb);

    Ok(())
}
