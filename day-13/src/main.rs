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

fn diff(l1: &str, l2: &str) -> u32 {
    l1.chars()
        .zip(l2.chars())
        .fold(0, |acc, (c1, c2)| acc + (c1 != c2) as u32)
}

fn is_symmetric(p: &Vec<String>, i: usize, j: usize, d: u32, smudge: u32) -> bool {
    // println!("{} / {}  {} / {}",i,j,p[i],p[j]);
    let d = diff(&p[i], &p[j]) + d;
    if d <= smudge {
        if (i == 0) || (j == p.len() - 1) {
            d == smudge
        } else {
            is_symmetric(p, i - 1, j + 1, d, smudge)
        }
    } else {
        false
    }
}

fn find_symmetry(p: &Vec<String>, smudge: u32) -> Option<usize> {
    (1..p.len()).find(|&i| is_symmetric(p, i - 1, i, 0, smudge))
}

fn swap(p: &Vec<String>) -> Vec<String> {
    let width = p.len();
    let height = p[0].len();

    let mut res = Vec::<String>::with_capacity(height);
    for y in 0..height {
        let mut s = String::with_capacity(width);
        for line in p.iter().take(width) {
            s.push(line.chars().nth(y).unwrap());
        }
        res.push(s);
    }

    res
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    for smudge in 0..2 {
        let file = File::open(&args.input)?;
        let mut lines = BufReader::new(file).lines();

        let mut sum = 0;
        loop {
            let p = lines
                .by_ref()
                .filter_map(|r| r.ok())
                .take_while(|line| !line.is_empty())
                .collect::<Vec<String>>();

            if p.is_empty() {
                break;
            }

            let sym = find_symmetry(&p, smudge);
            if let Some(pos) = sym {
                sum += pos * 100;
            } else {
                let swapped = swap(&p);
                let pos =
                    find_symmetry(&swapped, smudge).expect("should have one axis of symmetry");
                sum += pos;
            }
        }

        println!("Sum (smudge {}): {}", smudge, sum);
    }

    Ok(())
}
