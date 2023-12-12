//!
//! Advent of code challenge 2023 day 12.
//!
//! See <https://adventofcode.com/2023/day/12>
//!
use std::fs;

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

fn part1(_contents: &str) -> usize {
    0
}

fn part2(_contents: &str) -> usize {
    0
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum SpringCondition {
    Broken,
    Working,
    Unknown,
}

struct Record {
    springs: Vec<SpringCondition>,
    broken_spans: Vec<usize>,
}

fn is_consistent(spring: &[SpringCondition], broken_spans: &[usize]) -> bool {
    let slots = spring
        .split(|s| *s == SpringCondition::Working)
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect::<Vec<_>>();
    if slots.len() < broken_spans.len() {
        return true;
    }
    if slots.len() < broken_spans.len() {
        return true;
    } else if slots.len() > broken_spans.len() {
        return false;
    }
    slots.iter().zip(broken_spans.iter()).all(|(s, b)| s >= b)
}

fn possibilities(spring: &[SpringCondition], broken_spans: &[usize]) -> Option<usize> {
    // walk along record
    // ignore first working
    // if first is broken - fill remainder, then recurse
    // if first is unknown - fill remainder, end test
    //  ok - repeat until full, then recurse
    //  not ok - mark previous attempts working and continue
    None
}
