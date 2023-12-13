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

type Group = Vec<String>;

fn part1(contents: &str) -> usize {
    let mut groups = Vec::<Group>::new();
    let mut group = Group::new();
    let mut last_pushed = false;
    for line in contents.lines() {
        if line.trim().is_empty() {
            groups.push(group);
            group = Group::new();
            last_pushed = true;
        } else {
            group.push(line.to_owned());
            last_pushed = false;
        }
    }
    if !last_pushed {
        groups.push(group);
    }
    groups.into_iter().map(process_group).sum()
}

fn part2(_contents: &str) -> usize {
    0
}

fn transpose(group: Group) -> Group {
    let mut vlines = Vec::<Vec<char>>::new();
    for (i, line) in group.iter().enumerate() {
        line.chars().enumerate().for_each(|(j, c)| {
            if 0 == i {
                vlines.push(vec![c]);
            } else {
                vlines[j].push(c);
            }
        });
    }
    vlines.iter().map(String::from_iter).collect::<Vec<_>>()
}

fn process_group(horizontal_lines: Group) -> usize {
    let tester = |lines: &[String]| {
        'l1: for (idx, line) in lines.iter().enumerate().skip(1) {
            if *line == lines[idx - 1] {
                let num = (lines.len() - (idx + 1)).min(idx - 2);
                for i in 0..num {
                    let l1 = &lines[idx + i + 1];
                    let ls = &lines[idx - i - 2];
                    if l1 != ls {
                        continue 'l1;
                    }
                }
                return Some(idx);
            }
        }
        None
    };

    println!("horizontal_lines:");
    for line in horizontal_lines.iter() {
        println!("{line}");
    }
    if let Some(idx) = tester(&horizontal_lines) {
        return idx * 100;
    }
    println!("vertical:");
    let vertical_lines = transpose(horizontal_lines);
    for line in vertical_lines.iter() {
        println!("{line}");
    }
    if let Some(idx) = tester(&vertical_lines) {
        return idx * 100;
    }
    panic!("no reflections found");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test13 {
    use super::*;

    #[test]
    fn GIVEN_horizontal_lines_WHEN_transposing_THEN_transposed_lines_returned() {
        let h_lines = vec!["123".to_owned(), "456".to_owned()];
        let expected = vec!["14".to_owned(), "25".to_owned(), "36".to_owned()];
        assert_eq!(expected, transpose(h_lines));
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
}
