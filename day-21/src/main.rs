use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

type Vec2D = Vec<Vec<char>>;

struct Map2D {
    pub map: Vec2D,
    pub width: i32,
    pub height: i32,
}

impl Map2D {
    pub fn from(map: Vec2D) -> Self {
        let height = map.len() as i32;
        let width = map[0].len() as i32;
        Map2D { map, width, height }
    }

    pub fn clone_rocks(&self) -> Map2D {
        let map = self
            .map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|c| match c {
                        'O' => '.',
                        c => *c,
                    })
                    .collect()
            })
            .collect();

        Map2D {
            map,
            width: self.width,
            height: self.height,
        }
    }

    pub fn get(&self, pos: &(i32, i32)) -> Option<char> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.height && pos.1 < self.width {
            Some(self.map[pos.0 as usize][pos.1 as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: &(i32, i32), c: char) {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.height && pos.1 < self.width {
            self.map[pos.0 as usize][pos.1 as usize] = c;
        }
    }

    pub fn print(&self) {
        self.map.iter().for_each(|line| {
            println!("{}", line.iter().collect::<String>());
        })
    }
}

fn step(map: &Map2D) -> Map2D {
    let mut new_map = map.clone_rocks();
    map.map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if *c == 'O' {
                for ofs in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let pos = (y as i32 + ofs.0, x as i32 + ofs.1);
                    if let Some(c) = map.get(&pos) {
                        if c != '#' {
                            new_map.set(&pos, 'O');
                        }
                    }
                }
            }
        })
    });
    new_map
}

fn count(map: &Map2D) -> usize {
    map.map.iter().fold(0, |acc, line| {
        acc + line.iter().filter(|c| **c == 'O').count()
    })
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;

    let mut map = Map2D::from(
        BufReader::new(&file)
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec2D>(),
    );

    let start = map
        .map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| (*c == 'S').then_some((y, x)))
        })
        .map(|(y, x)| (y as i32, x as i32))
        .expect("No start position found");

    map.set(&start, 'O');

    for _ in 0..64 {
        map = step(&map);
        println!();
        map.print();
    }

    println!("Count = {}", count(&map));
    Ok(())
}
