use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use itertools::Itertools;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

// Returns a list of range ( vec of tuple (start, end, processed) )
fn init_seeds(line: &str) -> Vec<(i64, i64, bool)> {
    let re_seeds = Regex::new("seeds: ([ 0-9]+)").unwrap();

    re_seeds
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(str::parse::<i64>)
        .filter_map(|v| v.ok())
        .tuples()
        .map(|(start, len)| (start, start + len, false))
        .collect::<Vec<(i64, i64, bool)>>()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut lines = BufReader::new(file).lines();

    let re_head = Regex::new("(.+)-to-(.+) map:").unwrap();
    let re_range = Regex::new("([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

    let mut input = init_seeds(lines.next().unwrap().unwrap().as_ref());

    for line in lines {
        let line = line.unwrap();

        if let Some(cap) = re_head.captures(&line) {
            // begin a new translation ruleset
            println!("{} status {:?}", cap.get(1).unwrap().as_str(), input);
            input.iter_mut().for_each(|t| t.2 = false);
        } else if let Some(cap) = re_range.captures(&line) {
            let dst_start = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let src_start = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let len = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let src_end = src_start + len;
            let ofs: i64 = dst_start - src_start;

            input = input
                .iter()
                .flat_map(|(start, end, processed)| {
                    /*
                                [---- src ----[
                       [---[  [---[  [---[  [---[  [---[
                         A      B      C      D      E
                             [--------------------[
                                       F
                    */
                    if (*start >= src_end) || (*end <= src_start) || *processed {
                        //   Case E        ||      Case A
                        vec![(*start, *end, *processed)].into_iter()
                    } else if *start >= src_start {
                        if *end <= src_end {
                            // Case C, the whole range is envolved
                            vec![(*start + ofs, *end + ofs, true)].into_iter()
                        } else {
                            // Case D, split on the end
                            vec![(*start + ofs, src_end + ofs, true), (src_end, *end, false)]
                                .into_iter()
                        }
                    } else if *end <= src_end {
                        // Case B, split on the start
                        vec![
                            (*start, src_start, false),
                            (src_start + ofs, *end + ofs, true),
                        ]
                        .into_iter()
                    } else {
                        // Case F, 3 split (on the start & on the end)
                        vec![
                            (*start, src_start, false),
                            (src_start + ofs, src_end + ofs, true),
                            (src_end, *end, false),
                        ]
                        .into_iter()
                    }
                })
                .collect::<Vec<(i64, i64, bool)>>();
        }
    }

    println!("locations {:?}", input);

    println!("lowest location: {:?}", input.iter().min().unwrap());

    Ok(())
}
