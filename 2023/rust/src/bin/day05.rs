//!
//! Advent of code challenge 2023 day 5.
//!
//! See <https://adventofcode.com/2023/day/5>
//!
use std::fs;

use std::ops::Range;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 result is {}", part1(contents.as_str()));
    println!("part2 result is {}", part2(contents.as_str()));
}

fn part1(content: &str) -> i64 {
    let mut lines = content.lines().filter(|s| !s.trim().is_empty());
    let seeds = parse_seeds(
        lines
            .next()
            .unwrap_or_else(|| panic!("no non-empty lines in file")),
    );
    let mut mapped = seeds.clone();
    let mut mappings = Mappings(Vec::new());
    let process_mapping = |mapped: &Vec<_>, mappings: &Mappings| {
        mapped.iter().map(|&val| mappings.map(val)).collect()
    };
    for line in lines {
        if line.contains("map:") {
            mapped = process_mapping(&mapped, &mappings);
            mappings.0.clear();
        } else {
            mappings.0.push(Mapping::new(line));
        }
    }
    mapped = process_mapping(&mapped, &mappings);
    *mapped.iter().min().expect("no seeds provided")
}

fn part2(content: &str) -> i64 {
    let mut lines = content.lines().filter(|s| !s.trim().is_empty());
    let seeds = parse_seed_ranges(
        lines
            .next()
            .unwrap_or_else(|| panic!("no non-empty lines in file")),
    );
    let lines = lines.collect::<Vec<_>>();

    let mut mapped = seeds.clone();
    let mut mappings = Mappings(Vec::new());
    for line in lines {
        if line.contains("map:") {
            mapped = mappings.map_range(&mapped);
            mappings.0.clear();
        } else {
            mappings.0.push(Mapping::new(line));
        }
    }
    mapped = mappings.map_range(&mapped);
    mapped
        .iter()
        .map(|r| r.start)
        .min()
        .expect("no seeds provided")
}

fn parse_seeds(line: &str) -> Vec<i64> {
    assert!(line.starts_with("seeds:"));
    line[7..]
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("couldn't parse {line}"))
}

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
    let tokens = parse_seeds(line);
    let (even, odd): (Vec<_>, Vec<_>) = tokens
        .into_iter()
        .enumerate()
        .partition(|(idx, _)| idx & 1 == 0);
    let (_, starts): (Vec<usize>, Vec<i64>) = even.into_iter().unzip();
    let (_, lengths): (Vec<usize>, Vec<i64>) = odd.into_iter().unzip();
    starts
        .iter()
        .zip(lengths.iter())
        .map(|(&s, &l)| (s..(s + l)))
        .collect::<Vec<_>>()
}

struct Mapping {
    src: Range<i64>,
    offset: i64,
}

impl Mapping {
    fn new(line: &str) -> Self {
        let toks = line
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| panic!("unable to parse u8s in {line}"));
        Self {
            offset: (toks[0] - toks[1]), // dest - src
            src: (toks[1]..toks[1] + toks[2]),
        }
    }
}

fn intersection<T>(r1: &Range<T>, r2: &Range<T>) -> Option<Range<T>>
where
    T: Ord + Copy,
{
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);
    if start <= end {
        Some(start..end)
    } else {
        None
    }
}

fn merge_ranges<T>(ranges: &[Range<T>]) -> Vec<Range<T>>
where
    T: Ord + Copy,
{
    let mut done = false;
    let mut ranges = Vec::from(ranges);
    while !done {
        done = true;
        let mut to_remove: Option<(usize, usize)> = None;
        'l1: for (idx1, r1) in ranges.iter().enumerate() {
            for (idx2, r2) in ranges.iter().enumerate().skip(idx1 + 1) {
                if intersection(r1, r2).is_some() {
                    to_remove = Some((idx1, idx2));
                    done = false;
                    break 'l1;
                }
            }
        }
        if let Some((idx1, idx2)) = to_remove {
            let r1 = &ranges[idx1];
            let r2 = &ranges[idx2];
            let start = r1.start.min(r2.start);
            let end = r1.end.max(r2.end);
            ranges.push(start..end);
            ranges.remove(idx2);
            ranges.remove(idx1);
        }
    }
    ranges.sort_by(|lhs, rhs| lhs.start.cmp(&rhs.start));
    ranges
}

struct Mappings(Vec<Mapping>);

