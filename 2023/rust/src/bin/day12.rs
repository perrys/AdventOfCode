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

fn part1(contents: &str) -> usize {
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let record = Record::new(line);
            possibilities(&record.springs, &record.broken_spans)
                .unwrap_or_else(|| panic!("couldn't calculate possibliites for line \"{line}\""))
        })
        .sum()
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

#[allow(dead_code)] // for testing
fn is_consistent(spring: &[SpringCondition], broken_spans: &[usize]) -> bool {
    let slots = spring
        .split(|s| *s == SpringCondition::Working)
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect::<Vec<_>>();
    match slots.len().cmp(&broken_spans.len()) {
        std::cmp::Ordering::Less => {
            // I think we could be stricter here - e.g. by testing all possible
            // sub-divisions recursively. This might speed up the algo, but the
            // following may be good enough to get it working..
            true
        }
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Equal => slots
            .iter()
            .zip(broken_spans.iter())
            .all(|(slot_size, broken_span)| slot_size >= broken_span),
    }
}

fn possibilities(slice: &[SpringCondition], broken_spans: &[usize]) -> Option<usize> {
    // eat  whitespace:
    let mut slice = slice;
    while !slice.is_empty() && slice[0] == SpringCondition::Working {
        slice = &slice[1..]
    }

    let start_here = |mut sub_slice: &[SpringCondition]| {
        if sub_slice.len() < broken_spans[0] {
            return None;
        }
        for _ in 0..broken_spans[0] {
            match sub_slice[0] {
                SpringCondition::Unknown | SpringCondition::Broken => sub_slice = &sub_slice[1..],
                _ => return None,
            }
        }
        if !sub_slice.is_empty() {
            // the item following a broken span must be working spring:
            match sub_slice[0] {
                SpringCondition::Unknown | SpringCondition::Working => sub_slice = &sub_slice[1..],
                _ => return None,
            }
        }
        possibilities(sub_slice, &broken_spans[1..])
    };

    if broken_spans.is_empty() {
        match slice.iter().all(|s| *s != SpringCondition::Broken) {
            true => Some(1),
            false => None,
        }
    } else if slice.is_empty() {
        match broken_spans.len() {
            0 => Some(1),
            _ => None,
        }
    } else {
        match slice[0] {
            SpringCondition::Broken => {
                // if the first in this group is a broken spring, the remainder must match exactly
                start_here(slice)
            }
            SpringCondition::Unknown => {
                let mut sum_possibilities = 0;
                // 1. assume broken:
                if let Some(result) = start_here(slice) {
                    sum_possibilities += result;
                }
                // 2. assume working:
                if let Some(result) = possibilities(&slice[1..], broken_spans) {
                    sum_possibilities += result;
                }
                match sum_possibilities {
                    0 => None,
                    _ => Some(sum_possibilities),
                }
            }
            SpringCondition::Working => unreachable!("already walked past working slots"),
        }
    }
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

    #[test]
    fn GIVEN_valid_records_WHEN_counting_possibilities_THEN_correct_answers_returned() {
        let dotest = |line, expected| {
            let record = Record::new(line);
            assert_eq!(
                expected,
                possibilities(&record.springs, &record.broken_spans)
            );
        };
        dotest(". 1", None);
        dotest("# 1", Some(1));
        dotest("#.# 1,1", Some(1));
        dotest("#.? 1,1", Some(1));
        dotest(".#..? 1,1", Some(1));
        dotest(".#..?. 1,1", Some(1));
        dotest(".#..??. 1,1", Some(2));
        dotest(".??..#. 1,1", Some(2));
        dotest(".??..??. 1,1", Some(4));
        dotest(".??..??...?##. 1,1,3", Some(4));
        dotest("????? 2,1", Some(3));
        dotest("?????? 2,1", Some(6));
        dotest("??????? 2,1", Some(10));
        dotest("?###???????? 3,2,1", Some(10));
    }

    static EXAMPLE_INPUT: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(21, part1(EXAMPLE_INPUT));
    }
}
