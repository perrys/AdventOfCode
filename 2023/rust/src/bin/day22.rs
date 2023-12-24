//!
//! Advent of code challenge 2023 day 22.
//!
//! See <https://adventofcode.com/2023/day/22>
//!
use std::{collections::VecDeque, fs, ops::RangeInclusive};

use num::Integer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    0
}
fn part2(contents: &str) -> usize {
    0
}

struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: usize,
}

impl Brick {
    fn new(line: &str) -> Self {
        let mut parts = line.split('~');
        let first = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let second = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let parser = |part: &str| -> [usize; 3] {
            part.split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|_| panic!("parse error for {line}"))
                .try_into()
                .unwrap_or_else(|_| panic!("unexpected number of coords for {line}"))
        };
        let first = parser(first);
        let second = parser(second);
    }
}
