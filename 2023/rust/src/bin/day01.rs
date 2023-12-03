use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        panic!("USAGE {} <filename>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file contents");
    let contents: Vec<&str> = contents.lines().collect();
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &[&str]) {
    let mut total: usize = 0;
    for line in contents.iter() {
        if !line.is_empty() {
            let word: Vec<char> = line.chars().filter(|c| *c >= '0' && *c <= '9').collect();
            if word.is_empty() {
                panic!("no digits in line");
            }
            let twodigitnum = [word[0], *word.last().unwrap()];
            let twodigitnum: String = twodigitnum.iter().collect();
            let number: usize = twodigitnum.as_str().parse().unwrap();
            total += number;
        }
    }
    println!("part1 total: {}", total);
}

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(contents: &[&str]) {
    let mut total: usize = 0;
    let num_len: [usize; 9] = NUMBERS
        .iter()
        .map(|&s| s.len())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    let parse_next = |remain: &str| -> (Option<usize>, usize) {
        let first = remain.chars().next().unwrap();
        if first.is_ascii_digit() {
            let digit = first as u8 - b'0';
            return (Some(digit as usize), 1);
        }
        for (idx, (&digit_str, &digit_len)) in NUMBERS.iter().zip(num_len.iter()).enumerate() {
            if digit_len <= remain.len() && digit_str == &remain[0..digit_len] {
                return (Some(idx + 1), 1); // don't skip completely over full words, so "oneight" should create two digits
            }
        }
        (None, 1)
    };

    for &line in contents.iter() {
        let mut digits = Vec::<usize>::new();
        if !line.is_empty() {
            digits.clear();
            let mut idx: usize = 0;
            while idx < line.len() {
                let parse_result = parse_next(&line[idx..]);
                if let (Some(digit), strlen) = parse_result {
                    digits.push(digit);
                    idx += strlen;
                } else {
                    idx += 1
                }
            }

            if digits.is_empty() {
                panic!("no digits in line");
            }
            let mut number = *digits.last().unwrap();
            number += 10 * digits[0];
            total += number;
        }
    }
    println!("part2 total: {}", total);
}
