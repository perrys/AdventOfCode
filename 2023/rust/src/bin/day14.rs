//!
//! Advent of code challenge 2023 day 14.
//!
//! See <https://adventofcode.com/2023/day/14>
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

fn part1(contents: &str) -> usize {
    let mut columns = read_columns(contents);
    columns.iter_mut().for_each(tilt_column);
    columns.iter().map(get_score).sum()
}

fn part2(_contents: &str) -> usize {
    0
}

type Column = Vec<Cell<char>>;

fn get_score(col: &Column) -> usize {
    col.iter()
        .enumerate()
        .map(|(idx, c)| {
            let c = c.get();
            match c {
                'O' => col.len() - idx,
                _ => 0,
            }
        })
        .sum()
}

fn tilt_column(col: &mut Column) {
    let mut last_stop = 0;
    for (idx, c) in col.iter().enumerate() {
        match c.get() {
            '#' => last_stop = idx + 1,
            'O' => {
                if idx != last_stop {
                    col[last_stop].replace('O');
                    c.replace('.');
                }
                last_stop += 1;
            }
            _ => (),
        }
    }
}

fn read_columns(contents: &str) -> Vec<Column> {
    let mut vlines = Vec::<Column>::new();
    for (row_idx, line) in contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        line.chars().enumerate().for_each(|(j, c)| {
            if 0 == row_idx {
                vlines.push(vec![Cell::new(c)]);
            } else {
                vlines[j].push(Cell::new(c));
            }
        });
    }
    vlines
}

fn to_string(col: &Column) -> String {
    String::from_iter(col.iter().map(|c| c.take()))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test14 {
    use super::*;

    #[test]
    fn GIVEN_row_content_WHEN_parsing_THEN_columns_produced() {
        let example = r#"
1234
5678
"#;
        let columns = read_columns(example);
        assert_eq!(4, columns.len());
        assert_eq!("15", to_string(&columns[0]));
        assert_eq!("48", to_string(&columns[3]));
    }

    #[test]
    fn GIVEN_column_WHEN_tilting_THEN_rocks_roll_up() {
        let dotest = |columns: &str, expected: &str| {
            let mut col = columns.chars().map(|c| Cell::new(c)).collect::<Vec<_>>();
            tilt_column(&mut col);
            assert_eq!(expected, to_string(&col).as_str());
        };
        dotest(".O", "O.");
        dotest(".#.O#....O", ".#O.#O....");
        dotest(".#.O#....O", ".#O.#O....");
        dotest("OO.O.O..##", "OOOO....##");
    }

    #[test]
    fn GIVEN_column_WHEN_scoring_THEN_correct_score_returned() {
        let dotest = |columns: &str, expected: usize| {
            let col = columns.chars().map(|c| Cell::new(c)).collect::<Vec<_>>();
            assert_eq!(get_score(&col), expected);
        };
        dotest("#", 0);
        dotest("O", 1);
        dotest("#O.", 2);
        dotest("#OO...#O..", 20);
    }
    static EXAMPLE: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(136, part1(EXAMPLE));
    }
}
