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

impl SpringCondition {
    fn new(c: char) -> Self {
        match c {
            '#' => Self::Broken,
            '?' => Self::Unknown,
            '.' => Self::Working,
            _ => panic!("usupported spring condition \"{c}\""),
        }
    }
}

struct Record {
    springs: Vec<SpringCondition>,
    broken_spans: Vec<usize>,
}

impl Record {
    fn new(line: &str) -> Self {
        let mut iter = line.split(' ');
        let springs = iter
            .next()
            .unwrap_or_else(|| panic!("unable to tokenize \"{line}\""))
            .chars()
            .map(SpringCondition::new)
            .collect::<Vec<_>>();
        let broken_spans = iter
            .next()
            .unwrap_or_else(|| panic!("single token in line \"{line}\""))
            .split(',')
            .map(|s| s.parse::<usize>().expect("unparsable integer"))
            .collect::<Vec<_>>();
        Self {
            springs,
            broken_spans,
        }
    }
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
    let mut iter = spring
        .iter()
        .take_while(|&s| *s == SpringCondition::Working);

    // walk along record
    // ignore first working
    // if first is broken - fill remainder, then recurse
    // if first is unknown - fill remainder, end test
    //  ok - repeat until full, then recurse
    //  not ok - mark previous attempts working and continue
    None
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test12 {
    use crate::*;

    #[test]
    fn GIVEN_valid_line_record_WHEN_parsing_THEN_corect_record_produced() {
        let record = Record::new("##????????#?#????.? 4,1,8,2");
        assert_eq!(19, record.springs.len());
        assert_eq!(SpringCondition::Broken, record.springs[1]);
        assert_eq!(SpringCondition::Unknown, record.springs[5]);
        assert_eq!(SpringCondition::Working, record.springs[17]);
        assert_eq!(vec![4, 1, 8, 2], record.broken_spans);
    }

    #[test]
    fn GIVEN_valid_records_WHEN_testing_consistency_THEN_true_returned() {
        let dotest = |line| {
            let record = Record::new(line);
            assert!(is_consistent(&record.springs, &record.broken_spans));
        };
        dotest("???? 1,2");
        dotest("?.?? 1,2");

        for line in EXAMPLE_INPUT.lines().filter(|l| !l.trim().is_empty()) {
            dotest(line);
        }
    }

    #[test]
    fn GIVEN_invalid_records_WHEN_testing_consistency_THEN_false_returned() {
        let dotest = |line| {
            let record = Record::new(line);
            assert!(!is_consistent(&record.springs, &record.broken_spans));
        };
        dotest("?.? 1,2");
    }

    static EXAMPLE_INPUT: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;
}
