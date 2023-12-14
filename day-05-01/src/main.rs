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

fn init_seeds (line: &str) -> Vec<u64> {
    let re_seeds = Regex::new("seeds: ([ 0-9]+)").unwrap();

    re_seeds.captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(str::parse::<u64>)
        .filter_map(|v| v.ok())
        .collect::<Vec<u64>>()
}

fn merge (txt: &str, input: &Vec<u64>, output: &Vec<Option<u64>>) -> Vec<u64> {
    let ret = input
        .iter()
        .zip(output.iter())
        .map(|(inp, outp)| outp.unwrap_or(*inp))
        .collect::<Vec<u64>>();

    println! ("{}: {:?}", txt, ret);
    ret
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut lines = BufReader::new(file).lines();

    let re_head = Regex::new("(.+)-to-(.+) map:").unwrap();
    let re_range = Regex::new("([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

    let mut input = init_seeds(lines.next().unwrap().unwrap().as_ref());
    let mut output: Vec<Option<u64>> = input.iter().map(|v| Some(*v)).collect::<Vec<Option<u64>>>();

    for line in lines {
        let line = line.unwrap();

        if let Some(cap) = re_head.captures(&line) {
            // begin a new translation ruleset
            input = merge(
                cap.get(1).unwrap().as_str(),
                &input, &output);
            output = vec![Option::<u64>::None; input.len()];
        }
        else if let Some(cap) = re_range.captures(&line) {
            let dst_start = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let src_start = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let len = cap.get(3).unwrap().as_str().parse::<u64>().unwrap();

            output = output
                .iter()
                .zip(input.iter())
                .map(|(outp, inp)| {
                    if (*inp >= src_start) && (*inp < src_start+len) {
                        Some(*inp - src_start + dst_start)
                    } else {
                        *outp
                    }
                })
                .collect::<Vec<Option<u64>>>();
        }
    }

    let location = merge(
        "location",
        &input, &output);

    println! ("lowest location: {}", location.iter().min().unwrap());

    Ok(())
}
