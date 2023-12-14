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

#[derive(Default, Debug)]
struct CubeCnt {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let reader = BufReader::new(file);

    let re_start = Regex::new(r"Game ([0-9]+):").unwrap();
    let re_revealed = Regex::new(r"([:;,]) *([0-9]+) +(green|red|blue)").unwrap();

    let mut result_first_half = 0;
    let mut result_second_half = 0;
    for line in reader.lines() {
        let line = line?;

        let mut cubes = CubeCnt::default();

        let game = re_start
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let matches = re_revealed.captures_iter(&line);
        for m in matches {
            let (_, [_sep, qte, color]) = m.extract();
            let qte = qte.parse::<i32>().unwrap();
            match color {
                "red" => cubes.red = cubes.red.max(qte),
                "green" => cubes.green = cubes.green.max(qte),
                "blue" => cubes.blue = cubes.blue.max(qte),
                _ => panic!("unexpected color"),
            }
        }
        if cubes.blue <= 14 && cubes.red <= 12 && cubes.green <= 13 {
            result_first_half += game;
        }
        let power = cubes.blue * cubes.red * cubes.green;
        result_second_half += power;
    }

    println!("result_first_half: {}", result_first_half);
    println!(
        "result_second_half (sum of powers of each cube set): {}",
        result_second_half
    );

    Ok(())
}
