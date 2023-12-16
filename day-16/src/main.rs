use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

pub struct Map2D {
    content: Vec<char>,
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
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
                width = line.len() as i32;
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

    fn at(&self, pos: &(i32, i32)) -> char {
        let ofs = pos.0 * self.width + pos.1;
        self.content[ofs as usize]
    }

    fn set(&mut self, pos: &(i32, i32), c: char) {
        let ofs = pos.0 * self.width + pos.1;
        self.content[ofs as usize] = c;
    }

    fn pos_ok(&self, pos: &(i32, i32)) -> bool {
        (pos.0 >= 0) && (pos.1 >= 0) && (pos.0 < self.height) && (pos.1 < self.width)
    }

    fn filter_pos(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        if self.pos_ok(&pos) {
            Some(pos)
        } else {
            None
        }
    }

    fn rotate_anticlockwise(&self) -> Map2D {
        let mut content = Vec::<char>::with_capacity(self.content.len());
        let width = self.height;
        let height = self.width;

        for y in 0..height {
            for x in 0..width {
                // 12      246
                // 34  =>  135
                // 56
                let from_pos = ((height - 1 - y) + x * width) as usize;
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
                let from_pos = ((width - 1 - x) * height + y) as usize;
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
            let line = self.content[(y * self.width) as usize..((y + 1) * self.width) as usize]
                .iter()
                .collect::<String>();
            println!("{}", line);
        });
    }
}

type Visited = HashMap<(i32, i32), HashSet<(i8, i8)>>;

fn next(pos: &(i32, i32), dir: &(i8, i8)) -> (i32, i32) {
    (pos.0 + dir.0 as i32, pos.1 + dir.1 as i32)
}

/*
     /   (0,1) -> (-1,0) -> (0,1)
         (1,0) -> (0,-1) -> (1,0)

     \   (0,1) -> (1,0) -> (0,1)
         (0,-1)
*/

fn follow(map: &Map2D, trace: &mut Visited, pos: (i32, i32), dir: (i8, i8)) {
    let mut to_visit = Vec::<((i32, i32), (i8, i8))>::new();
    to_visit.push((pos, dir));

    while let Some((pos, dir)) = to_visit.pop() {
        //println!("inspecting {:?}/{:?}", pos, dir);
        let atpos = trace.entry(pos).or_default();
        if !atpos.insert(dir) {
            continue;
        }

        to_visit.extend(
            match map.at(&pos) {
                '/' => vec![(-dir.1, -dir.0)].into_iter(),
                '\\' => vec![(dir.1, dir.0)].into_iter(),
                '-' => match dir.0 {
                    0 => vec![dir].into_iter(),
                    _ => vec![(0, -1), (0, 1)].into_iter(),
                },
                '|' => match dir.1 {
                    0 => vec![dir].into_iter(),
                    _ => vec![(-1, 0), (1, 0)].into_iter(),
                },
                _ => vec![dir].into_iter(),
            }
            .filter_map(|dir| {
                let npos = next(&pos, &dir);
                map.filter_pos(npos).map(|valid_pos| (valid_pos, dir))
            })
            .collect::<Vec<((i32, i32), (i8, i8))>>(),
        );
    }
}

fn try_enter(map: &Map2D, pos: (i32, i32), dir: (i8, i8)) -> usize {
    let mut trace = Visited::new();
    follow(map, &mut trace, pos, dir);
    trace.len()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let map = Map2D::from_file(&args.input)?;

    // Part 1
    let mut trace = Visited::new();
    follow(&map, &mut trace, (0, 0), (0, 1));
    println!("part 1 {}", trace.len());

    let max_vert = (0..map.height)
        .flat_map(|y| {
            [
                try_enter(&map, (y, 0), (0, 1)),
                try_enter(&map, (y, map.width - 1), (0, -1)),
            ]
            .into_iter()
        })
        .max()
        .unwrap();

    let max_horiz = (0..map.width)
        .flat_map(|x| {
            [
                try_enter(&map, (0, x), (1, 0)),
                try_enter(&map, (map.height - 1, x), (-1, 0)),
            ]
            .into_iter()
        })
        .max()
        .unwrap();

    println!("Part 2 : {}", max_horiz.max(max_vert));

    Ok(())
}
