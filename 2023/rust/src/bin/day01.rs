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

fn part1_parse(line: &str) -> usize {
    let word: Vec<char> = line.chars().filter(|c| *c >= '0' && *c <= '9').collect();
    if word.is_empty() {
        panic!("no digits in line");
    }
    let twodigitnum = [word[0], *word.last().unwrap()];
    let twodigitnum: String = twodigitnum.iter().collect();
    twodigitnum.as_str().parse().unwrap()
}

fn part1(contents: &[&str]) {
    let mut total: usize = 0;
    for line in contents.iter() {
        if !line.is_empty() {
            total += part1_parse(line);
        }
    }
    println!("part1 total: {}", total);
}

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2_parse_next(remain: &str) -> Option<usize> {
    let first = remain.chars().next().unwrap();
    if first.is_ascii_digit() {
        let digit = first as u8 - b'0';
        return Some(digit as usize);
    }
    for (idx, &digit_str) in NUMBERS.iter().enumerate() {
        if remain.starts_with(digit_str) {
            return Some(idx + 1); // don't skip completely over full words, so "oneight" should create two digits
        }
    }
    None
}

fn part2_parse_line(line: &str, digits: &mut Vec<usize>) -> usize {
    let mut idx: usize = 0;
    while idx < line.len() {
        let parse_result = part2_parse_next(&line[idx..]);
        if let Some(digit) = parse_result {
            digits.push(digit);
        }
        idx += 1
    }
    if digits.is_empty() {
        panic!("no digits in line");
    }
    let mut number = *digits.last().unwrap();
    number += 10 * digits[0];
    number
}

fn part2(contents: &[&str]) {
    let mut total: usize = 0;
    let mut digits = Vec::<usize>::new();
    for &line in contents.iter() {
        if !line.is_empty() {
            total += part2_parse_line(line, &mut digits);
        }
        digits.clear();
    }
    println!("part2 total: {}", total);
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tester {
    use super::*;

    fn dotest(line: &str, expected: usize, mut parser: impl FnMut(&str) -> usize) {
        assert_eq!(expected, parser(line));
    }

    #[test]
    fn GIVEN_valid_lines_WHEN_part1_parser_run_THEN_expected_totals_produced() {
        let test = |line, expected| dotest(line, expected, part1_parse);

        test("12", 12); // only number
        test("1.2", 12); // start and end
        test("$%l12!$", 12); // middle
        test("$%l1..2!$", 12); // middle spaced
        test("..03..", 3); // leading zero
        test("..3..", 33); // single digit
    }

    #[should_panic]
    #[test]
    fn GIVEN_invalid_lines_WHEN_part1_parser_run_THEN_panics() {
        // only number
        let line = "%$^";
        part1_parse(line);
    }

    #[test]
    fn GIVEN_valid_lines_WHEN_part2_parser_run_THEN_expected_totals_produced() {
        let mut workspace = Vec::<usize>::new();
        let mut test = |line, expected| {
            dotest(line, expected, |line| {
                part2_parse_line(line, &mut workspace)
            });
            workspace.clear();
        };

        test("12", 12); // only number
        test("1.2", 12); // start and end
        test("$%l12!$", 12); // middle
        test("$%l1..2!$", 12); // middle spaced
        test("..03..", 3); // leading zero
        test("..3..", 33); // single digit

        test("..one..", 11); // single word
        test("..two..", 22); // single word
        test("..three..", 33); // single word
        test("..four..", 44); // single word
        test("..five..", 55); // single word
        test("..six..", 66); // single word
        test("..seven..", 77); // single word
        test("..eight..", 88); // single word
        test("..nine..", 99); // single word
        test("..three.eight..", 38); // double word
        test("..3.eight..", 38); // digit + word
        test("..threeeight..", 38); // non-spaced
        test("..threeight..", 38); // overlapping
    }

    #[should_panic]
    #[test]
    fn GIVEN_invalid_lines_WHEN_part2_parser_run_THEN_panics() {
        // only number
        let line = "%$^";
        let mut workspace = Vec::<usize>::new();
        part2_parse_line(line, &mut workspace);
    }
}
