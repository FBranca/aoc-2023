use std::collections::HashMap;
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

type Cache = HashMap<(usize, usize, u32), u64>;

fn solve_line(
    h: &mut Cache,
    flat: &str,
    lst: &Vec<u32>,
    flat_i: usize,
    lst_i: usize,
    bsize: u32,
) -> u64 {
    let mark = (flat_i, lst_i, bsize);
    if let Some(res) = h.get(&mark) {
        return *res;
    }

    let mut res = 0;

    if flat_i == flat.len() {
        if bsize == 0 {
            res = (lst_i == lst.len()) as u64
        } else {
            res = (lst_i == (lst.len() - 1) && lst[lst_i] == bsize) as u64
        }
        return res;
    }

    let fc = flat.chars().nth(flat_i).unwrap();
    if fc == '.' || fc == '?' {
        if bsize == 0 {
            res += solve_line(h, flat, lst, flat_i + 1, lst_i, 0);
        } else if lst_i < lst.len() && lst[lst_i] == bsize {
            res += solve_line(h, flat, lst, flat_i + 1, lst_i + 1, 0);
        }
    }

    if fc == '#' || fc == '?' {
        res += solve_line(h, flat, lst, flat_i + 1, lst_i, bsize + 1);
    }

    h.insert(mark, res);
    res
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let sum = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let (flat, lst) = line.split_once(' ').unwrap();
            let lst = lst
                .split(',')
                .map(str::parse::<u32>)
                .filter_map(|r| r.ok())
                .collect::<Vec<u32>>();
            let res = solve_line(&mut Cache::new(), flat, &lst, 0, 0, 0);

            let flat2 = [flat, "?", flat, "?", flat, "?", flat, "?", flat]
                .iter()
                .fold(String::new(), |acc, c| acc + *c);
            let lst2 = lst.repeat(5);
            let res2 = solve_line(&mut Cache::new(), &flat2, &lst2, 0, 0, 0);

            (res, res2)
        })
        .fold((0_u64, 0_u64), |(sum1, sum2), (res1, res2)| {
            (sum1 + res1, sum2 + res2)
        });

    println!("Sum1: {}", sum.0);
    println!("Sum2: {}", sum.1);
    Ok(())
}
