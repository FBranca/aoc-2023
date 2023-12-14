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

type Pos = (i32, i32);

fn find_start(map: &[String]) -> (i32, i32) {
    map.iter()
        .enumerate()
        .map(|(y, line)| line.find('S').map(|x| (y as i32, x as i32)))
        .find_map(|x| x)
        .unwrap()
}

fn is_valid(map: &Vec<String>, pos: &Pos) -> bool {
    let (y, x) = pos;
    if *y >= 0 && *y < map.len() as i32 {
        *x >= 0 && *x < map[0].len() as i32
    } else {
        false
    }
}

fn at(map: &[String], pos: &Pos) -> char {
    map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap()
}

fn add_guess(v: &mut Vec<Pos>, map: &Vec<String>, pos: &Pos, dir: &Pos, pipes: &[char]) -> bool {
    let p = (pos.0 + dir.0, pos.1 + dir.1);
    if is_valid(map, &p) {
        let c = at(map, &p);
        if pipes.contains(&c) {
            v.push(p);
            return true;
        }
    }
    false
}

fn guess_missing_char(left: &bool, right: &bool, up: &bool, down: &bool) -> char {
    match (left, right, up, down) {
        (true, true, false, false) => '-',
        (true, false, true, false) => 'J',
        (true, false, false, true) => '7',
        (false, true, true, false) => 'L',
        (false, true, false, true) => 'F',
        (false, false, true, true) => '|',
        _ => panic!("impossible start configuration"),
    }
}

fn next_dir(from: &Pos, p1: Pos, p2: Pos) -> Pos {
    if *from == p1 {
        p2
    } else {
        p1
    }
}

fn next(map: &[String], from: &Pos, pos: &Pos) -> Pos {
    let c = map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap();
    match c {
        'L' => next_dir(from, (pos.0 - 1, pos.1), (pos.0, pos.1 + 1)),
        'J' => next_dir(from, (pos.0 - 1, pos.1), (pos.0, pos.1 - 1)),
        '7' => next_dir(from, (pos.0 + 1, pos.1), (pos.0, pos.1 - 1)),
        'F' => next_dir(from, (pos.0 + 1, pos.1), (pos.0, pos.1 + 1)),
        '-' => next_dir(from, (pos.0, pos.1 - 1), (pos.0, pos.1 + 1)),
        '|' => next_dir(from, (pos.0 - 1, pos.1), (pos.0 + 1, pos.1)),
        _ => panic!("Invalid char {} at {:?}", c, pos),
    }
}

fn is_in_loop(map: &[String], pos: &Pos) -> bool {
    let c = at(map, pos);
    if c != ' ' {
        false
    } else {
        let (_, cnt) = map[pos.0 as usize].chars().take(pos.1 as usize).fold(
            (Option::<char>::None, 0),
            |(o, cnt), c| match c {
                '|' => (None, cnt + 1),
                'L' => (Some('7'), cnt),
                'F' => (Some('J'), cnt),
                '7' => (None, cnt + o.map_or(0, |v| (v == c) as i32)),
                'J' => (None, cnt + o.map_or(0, |v| (v == c) as i32)),
                _ => (o, cnt),
            },
        );
        // println!("{} {} {}", map[pos.0 as usize], pos.1, cnt);
        (cnt % 2) == 1
    }
}

fn to_ascii(map: &[String]) -> Vec<String> {
    map.iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => '└',
                    'F' => '┌',
                    'J' => '┘',
                    '7' => '┐',
                    '|' => '│',
                    '-' => '─',
                    '.' => ' ',
                    c => c,
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let map = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let start = find_start(&map);

    // guess 1st step
    let mut firsts = Vec::<Pos>::new();
    let on_left = add_guess(&mut firsts, &map, &start, &(-1, 0), &['F', '7', '|']);
    let on_right = add_guess(&mut firsts, &map, &start, &(1, 0), &['L', 'J', '|']);
    let on_up = add_guess(&mut firsts, &map, &start, &(0, -1), &['F', 'L', '-']);
    let on_down = add_guess(&mut firsts, &map, &start, &(0, 1), &['7', 'J', '-']);
    let start_char = guess_missing_char(&on_left, &on_right, &on_up, &on_down);

    let mut only_loop = (0..map.len())
        .map(|_x| {
            " ".repeat(map[0].len())
        })
        .collect::<Vec<String>>();

    let height = map.len();
    let width = map[0].len();

    let mut count = 1;
    let mut prec = start;
    let mut pos = firsts[0];
    while at(&map, &pos) != 'S' {
        only_loop[pos.0 as usize].replace_range(
            pos.1 as usize..pos.1 as usize + 1,
            at(&map, &pos).to_string().as_str(),
        );
        count += 1;
        let n = next(&map, &prec, &pos);
        prec = pos;
        pos = n;
    }
    only_loop[start.0 as usize].replace_range(
        start.1 as usize..start.1 as usize + 1,
        start_char.to_string().as_str(),
    );

    println!("{}", count / 2);
    for l in &only_loop {
        println!("{}!", l);
    }

    for l in to_ascii(&only_loop) {
        println!("{}", l);
    }

    let in_loop = (0..height).fold(0, |acc, y| {
        (0..width).fold(acc, |acc, x| {
            acc + is_in_loop(&only_loop, &(y as i32, x as i32)) as i32
        })
    });

    println!("{}", in_loop);
    // 535 too high
    Ok(())
}