impl Mappings {
    fn map(&self, src: i64) -> i64 {
        if let Some(mapping) = self.0.iter().find(|m| m.src.contains(&src)) {
            src + mapping.offset
        } else {
            src
        }
    }
    fn map_range(&self, srcs: &[Range<i64>]) -> Vec<Range<i64>> {
        let mut result: Vec<Range<i64>> = Vec::new();
        for src_range in srcs.iter() {
            let mut src_mapped_ranges: Vec<Range<i64>> = Vec::new();
            for mapping in self.0.iter() {
                if let Some(r) = intersection(src_range, &mapping.src) {
                    result.push((r.start + mapping.offset)..(r.end + mapping.offset));
                    src_mapped_ranges.push(r);
                }
            }
            // identity-map any unmapped src regions:
            let mut src_mapped_ranges = merge_ranges(&src_mapped_ranges);
            src_mapped_ranges.sort_by(|l, r| l.start.cmp(&r.start));
            let mut last = src_range.start;
            for r in src_mapped_ranges {
                if r.start > last {
                    result.push(last..r.start);
                }
                last = r.end;
            }
            if last < src_range.end {
                result.push(last..src_range.end);
            }
        }
        merge_ranges(&result)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tester {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn GIVEN_valid_lines_WHEN_constructing_mapping_THEN_matches_expected() {
        let dotest = |line, offset, range| {
            let m = Mapping::new(line);
            assert_eq!(offset, m.offset);
            assert_eq!(range, m.src);
        };
        dotest("2 1 3", 1, 1..4);
        dotest(" 5 2 100", 3, 2..102);
        dotest(
            "3985931590 3902342481 309035706",
            83589109,
            3902342481..4211378187,
        );
    }
    #[test]
    fn GIVEN_mappings_WHEN_mapping_THEN_matches_expected() {
        let mappings = Mappings(vec![Mapping::new("2 1 4"), Mapping::new("10 8 5")]);
        assert_eq!(2, mappings.map(1));
        assert_eq!(3, mappings.map(2));
        assert_eq!(0, mappings.map(0));
        assert_eq!(100, mappings.map(100));
        assert_eq!(10, mappings.map(8));
        assert_eq!(14, mappings.map(12));
        assert_eq!(13, mappings.map(13));
        assert_eq!(14, mappings.map(14));
    }

    #[test]
    fn GIVEN_valid_seed_line_WHEN_parsing_THEN_matches_expected() {
        assert_eq!(parse_seeds("seeds: 1 2 3 4"), vec![1, 2, 3, 4]);
    }

    #[test]
    fn GIVEN_valid_seed_line_WHEN_parsing_ranges_THEN_matches_expected() {
        assert_eq!(parse_seed_ranges("seeds: 1 2 3 4"), vec![(1..3), (3..7)]);
    }

    #[test]
    fn GIVEN_several_ranges_WHEN_testing_intersections_THEN_correct_intersections_produced() {
        let dotest = |r1, r2, expected: Option<Range<i32>>| {
            let overlap = intersection(r1, r2);
            if let Some(r3) = overlap {
                assert!(expected.is_some());
                assert_eq!(expected.unwrap(), r3);
            } else {
                assert!(expected.is_none());
            }
        };
        dotest(&(-10..0), &(2..4), None); // no overlap not adjacent
        dotest(&(-1..1), &(2..4), None); // no overlap adjacent
        dotest(&(1..2), &(2..4), Some(2..2)); // overlap r1 < r2
        dotest(&(1..3), &(-2..2), Some(1..2)); // overlap r2 < r1
        dotest(&(11..23), &(15..18), Some(15..18)); // r2 contained by r1
        dotest(&(11..23), &(1..50), Some(11..23)); // r1 contained by r2
    }

    #[test]
    fn GIVEN_vector_of_ranges_WHEN_merging_THEN_merged_ranges_produced() {
        let dotest = |ranges: Vec<Range<i32>>, expected: Vec<Range<i32>>| {
            let min = ranges.iter().map(|r| r.start).min().unwrap();
            let max = ranges.iter().map(|r| r.start).max().unwrap();
            let merged_ranges = merge_ranges(&ranges);
            // piecewise comparison across range:
            for idx in (min - 5)..(max + 5) {
                let orig_was_mapped = ranges.iter().any(|r| r.contains(&idx));
                let merged_is_mapped = merged_ranges.iter().any(|r| r.contains(&idx));
                assert_eq!(orig_was_mapped, merged_is_mapped);
            }
            assert_eq!(
                HashSet::<Range<i32>>::from_iter(merged_ranges),
                HashSet::from_iter(expected)
            );
        };
        dotest(vec![2..4, 2..6, 9..12, 1..6], vec![1..6, 9..12]);
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn GIVEN_src_ranges_and_mappings_WHEN_mapping_src_ranges_THEN_matches_brutforce_map() {
        let mappings = Mappings(Vec::<Mapping>::new());
        let sanity_check = [0..20i64, 0..3i64];
        let mapped = mappings.map_range(&sanity_check);
        assert_eq!(HashSet::from([0..20i64]), HashSet::from_iter(mapped));

        let mappings = Mappings(vec![Mapping::new("0 2 6"), Mapping::new("30 8 5")]);
        let src_ranges = [0..20i64, 40..60];
        let mapped = mappings.map_range(&src_ranges);
        let mut mapped_end_points = Vec::<i64>::new();
        for range in mapped.iter() {
            mapped_end_points.extend(range.clone());
        }
        mapped_end_points.sort();
        assert_eq!(
            HashSet::from([0..6i64, 13..20i64, 30..35i64, 40..60i64]),
            HashSet::from_iter(mapped)
        );
        // bruteforce:
        let start_points = src_ranges.into_iter().flatten().collect::<Vec<_>>();
        let mut end_points = start_points
            .iter()
            .map(|p| mappings.map(*p))
            .collect::<Vec<_>>();
        end_points.sort();
        end_points.dedup();
        assert_eq!(end_points, mapped_end_points);
    }

    static EXAMPLE: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(35, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(46, part2(EXAMPLE));
    }
}
