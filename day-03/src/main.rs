use core::ops::Range;
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

struct Gear {
    nb_parts: u32,
    product: u32,
}

fn is_symbol(c: &char) -> bool {
    (*c != '.') && !c.is_numeric()
}

fn have_symbol(range: &Range<usize>, prev: &str, line: &str, next: &str) -> bool {
    if range.start > 0 {
        if is_symbol(&prev.chars().nth(range.start - 1).unwrap())
            || is_symbol(&line.chars().nth(range.start - 1).unwrap())
            || is_symbol(&next.chars().nth(range.start - 1).unwrap())
        {
            return true;
        }
    }
    for i in range.start..range.end {
        if is_symbol(&prev.chars().nth(i).unwrap()) || is_symbol(&next.chars().nth(i).unwrap()) {
            return true;
        }
    }

    range.end < line.len()
        && (is_symbol(&prev.chars().nth(range.end).unwrap())
            || is_symbol(&line.chars().nth(range.end).unwrap())
            || is_symbol(&next.chars().nth(range.end).unwrap()))
}

fn add_gear(gears: &mut Vec<(usize, usize)>, l: usize, c: usize, line: &str) {
    if line.chars().nth(c).unwrap() == '*' {
        gears.push((l, c));
    }
}

fn have_gears(
    range: &Range<usize>,
    l: usize,
    prev: &str,
    line: &str,
    next: &str,
) -> Vec<(usize, usize)> {
    let mut gears = Vec::<(usize, usize)>::new();
    if range.start > 0 {
        add_gear(&mut gears, l - 1, range.start - 1, prev);
        add_gear(&mut gears, l, range.start - 1, line);
        add_gear(&mut gears, l + 1, range.start - 1, next);
    }
    for i in range.start..range.end {
        add_gear(&mut gears, l - 1, i, prev);
        add_gear(&mut gears, l + 1, i, next);
    }
    if range.end < line.len() {
        add_gear(&mut gears, l - 1, range.end, prev);
        add_gear(&mut gears, l, range.end, line);
        add_gear(&mut gears, l + 1, range.end, next);
    }
    gears
}

fn process_line(
    re: &Regex,
    l: usize,
    gears: &mut HashMap<(usize, usize), Gear>,
    connected_sum: &mut u32,
    prev: &str,
    line: &str,
    next: &str,
) {
    for cap in re.captures_iter(line) {
        let id = cap.get(0).unwrap();
        let range = id.range();
        let part_id = id.as_str().parse::<u32>().unwrap();
        if have_symbol(&range, prev, line, next) {
            *connected_sum += part_id;
        }
        let glist = have_gears(&range, l, prev, line, next);
        for gear_pos in glist {
            let g = gears.entry(gear_pos).or_insert_with(|| Gear {
                nb_parts: 0,
                product: 1,
            });
            g.nb_parts += 1;
            g.product *= part_id;
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut reader = BufReader::new(file);

    let mut result: u32 = 0;
    let mut gears = HashMap::<(usize, usize), Gear>::new();
    let re = Regex::new("([0-9]+)").unwrap();

    let mut l = 1; // line number
    let mut line: String = String::new();
    let _ = reader.read_line(&mut line);
    let mut prev = ".".repeat(line.len());
    for next in reader.lines() {
        let next = next.unwrap();
        process_line(&re, l, &mut gears, &mut result, &prev, &line, &next);

        prev = line;
        line = next;
        l += 1;
    }

    let next = ".".repeat(line.len());
    process_line(&re, l, &mut gears, &mut result, &prev, &line, &next);

    let mut product_sum = 0;
    for ((_, _), g) in gears {
        if g.nb_parts > 1 {
            product_sum += g.product;
        }
    }

    println!("Sum of parts (1st half) {}", result);
    println!("Product sum  (2nd half) {}", product_sum);
    Ok(())
}
