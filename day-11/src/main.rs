use std::fs::File;
use std::io::{prelude::*, BufReader};

use boolinator::Boolinator;
use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

type Pos = (usize, usize);

#[derive(Clone)]
pub struct Map2D {
    pub content: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl Map2D {
    pub fn from_file(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let mut map = Map2D {
            content: BufReader::new(file)
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<String>>(),
            width: 0,
            height: 0,
        };
        map.height = map.content.len();
        map.width = map.content[0].len();

        Ok(map)
    }

    // used when debugging
    #[allow(dead_code)]
    fn print(&self) {
        self.content.iter().for_each(|line| {
            println!("{}", line);
        });
    }
}

// Finally unused
#[allow(dead_code)]
fn dilate(sky: &Map2D) -> Map2D {
    // Find empty rows
    let empty_rows = sky
        .content
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            let galaxy_cnt = line.chars().fold(0, |acc, c| acc + (c == '#') as u32);

            (galaxy_cnt == 0).as_some(row)
        })
        .collect::<Vec<usize>>();

    // Find empty cols
    let empty_cols = (0..sky.width)
        .filter_map(|col| {
            let galaxy_cnt = sky.content.iter().fold(0, |acc, c| {
                acc + (c.chars().nth(col).unwrap() == '#') as u32
            });

            (galaxy_cnt == 0).as_some(col)
        })
        .collect::<Vec<usize>>();

    // Insert lines & columns
    let dilated_sky = sky
        .content
        .iter()
        .enumerate()
        .map(|(j, v)| {
            (
                j,
                v.chars()
                    .enumerate()
                    .flat_map(|(i, c)| {
                        let count = empty_cols.contains(&i) as usize + 1;
                        std::iter::repeat(c).take(count)
                    })
                    .collect::<String>(),
            )
        })
        .flat_map(|(j, row)| {
            let count = empty_rows.contains(&j) as usize + 1;
            std::iter::repeat(row).take(count)
        })
        .collect::<Vec<String>>();

    let width = dilated_sky[0].len();
    let height = dilated_sky.len();
    Map2D {
        content: dilated_sky,
        width,
        height,
    }
}

fn find_empty_spaces(sky: &Map2D) -> (Vec<usize>, Vec<usize>) {
    // Find empty rows
    let empty_rows = sky
        .content
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            let galaxy_cnt = line.chars().fold(0, |acc, c| acc + (c == '#') as u32);

            (galaxy_cnt == 0).as_some(row)
        })
        .collect::<Vec<usize>>();

    // Find empty cols
    let empty_cols = (0..sky.width)
        .filter_map(|col| {
            let galaxy_cnt = sky.content.iter().fold(0, |acc, c| {
                acc + (c.chars().nth(col).unwrap() == '#') as u32
            });

            (galaxy_cnt == 0).as_some(col)
        })
        .collect::<Vec<usize>>();

    (empty_rows, empty_cols)
}

fn extract_galaxies(sky: &Map2D) -> Vec<(usize, usize)> {
    sky.content
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .flat_map(move |(x, c)| std::iter::repeat((y, x)).take((c == '#') as usize))
        })
        .collect::<Vec<(usize, usize)>>()
}

fn distance(g1: &Pos, g2: &Pos, empty_spaces: &(Vec<usize>, Vec<usize>), space_weight: u64) -> u64 {
    /*  .5...........   .5
        .##.........6   .#
        ..##.........   .#
        ...##........   .#
        ....##...7...   .#   .
        8....9.......   8####9
    */
    let (empty_rows, empty_cols) = empty_spaces;

    let row_start = g1.0.min(g2.0);
    let row_end = g1.0.max(g2.0);
    let row_space = empty_rows.iter().fold(0_u64, |acc, row| {
        acc + space_weight * (row_start < *row && *row < row_end) as u64
    });
    let col_start = g1.1.min(g2.1);
    let col_end = g1.1.max(g2.1);
    let col_space = empty_cols.iter().fold(0_u64, |acc, col| {
        acc + space_weight * (col_start < *col && *col < col_end) as u64
    });
    row_space
        + (g1.0 as i64 - g2.0 as i64).unsigned_abs()
        + col_space
        + (g1.1 as i64 - g2.1 as i64).unsigned_abs()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let sky = Map2D::from_file(&args.input)?;

    // Return empty spaces list for (rows, columns)
    let empty_spaces = find_empty_spaces(&sky);

    let galaxies = extract_galaxies(&sky);
    println!("Galaxies: {:?}", galaxies);

    // Iterate all pairs
    for i in [1, 999999] {
        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|v| distance(v[0], v[1], &empty_spaces, i))
            .sum();
        println!("Sum (x{}) is {}", i, sum);
    }

    Ok(())
}
