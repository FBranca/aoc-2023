use std::collections::HashMap;
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
    let mut lines = BufReader::new(file).lines();

    let map_re = Regex::new("^([^ ]+) = \\(([^ ]+), ([^ ]+)\\)$").unwrap();
    let nav = lines.next().unwrap().unwrap();
    lines.next();

    let map = lines.fold(
        HashMap::<String, (String, String)>::new(),
        |mut map, line| {
            let line = line.unwrap();
            let cap = map_re.captures(&line).unwrap();

            let key = cap.get(1).unwrap().as_str().to_owned();
            let left = cap.get(2).unwrap().as_str().to_owned();
            let right = cap.get(3).unwrap().as_str().to_owned();
            map.insert(key, (left, right));

            map
        },
    );

    // First half
    let mut count = 0;
    let mut i = nav.chars().cycle();
    let mut cursor = "AAA".to_string();
    while cursor != "ZZZ" {
        count += 1;
        let dir = i.next().unwrap();
        let next = map.get(&cursor).unwrap();
        cursor = match dir {
            'L' => next.0.clone(),
            'R' => next.1.clone(),
            _ => panic!("invalid direction"),
        };
    }
    println!("count {}", count);

    // Second half
    let mut count: u64 = 0;
    let mut i = nav.chars().cycle();
    let mut cursors = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<&String>>();
    while !cursors
        .iter()
        .all(|cursor| cursor.ends_with('Z'))
    {
        count += 1;
        let dir = i.next().unwrap();
        //        println!("{:?}", cursors);
        cursors = cursors
            .iter()
            .map(|cursor| {
                let next = map.get(*cursor).unwrap();
                match dir {
                    'L' => &next.0,
                    'R' => &next.1,
                    _ => panic!("invalid direction"),
                }
            })
            .collect::<Vec<&String>>();
        if count % 1000000 == 0 {
            println!("{}", count);
        }
    }
    println!("{:?}", cursors);
    println!("count {}", count);

    Ok(())
}
