//!
//! Advent of code challenge 2123 day 21.
//!
//! See <https://adventofcode.com/2123/day/21>
//!
use std::{collections::VecDeque, fs};

use num::Integer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str(), 64));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str, nsteps: usize) -> usize {
    let grid = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();
    count_reachable_in_even_steps(grid, nsteps)
}

fn part2(_contents: &str) -> usize {
    0
}

#[derive(Debug, Eq, PartialEq)]
enum Point {
    Garden {
        is_start: bool,
        nsteps: Option<usize>,
    },
    Rock,
}

impl Point {
    fn to_char(&self) -> char {
        match self {
            Self::Garden { is_start, nsteps } => {
                if let Some(n) = nsteps {
                    (b'0' + *n as u8) as char // only works properly for n < 10
                } else if *is_start {
                    'S'
                } else {
                    '.'
                }
            }
            Point::Rock => '#',
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ExPoint {
    row_idx: isize,
    col_idx: isize,
    nsteps: usize,
}

fn parse_line(line: &str) -> Vec<Point> {
    line.chars()
        .map(|c| match c {
            '.' => Point::Garden {
                is_start: false,
                nsteps: None,
            },
            '#' => Point::Rock,
            'S' => Point::Garden {
                is_start: true,
                nsteps: None,
            },
            _ => panic!("unknown point type '{c}' in {line}"),
        })
        .collect::<Vec<_>>()
}

fn count_reachable_in_even_steps(mut grid: Vec<Vec<Point>>, max_steps: usize) -> usize {
    breadth_first_search(&mut grid, max_steps);

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|&pt| {
                    if let Point::Garden {
                        is_start: _,
                        nsteps: Some(n),
                    } = pt
                    {
                        n.is_even()
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

fn breadth_first_search(grid: &mut Vec<Vec<Point>>, max_steps: usize) {
    let ncols = grid[0].len();
    let nrows = grid.len();
    let check_bounds = |ex_pt: &ExPoint| {
        ex_pt.col_idx >= 0
            && ex_pt.col_idx < ncols as isize
            && ex_pt.row_idx >= 0
            && ex_pt.row_idx < nrows as isize
            && ex_pt.nsteps <= max_steps
    };
    if let Some(start) = find_start(grid) {
        let mut queue = VecDeque::<ExPoint>::new();
        queue.push_back(start);
        'l1: while !queue.is_empty() {
            let ex_pt = queue.pop_front().unwrap();
            if check_bounds(&ex_pt) {
                let point = &mut grid[ex_pt.row_idx as usize][ex_pt.col_idx as usize];
                match point {
                    Point::Rock => continue 'l1,
                    Point::Garden {
                        is_start: _,
                        nsteps,
                    } => {
                        if nsteps.is_none() {
                            *nsteps = Some(ex_pt.nsteps);
                            queue.push_back(ExPoint {
                                row_idx: ex_pt.row_idx,
                                col_idx: ex_pt.col_idx + 1,
                                nsteps: ex_pt.nsteps + 1,
                            });
                            queue.push_back(ExPoint {
                                row_idx: ex_pt.row_idx,
                                col_idx: ex_pt.col_idx - 1,
                                nsteps: ex_pt.nsteps + 1,
                            });
                            queue.push_back(ExPoint {
                                row_idx: ex_pt.row_idx + 1,
                                col_idx: ex_pt.col_idx,
                                nsteps: ex_pt.nsteps + 1,
                            });
                            queue.push_back(ExPoint {
                                row_idx: ex_pt.row_idx - 1,
                                col_idx: ex_pt.col_idx,
                                nsteps: ex_pt.nsteps + 1,
                            });
                        }
                    }
                }
            }
        }
    } else {
        panic!("Couldn't find start point");
    }
}

fn find_start(grid: &[Vec<Point>]) -> Option<ExPoint> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, point) in row.iter().enumerate() {
            if let Point::Garden {
                is_start,
                nsteps: _,
            } = point
            {
                if *is_start {
                    return Some(ExPoint {
                        row_idx: row_idx as isize,
                        col_idx: col_idx as isize,
                        nsteps: 0,
                    });
                }
            }
        }
    }
    None
}

#[allow(dead_code)] // for debugging
fn print_grid(grid: &[Vec<Point>]) {
    for row in grid.iter() {
        println!("{}", row.iter().map(Point::to_char).collect::<String>());
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test21 {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn GIVEN_small_grid_WHEN_bfs_searching_THEN_correct_points_reached() {
        let mut grid = EXAMPLE
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_line)
            .collect::<Vec<_>>();
        breadth_first_search(&mut grid, 3);
        // squares reached in the first three iterations of the example:
        let expected: HashSet<(usize, usize, usize)> = HashSet::from([
            (0, 5, 5),
            (1, 4, 5),
            (1, 5, 4),
            (2, 3, 5),
            (2, 5, 3),
            (2, 6, 4),
            (3, 3, 6),
            (3, 4, 3),
            (3, 6, 3),
            (3, 7, 4),
        ]);
        let mut found = HashSet::new();
        grid.iter().enumerate().for_each(|(row_idx, line)| {
            line.iter().enumerate().for_each(|(col_idx, pt)| {
                if let Point::Garden {
                    is_start: _,
                    nsteps: Some(n),
                } = pt
                {
                    found.insert((*n, row_idx, col_idx));
                }
            })
        });
        assert_eq!(expected, found);
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(16, part1(EXAMPLE, 6));
    }

    static EXAMPLE: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
}
