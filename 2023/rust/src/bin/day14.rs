//!
//! Advent of code challenge 2023 day 14.
//!
//! See <https://adventofcode.com/2023/day/14>
//!
use std::{cell::Cell, collections::HashMap, fs};

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
    let columns = read_columns(contents);
    columns.iter().for_each(|c| tilt_column(c, true));
    columns.iter().map(get_score).sum()
}

fn part2(contents: &str) -> usize {
    let columns = read_columns(contents);
    type CycleMap = HashMap<String, usize>;
    let mut cycle_map = CycleMap::new();
    let mut cycle = |cycle_idx, found: bool| {
        columns.iter().for_each(|c| tilt_column(c, true));
        (0..columns[0].len()).for_each(|row_idx| tilt_row(&columns, row_idx, true));
        columns.iter().for_each(|c| tilt_column(c, false));
        (0..columns[0].len()).for_each(|row_idx| tilt_row(&columns, row_idx, false));
        if !found {
            let key = cols_to_string(&columns);
            let previous = cycle_map.get(&key);
            if let Some(&n) = previous {
                //println!("{n}: {key}");
                return Some(n);
            }
            cycle_map.insert(key, cycle_idx);
        }
        None::<usize>
    };
    let loop_count: usize = 1000000000;
    let mut cycle_idx = 0;
    let mut found = false;
    while cycle_idx < loop_count {
        if let Some(previous_idx) = cycle(cycle_idx, found) {
            found = true;
            let cycle_length = cycle_idx - previous_idx;
            println!("Found cycle of length {cycle_length} at index {cycle_idx}");
            let remain = loop_count - cycle_idx;
            let skips = remain / cycle_length;
            cycle_idx += skips * cycle_length;
        }
        cycle_idx += 1;
    }
    columns.iter().map(get_score).sum()
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

fn tilt_column(col: &Column, to_north: bool) {
    let iter: Box<dyn Iterator<Item = &Cell<char>>> = if to_north {
        Box::new(col.iter())
    } else {
        Box::new(col.iter().rev())
    };
    let mut last_stop = 0;
    for (idx, c) in iter.enumerate() {
        match c.get() {
            '#' => last_stop = idx + 1,
            'O' => {
                if idx != last_stop {
                    let last = if to_north {
                        &col[last_stop]
                    } else {
                        &col[col.len() - last_stop - 1]
                    };
                    last.replace('O');
                    c.replace('.');
                }
                last_stop += 1;
            }
            _ => (),
        }
    }
}

fn tilt_row(cols: &[Column], row_idx: usize, to_west: bool) {
    let mut last_stop = if to_west { 0 } else { cols.len() - 1 };
    for i in 0..cols.len() {
        let col_idx = if to_west { i } else { cols.len() - i - 1 };
        let c = &cols[col_idx][row_idx];
        match c.get() {
            '#' => {
                last_stop = if to_west {
                    col_idx + 1
                } else {
                    col_idx.saturating_sub(1)
                }
            }
            'O' => {
                if col_idx != last_stop {
                    let last = &cols[last_stop][row_idx];
                    last.replace('O');
                    c.replace('.');
                }
                last_stop = if to_west {
                    last_stop + 1
                } else {
                    last_stop.saturating_sub(1)
                };
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

#[allow(dead_code)] // for testing
fn to_string(col: &Column) -> String {
    String::from_iter(col.iter().map(|c| c.take()))
}

fn cols_to_string(cols: &[Column]) -> String {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for (idx, col) in cols.iter().enumerate() {
        if 0 == idx {
            for c in col.iter() {
                rows.push(vec![c.get()]);
            }
        } else {
            for (row_idx, c) in col.iter().enumerate() {
                rows[row_idx].push(c.get());
            }
        }
    }
    rows.iter()
        .map(|r| String::from_iter(r.iter()))
        .collect::<Vec<_>>()
        .join("\n")
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
            let col = columns.chars().map(Cell::new).collect::<Vec<_>>();
            tilt_column(&col, true);
            assert_eq!(expected, to_string(&col).as_str());
        };
        dotest(".O", "O.");
        dotest(".#.O#....O", ".#O.#O....");
        dotest(".#.O#....O", ".#O.#O....");
        dotest("OO.O.O..##", "OOOO....##");
    }

    #[test]
    fn GIVEN_column_WHEN_tilting_south_THEN_rocks_roll_down() {
        let dotest = |column: &str, expected: &str| {
            let col = column.chars().map(Cell::new).collect::<Vec<_>>();
            tilt_column(&col, false);
            assert_eq!(expected, to_string(&col).as_str());
        };
        dotest(".O", ".O");
        dotest(".#.O#....O", ".#.O#....O");
        dotest(".#O.#...O.", ".#.O#....O");
        dotest("OO.O.O..##", "....OOOO##");
    }
    #[test]
    fn GIVEN_columns_WHEN_tilting_west_THEN_rocks_roll_left() {
        let dotest = |columns: &str, expected: &str| {
            let cols = columns
                .chars()
                .map(|c| vec![Cell::new(c)])
                .collect::<Vec<_>>();
            tilt_row(&cols, 0, true);
            assert_eq!(
                expected,
                to_string(&cols.iter().map(|c| c[0].clone()).collect::<Vec<_>>()).as_str()
            );
        };
        dotest(".O", "O.");
        dotest(".#.O#....O", ".#O.#O....");
        dotest(".#.O#....O", ".#O.#O....");
        dotest("OO.O.O..##", "OOOO....##");
    }
    #[test]
    fn GIVEN_columns_WHEN_tilting_east_THEN_rocks_roll_right() {
        let dotest = |columns: &str, expected: &str| {
            let cols = columns
                .chars()
                .map(|c| vec![Cell::new(c)])
                .collect::<Vec<_>>();
            tilt_row(&cols, 0, false);
            assert_eq!(
                expected,
                to_string(&cols.iter().map(|c| c[0].clone()).collect::<Vec<_>>()).as_str()
            );
        };
        dotest("O.", ".O");
        dotest("#O.", "#.O"); // tests for underflow
        dotest(".#.O#..O.O", ".#.O#...OO");
        dotest(".#O.#...O.", ".#.O#....O");
        dotest("OO.O.O..##", "....OOOO##");
    }

    #[test]
    fn GIVEN_column_WHEN_scoring_THEN_correct_score_returned() {
        let dotest = |columns: &str, expected: usize| {
            let col = columns.chars().map(Cell::new).collect::<Vec<_>>();
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
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(136, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(64, part2(EXAMPLE));
    }
}
