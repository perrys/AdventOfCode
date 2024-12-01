//!
//! Advent of code challenge 2024.
//! Day 1: Historian Hysteria
//!
//! See <https://adventofcode.com/2024/day/1>

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE {} <filename>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file contents");

    let lineprocessor = |line: &str| -> (i32, i32) {
        let v = line
            .split(' ')
            .filter(|&tok| !tok.trim().is_empty())
            .map(|token| token.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| panic!("couldn't parse integers in {line}"));
        if v.len() != 2 {
            panic!("ERROR: invalid input at  line: {}", line);
        }
        (v[0], v[1])
    };

    let (mut first, mut second): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .map(lineprocessor)
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    first.sort();
    second.sort();

    println!(
        "part1 answer: {}",
        first
            .iter()
            .zip(second.iter())
            .map(|(lhs, rhs)| (lhs - rhs).abs())
            .sum::<i32>()
    );

    let mut occurences = HashMap::new();
    second.into_iter().for_each(|n| {
        let count = occurences.get(&n).unwrap_or(&0);
        occurences.insert(n, count + 1);
    });

    println!(
        "part2 answer: {}",
        first
            .iter()
            .map(|n| n * occurences.get(n).unwrap_or(&0))
            .sum::<i32>()
    );
}
