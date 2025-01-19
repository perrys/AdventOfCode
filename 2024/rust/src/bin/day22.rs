//!
//! Advent of code challenge 2024.
//! Day 1: Historian Hysteria
//!
//! See <https://adventofcode.com/2024/day/1>

use std::env;
use std::fs;

const MODULO: i64 = 16777216;

fn p1_transform(i: i32) -> i32 {
    let mut val: i64 = i as i64;
    let mut tmp = val * 64;
    println!("tmp: {tmp: >64b}");
    val ^= tmp;
    println!("val: {val: >64b}");
    val %= MODULO;
    println!("val: {val: >64b}");

    tmp = val / 32;
    println!("tmp: {tmp: >64b}");
    val ^= tmp;
    println!("val: {val: >64b}");
    val %= MODULO;
    println!("val: {val: >64b}");

    tmp = val * 2048;
    println!("tmp: {tmp: >64b}");
    val ^= tmp;
    println!("val: {val: >64b}");
    val %= MODULO;
    println!("val: {val: >64b}");

    val as i32
}

fn p1_repeat(i: &i32) -> i32 {
    let mut val = *i;
    for _n in 0..4 {
        println!("--\n");
        val = p1_transform(val);
    }
    val
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE {} <filename>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file contents");

    let numbers: Vec<i32> = contents
        .lines()
        .filter(|&tok| !tok.trim().is_empty())
        .map(|token| token.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("couldn't parse integer"));
    let p1result: i64 = numbers.iter().map(p1_repeat).map(|i| i as i64).sum();
    println!("part1 result is {p1result}");
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn can_reproduce_example_results() {
        println!("mod: {MODULO:b}");
        // assert_eq!(p1_repeat(&1), 8685429);
        assert_eq!(p1_repeat(&10), 4700978);
        // assert_eq!(p1_repeat(&100), 15273692);
        // assert_eq!(p1_repeat(&2024), 8667524);
    }
}
