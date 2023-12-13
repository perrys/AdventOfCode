//!
//! Advent of code challenge 2023 day 12.
//!
//! See <https://adventofcode.com/2023/day/12>
//!
use std::{collections::HashMap, fs};

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
    run_part(contents, Record::new)
}

fn part2(contents: &str) -> usize {
    run_part(contents, Record::new_from_folded)
}

fn run_part(contents: &str, factory: impl Fn(&str) -> Record) -> usize {
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let record = factory(line);
            let mut cache = PermutationsCache::new();
            possibilities(&record.springs, &record.broken_spans, &mut cache)
                .unwrap_or_else(|| panic!("couldn't calculate possibliites for line \"{line}\""))
        })
        .sum()
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Hash)]
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
    fn new_from_folded(line: &str) -> Self {
        let folded = Self::new(line);
        let mut springs = folded.springs.clone();
        let mut broken_spans = folded.broken_spans.clone();
        for _ in 0..4 {
            springs.push(SpringCondition::Unknown);
            springs.extend_from_slice(&folded.springs);
            broken_spans.extend_from_slice(&folded.broken_spans);
        }
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
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Equal => slots
            .iter()
            .zip(broken_spans.iter())
            .all(|(slot_size, broken_span)| slot_size >= broken_span),
    }
}

type PermutationsCache = HashMap<(Vec<SpringCondition>, Vec<usize>), Option<usize>>;

fn cached_possibilities(
    slice: &[SpringCondition],
    broken_spans: &[usize],
    cache: &mut PermutationsCache,
) -> Option<usize> {
    // I guess a borrowed slice would be a better choice of key for performance,
    // but then we would have to deal with the lifetimes
    let key = (slice.to_vec(), broken_spans.to_vec());
    if let Some(result) = cache.get(&key) {
        *result
    } else {
        let result = possibilities(slice, broken_spans, cache);
        cache.insert(key, result);
        result
    }
}

fn possibilities(
    slice: &[SpringCondition],
    broken_spans: &[usize],
    cache: &mut PermutationsCache,
) -> Option<usize> {
    // eat  whitespace:
    let mut slice = slice;
    while !slice.is_empty() && slice[0] == SpringCondition::Working {
        slice = &slice[1..]
    }

    let start_here = |mut sub_slice: &[SpringCondition], cache| {
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
        cached_possibilities(sub_slice, &broken_spans[1..], cache)
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
                start_here(slice, cache)
            }
            SpringCondition::Unknown => {
                let mut sum_possibilities = 0;
                // 1. assume broken:
                if let Some(result) = start_here(slice, cache) {
                    sum_possibilities += result;
                }
                // 2. assume working:
                if let Some(result) = cached_possibilities(&slice[1..], broken_spans, cache) {
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
            let mut cache = PermutationsCache::new();
            assert_eq!(
                expected,
                possibilities(&record.springs, &record.broken_spans, &mut cache),
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

    #[test]
    fn GIVEN_permutations_cache_WHEN_inserting_keys_from_slices_THEN_caches_correctly() {
        let mut cache = PermutationsCache::new();
        let key = (vec![SpringCondition::Broken], vec![1]);
        cache.insert(key.clone(), Some(234));
        assert_eq!(cache.get(&key), Some(&Some(234)));
    }

    #[test]
    fn GIVEN_recursive_example_WHEN_counting_possibilities_THEN_correct_answer_returned() {
        // this is actually a performance test - it will basically hang without recurisive caching.
        let record = Record::new_from_folded("..?.????#?????????? 1,1,1,1,1,4");
        let mut cache = PermutationsCache::new();
        assert_eq!(
            Some(3916284121),
            possibilities(&record.springs, &record.broken_spans, &mut cache)
        );
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

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(525152, part2(EXAMPLE_INPUT));
    }
}
