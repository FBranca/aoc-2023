use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, EnumIter, Debug)]
enum Dir {
    East,
    South,
    North,
    West,
}

impl Dir {
    pub fn get_ofs(&self) -> (i32, i32) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }

    pub fn apply(&self, pos: &(usize, usize)) -> (i32, i32) {
        let ofs = self.get_ofs();
        (pos.0 as i32 + ofs.0, pos.1 as i32 + ofs.1)
    }

    #[allow(dead_code)]
    pub fn apply_inv(&self, pos: &(usize, usize)) -> (i32, i32) {
        let ofs = self.get_ofs();
        (pos.0 as i32 - ofs.0, pos.1 as i32 - ofs.1)
    }

    pub fn is_opposite(&self, dir: &Dir) -> bool {
        match *dir {
            Dir::North => *self == Dir::South,
            Dir::East => *self == Dir::West,
            Dir::South => *self == Dir::North,
            Dir::West => *self == Dir::East,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
struct Status {
    pub pos: (usize, usize),
    pub dir: Dir,
    pub dirlg: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let reader = BufReader::new(file);

    // Parse input
    let map = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut cost_map = HashMap::<Status, u32>::new();
    let mut heap = VecDeque::<(Status, u32)>::new();
    heap.push_back((
        Status {
            pos: (0, 0),
            dir: Dir::East,
            dirlg: 0,
        },
        0_u32,
    ));

    while let Some((status, dist)) = heap.pop_front() {
        match cost_map.entry(status.clone()) {
            Vacant(e) => {
                e.insert(dist);
            }
            Occupied(mut e) => {
                let d = e.get_mut();
                if dist < *d {
                    *d = dist;
                } else {
                    continue;
                }
            }
        };
        //        println!("{:?}  {:?}", status, dist);

        for dir in Dir::iter() {
            if dir.is_opposite(&status.dir) {
                continue;
            }
            let newpos = dir.apply(&status.pos);
            let dirlg = match &dir {
                d if *d == status.dir => status.dirlg + 1,
                _ => 1,
            };

            let pos_valid =
                (newpos.0 >= 0) && (newpos.0 < height) && (newpos.1 >= 0) && (newpos.1 < width);

            // Part 1:
            // let long_valid = (dirlg <= 3);

            // Part 2:
            let long_valid = (dirlg <= 10)
                && ((dir == status.dir) || (status.dirlg == 0) || (status.dirlg >= 4));

            if pos_valid && long_valid {
                let pos = (newpos.0 as usize, newpos.1 as usize);
                let cost = map[pos.0][pos.1];
                heap.push_back((Status { pos, dir, dirlg }, dist + cost));
            }
        }
    }

    let min_cost = cost_map
        .iter()
        .filter_map(|(status, dist)| {
            (status.pos == (height as usize - 1, width as usize - 1)).then_some((status, dist))
        })
        .map(|x| {
            println!("{:?}", x);
            x.1
        })
        .min();

    println!("{:?}", min_cost);

    Ok(())
}
