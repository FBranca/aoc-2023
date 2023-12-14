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

fn winning_range(time: u64, record: u64) -> (u64, u64) {
    /*
        searching for distance > record.
        distance = (hold_time) * (time - hold_time)

        so: record < hold_time * (time - hold_time)
            record < time*hold_time - hold_time^2

        Search the critical point:
            0 = hold_time^2 -time*hold_time + record
            hold_time = (time +/- sqrt(time^2 - 4*record)) / 2
    */
    let crit_inf: f64 = (time as f64 - (time.pow(2) as f64 - 4.0 * record as f64).sqrt()) / 2.0;
    let crit_sup: f64 = (time as f64 + (time.pow(2) as f64 - 4.0 * record as f64).sqrt()) / 2.0;

    let crit_inf = match crit_inf {
        v if v.ceil() == v => v.ceil() as u64 + 1,
        v => v.ceil() as u64,
    };
    let crit_sup = match crit_sup {
        v if v.trunc() == v => v.trunc() as u64 - 1,
        v => v.trunc() as u64,
    };

    (crit_inf, crit_sup)
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let file = File::open(args.input)?;
    let mut lines = BufReader::new(file).lines();

    let line_times = lines.next().unwrap().unwrap();
    let line_distances = lines.next().unwrap().unwrap();

    // 1st half of the day
    // extract as a vector of tuples (time, record)
    let races = line_times
        .split_whitespace()
        .skip(1)
        .map(str::parse::<u64>)
        .filter_map(|v| v.ok())
        .zip(
            line_distances
                .split_whitespace()
                .skip(1)
                .map(str::parse::<u64>)
                .filter_map(|v| v.ok()),
        )
        .collect::<Vec<(u64, u64)>>();
    println!(
        "{}",
        line_times
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
    );
    // 2nd half of the day
    let only_one_race = (
        line_times
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
        line_distances
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
    );

    let mut result = 1;
    for (time, record) in races {
        let (crit_inf, crit_sup) = winning_range(time, record);
        println!("{}  {}", crit_inf, crit_sup);
        result *= 1 + crit_sup - crit_inf;
    }

    println!("result for 1st star is {}", result);

    let (crit_inf, crit_sup) = winning_range(only_one_race.0, only_one_race.1);
    println!("{}  {}", crit_inf, crit_sup);

    println!("result for 2st star is {}", 1 + crit_sup - crit_inf);

    Ok(())
}
