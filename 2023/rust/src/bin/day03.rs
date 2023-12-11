//!
//! Advent of code challenge 2023 day 3.
//!
//! See <https://adventofcode.com/2023/day/3>
//!
use std::{fs, ops::Range};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    let contents: Vec<_> = contents.lines().collect();

    println!("part1 total is {}", part1(&contents));
    println!("part2 total is {}", part2(&contents));
}

#[derive(PartialEq, Debug)]
struct PartNumber {
    value: usize,
    range: Range<usize>,
}

impl PartNumber {
    fn new(value: usize, range: Range<usize>) -> Self {
        Self { value, range }
    }

    fn is_adjacent_to(&self, col_idx: usize) -> bool {
        let diag_range =
            (self.range.start - std::cmp::min(self.range.start, 1))..self.range.end + 1;
        diag_range.contains(&col_idx)
    }
}

#[derive(Debug)]
struct ParsedLine {
    pub numbers: Vec<PartNumber>,
    pub sym_posns: Vec<usize>,
}

impl ParsedLine {
    fn new(line: &str, symbol_predicate: impl Fn(char) -> bool) -> Self {
        let mut result = Self {
            numbers: Vec::new(),
            sym_posns: Vec::new(),
        };
        let mut accum: usize = 0;
        let mut len: usize = 0;
        for (col_idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                accum *= 10;
                accum += (c as u8 - b'0') as usize;
                len += 1;
            } else {
                if accum > 0 {
                    result
                        .numbers
                        .push(PartNumber::new(accum, col_idx - len..col_idx));
                    accum = 0;
                    len = 0;
                }
                if symbol_predicate(c) {
                    result.sym_posns.push(col_idx);
                }
            }
        }
        if accum > 0 {
            result
                .numbers
                .push(PartNumber::new(accum, line.len() - len..line.len()));
        }
        result
    }
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn part1(lines: &[&str]) -> usize {
    let mut total: usize = 0;
    let parsedlines = lines
        .iter()
        .map(|line| ParsedLine::new(line, is_symbol))
        .collect::<Vec<_>>();
    for (line_idx, pline) in parsedlines.iter().enumerate() {
        for num in pline.numbers.iter() {
            if line_idx > 0
                && parsedlines[line_idx - 1]
                    .sym_posns
                    .iter()
                    .any(|&col_idx| num.is_adjacent_to(col_idx))
            {
                total += num.value;
                continue;
            }
            if pline
                .sym_posns
                .iter()
                .any(|&col_idx| num.is_adjacent_to(col_idx))
            {
                total += num.value;
                continue;
            }
            if line_idx < parsedlines.len() - 1
                && parsedlines[line_idx + 1]
                    .sym_posns
                    .iter()
                    .any(|&col_idx| num.is_adjacent_to(col_idx))
            {
                total += num.value;
                continue;
            }
        }
    }
    total
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn part2(lines: &[&str]) -> usize {
    let mut total: usize = 0;
    let parsedlines = lines
        .iter()
        .map(|line| ParsedLine::new(line, is_gear))
        .collect::<Vec<_>>();
    let mut candidates = Vec::<&PartNumber>::new();
    for (line_idx, pline) in parsedlines.iter().enumerate() {
        for &gear_posn in pline.sym_posns.iter() {
            if line_idx > 0 {
                candidates.extend(
                    parsedlines[line_idx - 1]
                        .numbers
                        .iter()
                        .filter(|&num| num.is_adjacent_to(gear_posn)),
                );
            }
            if line_idx < lines.len() - 1 {
                candidates.extend(
                    parsedlines[line_idx + 1]
                        .numbers
                        .iter()
                        .filter(|&num| num.is_adjacent_to(gear_posn)),
                );
            }
            candidates.extend(
                pline
                    .numbers
                    .iter()
                    .filter(|&num| num.is_adjacent_to(gear_posn)),
            );
            assert!(candidates.len() <= 2);
            if candidates.len() == 2 {
                total += candidates[0].value * candidates[1].value;
            }
            candidates.clear();
        }
    }
    total
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_various_lines_WHEN_parsing_THEN_expected_numbers_and_symbols() {
        // number in middle
        let line = "*(123%.";
        let pline = ParsedLine::new(line, |_| false);
        assert_eq!(pline.numbers, vec![PartNumber::new(123, 2..5)]);
        assert!(pline.sym_posns.is_empty());

        // at start
        let line = "123%.";
        let pline = ParsedLine::new(line, |_| false);
        assert_eq!(pline.numbers, vec![PartNumber::new(123, 0..3)]);
        assert!(pline.sym_posns.is_empty());

        // just symbol
        let line = ".^.";
        let pline = ParsedLine::new(line, |c| c == '^');
        assert!(pline.numbers.is_empty());
        assert_eq!(pline.sym_posns, vec![1]);

        // at end with symbol
        let line = ".^.2345";
        let pline = ParsedLine::new(line, |c| c == '^');
        assert_eq!(pline.numbers, vec![PartNumber::new(2345, 3..7)]);
        assert_eq!(pline.sym_posns, vec![1]);

        // multiple, no spaces
        let line = "123^234";
        let pline = ParsedLine::new(line, |c| c == '^');
        assert_eq!(
            pline.numbers,
            vec![PartNumber::new(123, 0..3), PartNumber::new(234, 4..7)]
        );
        assert_eq!(pline.sym_posns, vec![3]);
    }

    #[test]
    fn GIVEN_parsed_number_WHEN_testing_adjacent_THEN_allows_diagonals() {
        let num = PartNumber::new(7, 0..1);
        assert!(num.is_adjacent_to(0));
        assert!(num.is_adjacent_to(1));
        assert!(!num.is_adjacent_to(2));

        let num = PartNumber::new(3, 2..5);
        assert!(!num.is_adjacent_to(0));
        assert!(num.is_adjacent_to(1));
        assert!(num.is_adjacent_to(2));
        assert!(num.is_adjacent_to(4));
        assert!(num.is_adjacent_to(5));
        assert!(!num.is_adjacent_to(6));
    }

    static TEST_INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_matches_expected_total() {
        let contents: Vec<_> = TEST_INPUT.lines().collect();
        assert_eq!(part1(&contents), 4361);
    }

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part2_run_THEN_matches_expected_total() {
        let contents: Vec<_> = TEST_INPUT.lines().collect();
        assert_eq!(part2(&contents), 467835);
    }
}
