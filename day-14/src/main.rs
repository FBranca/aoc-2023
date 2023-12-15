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

pub struct Map2D {
    content: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Map2D {
    fn from_file(name: &str) -> Result<Self, std::io::Error> {
        let mut content = Vec::<char>::new();
        let mut height = 0;
        let mut width = 0;

        let file = File::open(name)?;
        BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .for_each(|line| {
                height += 1;
                width = line.len();
                content.extend(line.chars());
            });

        Ok(Map2D {
            content,
            width,
            height,
        })
    }

    fn clone(&self) -> Map2D {
        Map2D {
            content: self.content.clone(),
            width: self.width,
            height: self.height,
        }
    }

    fn at(&self, pos: (usize, usize)) -> char {
        let ofs = pos.0 * self.width + pos.1;
        self.content[ofs]
    }

    fn set(&mut self, pos: (usize, usize), c: char) {
        let ofs = pos.0 * self.width + pos.1;
        self.content[ofs] = c;
    }

    #[allow(dead_code)]
    fn rotate_anticlockwise(&self) -> Map2D {
        let mut content = Vec::<char>::with_capacity(self.content.len());
        let width = self.height;
        let height = self.width;

        for y in 0..height {
            for x in 0..width {
                // 12      246
                // 34  =>  135
                // 56
                let from_pos = (height - 1 - y) + x * width;
                content.push(self.content[from_pos]);
            }
        }

        Map2D {
            content,
            width,
            height,
        }
    }

    fn rotate_clockwise(&self) -> Map2D {
        let mut content = Vec::<char>::with_capacity(self.content.len());
        let width = self.height;
        let height = self.width;

        for y in 0..height {
            for x in 0..width {
                // 12      531
                // 34  =>  642
                // 56
                // (y, w-1-x) <=> (x, y)
                let from_pos = (width - 1 - x) * height + y;
                content.push(self.content[from_pos]);
            }
        }

        Map2D {
            content,
            width,
            height,
        }
    }

    fn equal(&self, other: &Map2D) -> bool {
        self.content.eq(&other.content)
    }

    fn print(&self) {
        (0..self.height).for_each(|y| {
            let line = self.content[y * self.width..(y + 1) * self.width]
                .iter()
                .collect::<String>();
            println!("{}", line);
        });
    }
}

fn stack_north(map: &mut Map2D) {
    for x in 0..map.width {
        let mut ys = 0;
        for y in 1..map.height {
            let c = map.at((y, x));
            match c {
                'O' => {
                    while ys < y && map.at((ys, x)) != '.' {
                        ys += 1;
                    }
                    if ys < y {
                        map.set((ys, x), 'O');
                        map.set((y, x), '.');
                    }
                }
                '#' => ys = y + 1,
                _ => {}
            }
        }
    }
}

fn score(map: &Map2D) -> u64 {
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.at((y, x)) == 'O' {
                sum += (map.height - y) as u64;
            }
        }
    }
    sum
}

fn cycle(map: &mut Map2D) -> Map2D {
    stack_north(map);
    let mut map = map.rotate_clockwise();
    stack_north(&mut map);
    let mut map = map.rotate_clockwise();
    stack_north(&mut map);
    let mut map = map.rotate_clockwise();
    stack_north(&mut map);
    map.rotate_clockwise()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut map = Map2D::from_file(&args.input)?;
    let mut map2 = map.clone();

    stack_north(&mut map2);
    println!("Score: {}", score(&map2));

    let mut cycles = 0u64;
    for _i in 0..1000 {
        cycles += 1;
        map = cycle(&mut map);
    }
    println!("cycles {}", cycles);
    let after1000 = map.clone();
    loop {
        cycles += 1;
        map = cycle(&mut map);
        if map.equal(&after1000) {
            break;
        }
    }
    let rep = cycles - 1000;
    let rem = (1000000000 - cycles) % rep;
    for _i in 0..rem {
        map = cycle(&mut map);
    }

    // map.print();
    println!("Score {}", score(&map));

    Ok(())
}
