use std::{env, fs};

fn to_int(chars: &[char]) -> u64 {
    let mut result = 0;
    let mut mult = 1;
    let zero = '0' as u64;
    for c in chars.iter().rev() {
        result += (*c as u64 - zero) * mult;
        mult *= 10;
    }
    result
}

fn largest_pair(line_str: &str) -> u64 {
    let line: Vec<_> = line_str.chars().collect();
    let mut first = None;
    let mut second = None;
    for i in 0..line.len() {
        let next = line[i];
        if let Some(cur) = first {
            if next > cur && i != line.len() - 1 {
                first = Some(next);
                second = None;
                continue;
            }
        } else {
            first = Some(next);
            continue;
        }
        if let Some(cur) = second
            && next < cur
        {
            continue;
        }
        second = Some(next);
    }
    to_int(&[first.unwrap(), second.unwrap()])
}

fn largest(line: &[char], ndigits: usize, result: &mut Vec<char>) {
    if 0 == ndigits {
        return;
    }
    assert!(line.len() >= ndigits);
    let usable_len = line.len() - ndigits + 1;
    let mut idx = None;
    for i in 0..usable_len {
        if let Some(max_idx) = idx
            && line[i] <= line[max_idx]
        {
            continue;
        }
        idx = Some(i);
    }
    let pos = idx.expect("empty line");
    result.push(line[pos]);
    let next_line = &line[pos + 1..];
    largest(next_line, ndigits - 1, result);
}

fn largest_12(line_str: &str) -> u64 {
    let line: Vec<_> = line_str.chars().collect();
    let mut result = Vec::<char>::with_capacity(12);
    largest(&line, 12, &mut result);
    to_int(&result)
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file");
    let lines: Vec<_> = contents.split("\n").filter(|l| !l.is_empty()).collect();
    let p1: u64 = lines.clone().into_iter().map(largest_pair).sum();
    println!("part1: {}", p1);
    let p2: u64 = lines.into_iter().map(largest_12).sum();
    println!("part2: {}", p2);
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn largest_pair_test() {
        assert_eq!(largest_pair("987654321111111"), 98);
        assert_eq!(largest_pair("811111111111119"), 89);
        assert_eq!(largest_pair("234234234234278"), 78);
        assert_eq!(largest_pair("818181911112111"), 92);
    }

    #[test]
    fn largest_12_test() {
        assert_eq!(largest_12("987654321111111"), 987654321111);
        assert_eq!(largest_12("811111111111119"), 811111111119);
        assert_eq!(largest_12("234234234234278"), 434234234278);
        assert_eq!(largest_12("818181911112111"), 888911112111);
    }
}
