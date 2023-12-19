use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use itertools::Itertools;
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
}

impl From<char> for Dir {
    fn from(c: char) -> Dir {
        match c {
            'U' => Dir::North,
            'D' => Dir::South,
            'L' => Dir::West,
            'R' => Dir::East,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
struct Status {
    pub pos: (usize, usize),
    pub dir: Dir,
    pub dirlg: usize,
}

fn move_pos(pos: &(i32, i32), dir: &Dir, steps: i32) -> (i32, i32) {
    let ofs = dir.get_ofs();
    (pos.0 + ofs.0 * steps, pos.1 + ofs.1 * steps)
}

fn print_map_to_file(map: &[Vec<char>], filename: &str) {
    let mut f = std::fs::File::create(filename).unwrap();
    map.iter().for_each(|line| {
        line.iter().for_each(|c| {
            write!(f, "{}", c).unwrap();
        });
        writeln!(f).unwrap();
    });
}

fn delimit(map: &mut Vec<Vec<char>>) {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut to_visit = VecDeque::<(i32, i32)>::new();
    for y in 0..height {
        to_visit.push_back((y, 0));
        to_visit.push_back((y, width - 1));
    }
    for x in 0..width {
        to_visit.push_back((0, x));
        to_visit.push_back((height - 1, x));
    }

    while let Some(pos) = to_visit.pop_front() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as i32 || pos.1 >= map[0].len() as i32 {
            continue;
        }

        let c = map
            .get_mut(pos.0 as usize)
            .unwrap()
            .get_mut(pos.1 as usize)
            .unwrap();
        if *c == ' ' {
            *c = '-';

            to_visit.push_back((pos.0 - 1, pos.1));
            to_visit.push_back((pos.0 + 1, pos.1));
            to_visit.push_back((pos.0, pos.1 - 1));
            to_visit.push_back((pos.0, pos.1 + 1));
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut reader = BufReader::new(file);

    // Find map size
    let tr_bl = (&mut reader)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .collect_tuple::<(&str, &str, &str)>()
                .map(|(dir, steps, color)| {
                    (
                        Dir::from(dir.chars().next().unwrap()),
                        steps.parse::<i32>().unwrap(),
                        color.to_string(),
                    )
                })
                .unwrap()
        })
        .scan((0, 0), |pos, (dir, steps, _color)| {
            *pos = move_pos(pos, &dir, steps);
            println!("{:?}", pos);
            Some(*pos)
        })
        .fold((0, 0, 0, 0), |acc, pos| {
            (
                acc.0.min(pos.0),
                acc.1.min(pos.1),
                acc.2.max(pos.0),
                acc.3.max(pos.1),
            )
        });

    let (height, width) = (1 + tr_bl.2 - tr_bl.0, 1 + tr_bl.3 - tr_bl.1);
    let start_pos = (-tr_bl.0, -tr_bl.1);

    println!(
        "Top: {}, Right: {}, Bottom: {}, Left: {}, Height: {}, Width: {}",
        tr_bl.0, tr_bl.1, tr_bl.2, tr_bl.3, height, width
    );

    // Alloc map
    let mut map = Vec::<Vec<char>>::with_capacity(height as usize);
    (0..height as usize).for_each(|_| {
        map.push(
            std::iter::repeat(' ')
                .take(width as usize)
                .collect::<Vec<char>>(),
        )
    });

    // Restart a the beginning of the file to draw the path
    reader.seek(std::io::SeekFrom::Start(0))?;

    // Parse input
    let _ = (&mut reader)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .collect_tuple::<(&str, &str, &str)>()
                .map(|(dir, steps, color)| {
                    (
                        Dir::from(dir.chars().next().unwrap()),
                        steps.parse::<i32>().unwrap(),
                        color.to_string(),
                    )
                })
                .unwrap()
        })
        .fold(start_pos, |pos, (dir, steps, _color)| {
            let ofs = dir.get_ofs();
            let mut p = pos;

            (0..steps as usize).for_each(|_| {
                p = (p.0 + ofs.0, p.1 + ofs.1);
                map[p.0 as usize][p.1 as usize] = '#';
                println!("{:?}", p);
            });
            p
        });

    print_map_to_file(&map, "map_rebuilt.txt");
    delimit(&mut map);
    print_map_to_file(&map, "map_with_lava.txt");

    let count = map.iter().fold(0, |acc, v| {
        acc + v.iter().fold(0_usize, |acc, c| acc + (*c != '-') as usize)
    });

    println!("Part 1: {}", count);

    Ok(())
}
