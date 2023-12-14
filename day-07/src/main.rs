use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

// Convert Card to it hexadecimal value
fn sortable_card(c: &char, joker: Option<char>) -> char {
    match joker {
        Some(j) if j == *c => '1',
        _ => match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            _ => *c,
        },
    }
}

// Wrong way, too much poker :)
// This version sort the cards instead of preserving the initial order
// (I didn't read correctly the description .. oups)
//
// Build a sortable representation :
// - A, K, Q, J, T are replaced by their hex value (A=T, B=J, C=Q, D=K, E=A)
// - Begin with the hand type (number of occurences of cards)
// - Followed by a sorted list of cards
// Examples:
//  23456 (23456 -> 65432) => 1111165432
//  T4A34 (A4E34 -> 44EA3) => 21114EA3
//  47747 (47747 -> 77744) => 3274
#[allow(dead_code)]
fn build_sortable_repr(hand: &str, joker: Option<char>) -> (String, String) {
    let freq = hand
        .chars()
        .fold(
            // Build a frequency table
            HashMap::new(),
            |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            },
        )
        .iter()
        .map(|(char, count)| (sortable_card(char, joker), count))
        .sorted_by(|(char1, count1), (char2, count2)| {
            if count1 == count2 {
                char2.cmp(char1)
            } else {
                count2.cmp(count1)
            }
        })
        .fold(
            (String::with_capacity(5), String::with_capacity(5)),
            |(mut str_cnt, mut str_cards), (char, count)| {
                str_cnt.push_str(count.to_string().as_str());
                str_cards.push(char);
                (str_cnt, str_cards)
            },
        );

    (freq.0, freq.1)
}

fn build_type_repr(hand: &str, joker: Option<char>) -> String {
    let (freq_map, jokers) = hand.chars().fold(
        // Build a frequency table
        (HashMap::<char, u32>::new(), 0),
        |(mut map, jokers), c| match joker.cmp(&Some(c)).is_eq() {
            true => (map, jokers + 1),
            false => {
                *map.entry(c).or_insert(0) += 1;
                (map, jokers)
            }
        },
    );
    if jokers == 5 {
        return "5".to_owned();
    }

    freq_map
        .iter()
        .sorted_by(|(_, count1), (_, count2)| count2.cmp(count1))
        .fold(
            (jokers, String::with_capacity(5)),
            |(jokers, mut str_cnt), (_char, count)| {
                str_cnt.push_str((count + jokers).to_string().as_str());
                (0, str_cnt)
            },
        )
        .1
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;

    let mut games = Vec::<(String, String, u64)>::new();
    let mut games_joker = Vec::<(String, String, u64)>::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_at(5);
        let type_repr = build_type_repr(hand, None);
        let type_repr_joker = build_type_repr(hand, Some('J'));
        games.push((
            hand.to_owned(),
            type_repr,
            bid.trim().parse::<u64>().unwrap(),
        ));
        games_joker.push((
            hand.to_owned(),
            type_repr_joker,
            bid.trim().parse::<u64>().unwrap(),
        ));
    }

    games.sort_by(|(hand1, type1, _), (hand2, type2, _)| {
        if type1 != type2 {
            type1.cmp(type2)
        } else {
            hand1
                .chars()
                .map(|c| sortable_card(&c, None))
                .collect::<String>()
                .cmp(
                    &hand2
                        .chars()
                        .map(|c| sortable_card(&c, None))
                        .collect::<String>(),
                )
        }
    });

    games_joker.sort_by(|(hand1, type1, _), (hand2, type2, _)| {
        if type1 != type2 {
            type1.cmp(type2)
        } else {
            hand1
                .chars()
                .map(|c| sortable_card(&c, Some('J')))
                .collect::<String>()
                .cmp(
                    &hand2
                        .chars()
                        .map(|c| sortable_card(&c, Some('J')))
                        .collect::<String>(),
                )
        }
    });

    let result = games
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, (_hand, _shand, bid))| {
            let rank = 1 + idx as u64;
            acc + rank * bid
        });

    let result_joker =
        games_joker
            .iter()
            .enumerate()
            .fold(0u64, |acc, (idx, (hand, shand, bid))| {
                let rank = 1 + idx as u64;
                println!(
                    "{} [{:10}]   {:4} * {:3}  sum: {}",
                    hand, shand, rank, bid, acc
                );
                acc + rank * bid
            });

    println!("result {}", result);
    println!("result with joker {}", result_joker);

    Ok(())
}
