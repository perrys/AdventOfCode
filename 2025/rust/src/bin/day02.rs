use std::{env, fs};

struct Range {
    lo: i32,
    hi: i32,
    lo_digits: Vec<char>,
    hi_digits: Vec<char>,
}

impl Range {
    fn parse(tok: &str) -> Self {
        let mut toks = tok.split('-');
        let first = toks.next().expect("first element missing");
        let second = toks.next().expect("second element missing");
        Self {
            lo: first.parse().expect("non integer"),
            hi: second.parse().expect("non integer"),
            lo_digits: first.chars().collect(),
            hi_digits: second.chars().collect(),
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file");
    let ranges: Vec<Range> = contents.split(',').map(Range::parse).collect();
    let p1_score: i32 = 0;
    let p2_score: i32 = 0;
    for range in ranges {
        assert!(range.lo < range.hi);
        let common_prefix: Vec<i32> = Vec::new();
        if range.lo_digits.len() == range.hi_digits.len() {
            range.lo_digits.iter().zip(range.hi_digits.iter()).map( |item| if item[0] == item[1] 
            
    }
    println!("part1: {}", p1_score);
    println!("part2: {}", p2_score);
}
