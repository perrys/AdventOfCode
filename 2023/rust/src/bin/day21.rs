//!
//! Advent of code challenge 2023 day 21.
//!
//! See <https://adventofcode.com/2023/day/21>
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
    println!("part2 total is {}", part2(contents.as_str(), 26501365));
}

fn part1(contents: &str, nsteps: usize) -> usize {
    let grid = parse_grid(contents);
    count_reachable_in_even_or_odd_steps(grid, nsteps)
}

fn part2(contents: &str, nsteps: usize) -> usize {
    // the target nsteps is an integer number of grid widths from the start
    // point in both x and y directions, which are clear of obstacles. The part1
    // answer for multiples of grid lengths is quadratic in nsteps, so can be
    // extrapolated from the first 3 results (thanks Reddit).
    let grid = parse_grid(contents);
    let nrows = grid.len();
    let start = find_start(&grid).expect("no start point");
    assert_eq!(start.row_idx, nsteps as isize % nrows as isize);
    let nwidths = (nsteps - start.row_idx as usize) / nrows;

    let fit_points = (0..3)
        .map(|n| {
            let grid = scale_grid(contents, 1 + n * 2);
            let nsteps = start.row_idx as usize + nrows * n;
            let npoints = count_reachable_in_even_or_odd_steps(grid, nsteps);
            (n as f64, npoints as f64)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let (a, b, c) = quadratic_fit(fit_points);
    let x = nwidths as f64;
    (a * x * x + b * x + c) as usize
}

fn quadratic_fit(points: [(f64, f64); 3]) -> (f64, f64, f64) {
    // formula from https://www.mathcelebrity.com/3ptquad.php
    let (x0, y0) = points[0];
    let (x1, y1) = points[1];
    let (x2, y2) = points[2];

    let delta = (x0 * x0 * x1) + (x0 * x2 * x2) + (x1 * x1 * x2)
        - (x1 * x2 * x2)
        - (x0 * x0 * x2)
        - (x0 * x1 * x1);
    let a_num = (y0 * x1) + (x0 * y2) + (y1 * x2) - (x1 * y2) - (y0 * x2) - (x0 * y1);
    let b_num = (x0 * x0 * y1) + (y0 * x2 * x2) + (x1 * x1 * y2)
        - (y1 * x2 * x2)
        - (x0 * x0 * y2)
        - (y0 * x1 * x1);
    let c_num = (x0 * x0 * x1 * y2) + (x0 * y1 * x2 * x2) + (y0 * x1 * x1 * x2)
        - (y0 * x1 * x2 * x2)
        - (x0 * x0 * y1 * x2)
        - (x0 * x1 * x1 * y2);
    (a_num / delta, b_num / delta, c_num / delta)
}

fn scale_grid(contents: &str, scale: usize) -> Vec<Vec<Point>> {
    assert!(scale.is_odd());
    let mut grid = parse_grid(contents);
    let start = find_start(&grid).expect("no start point");
    if let Point::Garden {
        is_start,
        nsteps: _,
    } = &mut grid[start.row_idx as usize][start.col_idx as usize]
    {
        *is_start = false;
    } else {
        panic!("start is not a start point");
    }

    let ncols = grid[0].len();
    let nrows = grid.len();
    let wide_grid = grid
        .into_iter()
        .map(|row| {
            let mut wide_row = Vec::new();
            for _ in 0..scale {
                wide_row.extend(row.clone());
            }
            wide_row
        })
        .collect::<Vec<_>>();
    let mut result = Vec::with_capacity(nrows * scale);
    for _ in 0..scale {
        result.extend(wide_grid.clone());
    }
    if let Point::Garden {
        is_start,
        nsteps: _,
    } = &mut result[scale / 2 * nrows + start.row_idx as usize]
        [scale / 2 * ncols + start.col_idx as usize]
    {
        *is_start = true;
    } else {
        panic!("wide_start is not a start point")
    }
    result
}

fn parse_grid(contents: &str) -> Vec<Vec<Point>> {
    let grid = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();
    grid
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

fn count_reachable_in_even_or_odd_steps(mut grid: Vec<Vec<Point>>, max_steps: usize) -> usize {
    breadth_first_search(&mut grid, max_steps);
    let predicate = |n: usize| if max_steps.is_even() { n.is_even() } else { !n.is_even() };

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|&pt| {
                    if let Point::Garden {
                        is_start: _,
                        nsteps: Some(n),
                    } = pt
                    {
                        predicate(*n)
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

    fn approx_equal(a: f64, b: f64, dp: u8) -> bool {
        let p = 10f64.powi(-(dp as i32));
        (a - b).abs() < p
    }

    #[test]
    fn GIVEN_quadratic_WHEN_fitting_3_points_THEN_coefficients_match() {
        let a = 2.5;
        let b = -5.0;
        let c = 27.0;
        let quad = |x| a * x * x + b * x + c;
        let xvals = [2.5, 7.9, -3.0];
        let mut points = [(0.0, 0.0); 3];
        for (i, x) in xvals.iter().enumerate() {
            points[i] = (*x, quad(*x));
        }
        let (aprime, bprime, cprime) = quadratic_fit(points);
        assert!(approx_equal(a, aprime, 11));
        assert!(approx_equal(b, bprime, 11));
        assert!(approx_equal(c, cprime, 11));
    }

    #[test]
    fn GIVEN_scaled_grids_WHEN_counting_rechable_points_THEN_matches_part2_examples() {
        let grid = EXAMPLE
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_line)
            .collect::<Vec<_>>();
        let nrows = grid.len();
        let dotest = |nsteps, expected| {
            let wide_grid = scale_grid(EXAMPLE, 1 + 2 * (nsteps / nrows));
            assert_eq!(
                expected,
                count_reachable_in_even_or_odd_steps(wide_grid, nsteps)
            );
        };
        // this takes quite a few seconds:
        dotest(500, 167004);
        dotest(1000, 668697);
        dotest(5000, 16733044);
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
