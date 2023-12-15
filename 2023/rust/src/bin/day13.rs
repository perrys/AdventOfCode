//!
//! Advent of code challenge 2023 day 12.
//!
//! See <https://adventofcode.com/2023/day/12>
//!
use std::{cell::Cell, fs};

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

type Group = Vec<Vec<char>>;
type MutGroup = Vec<Vec<Cell<char>>>;

fn part1(contents: &str) -> usize {
    let groups = get_groups(contents);
    groups.into_iter().map(process_group).sum()
}

fn part2(contents: &str) -> usize {
    let to_mut_group = |group: Group| {
        group
            .into_iter()
            .map(|line| line.iter().map(|&c| Cell::new(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    let groups = get_groups(contents);
    let groups = groups.into_iter().map(to_mut_group).collect::<Vec<_>>();

    groups
        .into_iter()
        .enumerate()
        .map(|(idx, g)| {
            process_smudged_group(g)
                .unwrap_or_else(|| panic!("unable to find reflection for {idx}"))
        })
        .sum()
}

fn get_groups(contents: &str) -> Vec<Group> {
    let mut groups = Vec::<Group>::new();
    let mut group = Group::new();
    let mut last_pushed = false;
    for line in contents.lines() {
        if line.trim().is_empty() {
            groups.push(group);
            group = Group::new();
            last_pushed = true;
        } else {
            group.push(Vec::from_iter(line.chars()));
            last_pushed = false;
        }
    }
    if !last_pushed {
        groups.push(group);
    }
    groups
}

fn transpose<T>(group: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut vlines = Vec::<Vec<T>>::new();
    for (i, line) in group.iter().enumerate() {
        line.iter().enumerate().for_each(|(j, c)| {
            if 0 == i {
                vlines.push(vec![c.clone()]);
            } else {
                vlines[j].push(c.clone());
            }
        });
    }
    vlines
}

fn test_symmetry<T>(lines: &[T], ignore: Option<usize>) -> Option<usize>
where
    T: PartialOrd,
{
    'l1: for (idx, line) in lines.iter().enumerate().skip(1) {
        if *line == lines[idx - 1] {
            let num = (lines.len() - idx - 1).min(idx - 1);
            for i in 0..num {
                let l1 = &lines[idx + i + 1];
                let ls = &lines[idx - i - 2];
                if l1 != ls {
                    continue 'l1;
                }
            }
            if let Some(score) = ignore {
                if idx == score {
                    continue 'l1;
                }
            }
            return Some(idx);
        }
    }
    None
}

fn process_group(horizontal_lines: Group) -> usize {
    if let Some(idx) = test_symmetry(&horizontal_lines, None) {
        return idx * 100;
    }
    let vertical_lines = transpose(&horizontal_lines);
    if let Some(idx) = test_symmetry(&vertical_lines, None) {
        return idx;
    }
    panic!("no reflections found");
}

fn process_smudged_group(horizontal_lines: MutGroup) -> Option<usize> {
    let vertical_lines = transpose(&horizontal_lines);
    let mut original_score = test_symmetry(&horizontal_lines, None);
    if let Some(score) = original_score {
        original_score = Some(score * 100);
    }
    if original_score.is_none() {
        original_score = test_symmetry(&vertical_lines, None);
    }
    let original_score = original_score
        .unwrap_or_else(|| panic!("unable to calc original score for {:?}", &horizontal_lines));

    let flip = |c: &Cell<char>| {
        let new_c = if c.get() == '#' { '.' } else { '#' };
        c.replace(new_c);
    };
    let to_ignore = match original_score {
        _ if original_score >= 100 => Some(original_score / 100),
        _ => None,
    };
    for line in horizontal_lines.iter() {
        for c in line.iter() {
            flip(c);
            let result = test_symmetry(&horizontal_lines, to_ignore);
            flip(c);
            if let Some(score) = result {
                return Some(score * 100);
            }
        }
    }
    let to_ignore = match original_score {
        _ if original_score < 100 => Some(original_score),
        _ => None,
    };
    for line in vertical_lines.iter() {
        for c in line.iter() {
            flip(c);
            let result = test_symmetry(&vertical_lines, to_ignore);
            flip(c);
            if let Some(score) = result {
                return Some(score);
            }
        }
    }
    None
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test13 {
    use super::*;

    fn to_vec(s: &str) -> Vec<char> {
        Vec::from_iter(s.chars())
    }

    #[test]
    #[rustfmt::skip]
    fn GIVEN_symmetrical_line_groups_WHEN_testing_symmetry_THEN_correct() {
        let dotest = |expected, lines: &[&str]| {
            let owned_lines = lines.iter().map(|&s| to_vec(s)).collect::<Vec<_>>();
            assert_eq!(expected, test_symmetry(&owned_lines, None));
        };
        dotest(Some(2), &["0000",
                          "1111",
                          "1111"]);
        dotest(Some(3), &["0000",
                          "1111",
                          "2222",
                          "2222",
                          "1111"]);
        dotest(None, &["1111",
                       "2222",
                       "2222",
                       "3333"]);
    }

    #[test]
    fn GIVEN_horizontal_lines_WHEN_transposing_THEN_transposed_lines_returned() {
        let h_lines = vec![to_vec("123"), to_vec("456")];
        let expected = vec![to_vec("14"), to_vec("25"), to_vec("36")];
        assert_eq!(expected, transpose(&h_lines));
    }

    static EXAMPLE_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(405, part1(EXAMPLE_INPUT));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(400, part2(EXAMPLE_INPUT));
    }

    static EXAMPLE_INPUT2: &str = r#"#.#...#...##...##
#.#...#...##...##
#....##....#.....
##...##...##.##..
#..##..####....##
.....#.##..#.###.
#.......###..#...
..####.#.#.###..#
#..###.#.#.#####.
.....#.###....###
.....#.####...###"#;

    #[test]
    fn GIVEN_same_axis_smudge_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(1000, part2(EXAMPLE_INPUT2));
    }
}
