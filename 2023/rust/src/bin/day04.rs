//!
//! Advent of code challenge 2023 day 4.
//!
//! See <https://adventofcode.com/2023/day/4>
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

fn split_line(line: &str) -> [&str; 2] {
    let prefix = line.find(": ").expect("Couldn't find first colon");
    line[1 + prefix..]
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>()
        .try_into()
        .expect("didn't split into 2 parts")
}

fn parse_line(line: &str) -> usize {
    let [winning_nums, my_nums] = split_line(line);
    let mut winning_nums = winning_nums
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("unparsable number in {winning_nums}"))
        })
        .collect::<Vec<u8>>();
    winning_nums.sort();
    let mut total = 0;
    my_nums
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .for_each(|nstr| {
            let n = nstr
                .parse()
                .unwrap_or_else(|_| panic!("unable to parse number in {my_nums}"));
            if winning_nums.binary_search(&n).is_ok() {
                total += 1;
            };
        });
    total
}

fn part1(content: &str) -> usize {
    content
        .lines()
        .filter(|&s| !s.trim().is_empty())
        .map(parse_line)
        .map(|n| {
            if n > 0 {
                2_usize.pow((n - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}
fn part2(content: &str) -> usize {
    let card_scores = content
        .lines()
        .filter(|&s| !s.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();
    let mut num_cards = vec![1; card_scores.len()];
    let trunc_scores = &card_scores[0..(card_scores.len() - 1)];
    for (idx, score) in trunc_scores.iter().enumerate() {
        let this_card_count = num_cards[idx];
        let next_cards = &mut num_cards[idx + 1..(idx + 1 + *score)];
        for num in next_cards.iter_mut() {
            *num += this_card_count;
        }
    }
    num_cards.iter().sum()
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_valid_line_WHEN_splitting_THEN_parts_returned() {
        let line = "c1: 1234 | 4567 ";
        let parts = split_line(line);
        assert_eq!(parts[0], "1234");
        assert_eq!(parts[1], "4567");
    }

    #[test]
    fn GIVEN_valid_lines_WHEN_run_part1_parse_THEN_expected_total_returned() {
        let dotest = |line, expected| {
            assert_eq!(expected, parse_line(line));
        };
        dotest("c1: 12 34 | 45 67 ", 0);
        dotest("c1: 12 34 | 12 34 ", 2);
        dotest("c1: 3 2 4 |  2 4 5 67 ", 2);
        dotest("c1: 1 2 3 4 | 4 3 2 1 ", 4);
    }

    static EXAMPLE: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_expected_total_returned() {
        assert_eq!(13, part1(EXAMPLE));
    }
    #[test]
    fn GIVEN_aoc_example_input_WHEN_part2_run_THEN_expected_total_returned() {
        assert_eq!(30, part2(EXAMPLE));
    }
}
