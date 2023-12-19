use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use geo::{Area, EuclideanLength, Polygon};
use itertools::Itertools;

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
    let mut reader = BufReader::new(file);

    // Find map size
    let vertices = (&mut reader)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            line.split_whitespace()
                .collect_tuple::<(&str, &str, &str)>()
                .map(|(_, _, color)| {
                    let count = i64::from_str_radix(&color[2..7], 16).unwrap();
                    match &color[7..8] {
                        "0" => (0, count),  // Right
                        "1" => (count, 0),  // Down
                        "2" => (0, -count), // Left
                        "3" => (-count, 0), // Up
                        _ => unreachable!(),
                    }
                })
        })
        .scan((0, 0), |pos, mv| {
            *pos = (pos.0 + mv.0, pos.1 + mv.1);
            Some((pos.0 as f64, pos.1 as f64))
        })
        .collect::<Vec<(f64, f64)>>();

    let poly = Polygon::new(vertices.into(), vec![]);

    println!(
        "Part 2: {}",
        poly.unsigned_area() + (poly.exterior().euclidean_length() / 2.0).trunc() + 1.0
    );

    Ok(())
}
