//!
//! Advent of code challenge 2023 day 18.
//!
//! See <https://adventofcode.com/2023/day/18>
//!
use std::{fs, ops::RangeInclusive};

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
    // Solved using a bitmap and flood-fill algorithm, similar to Day 10.
    use FillSegment::*;
    let trenches = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Trench::new)
        .collect::<Vec<_>>();

    let (xrange, yrange) = get_bounds(&trenches);
    let nrows = (1 + yrange.end() - yrange.start()) as usize;
    let ncols = (1 + xrange.end() - xrange.start()) as usize;
    let mut rows = Vec::<Vec<FillSegment>>::with_capacity(nrows);
    for _ in 0..nrows {
        rows.push(vec![Unknown; ncols]);
    }

    let start_point = (
        xrange.start().unsigned_abs() as usize,
        yrange.start().unsigned_abs() as usize,
    );

    trace_path(&trenches, start_point, |point, _dir| {
        rows[point.1][point.0] = Path;
    });

    let mut idx = 0;
    trace_path(&trenches, start_point, |point, dir| {
        if true {
            flood_fill(point, dir, &mut rows);
        }
        idx += 1;
    });

    rows.iter()
        .map(|row| row.iter().filter(|&&c| c == Path || c == RightSide).count())
        .sum()
}

fn part2(contents: &str) -> usize {
    // The bitmap approach cannot be used here due to the very large numbers
    // involved. Instead this is solved using a polygon area formula - the main
    // difficulty with this is accounting for the width of the trench (see
    // below).
    let trenches = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Trench::new_from_color)
        .collect::<Vec<_>>();

    let vertices = get_outside_vertices(&trenches);
    area_from_shoelace_formula(&vertices) as usize
}

fn get_outside_vertices(trenches: &[Trench]) -> Vec<(i64, i64)> {
    // the perimiter of the trench is one unit larger for outside corners and
    // one unit smaller for inside corners. The following assumes clockwise
    // rotation.
    let mut vertices = Vec::<(i64, i64)>::with_capacity(trenches.len() + 1);
    use Direction::*;
    let mut point = (0, 0);
    for (idx, trench) in trenches.iter().enumerate() {
        let prev = &trenches[if idx == 0 { trenches.len() - 1 } else { idx - 1 }];
        let next = &trenches[if idx == trenches.len() - 1 { 0 } else { idx + 1 }];
        let len = trench.length as i64;
        let vec = match trench.direction {
            Up => {
                let offset = match (prev.direction, next.direction) {
                    (Left, Right) => 1,
                    (Right, Left) => -1,
                    _ => 0,
                };
                (0, len + offset)
            }
            Down => {
                let offset = match (prev.direction, next.direction) {
                    (Right, Left) => 1,
                    (Left, Right) => -1,
                    _ => 0,
                };
                (0, -len - offset)
            }
            Left => {
                let offset = match (prev.direction, next.direction) {
                    (Down, Up) => 1,
                    (Up, Down) => -1,
                    _ => 0,
                };
                (-len - offset, 0)
            }
            Right => {
                let offset = match (prev.direction, next.direction) {
                    (Up, Down) => 1,
                    (Down, Up) => -1,
                    _ => 0,
                };
                (len + offset, 0)
            }
        };
        point.0 += vec.0;
        point.1 += vec.1;
        vertices.push(point);
    }
    vertices
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum FillSegment {
    LeftSide,
    RightSide,
    Path,
    Unknown,
}

fn flood_fill(start_point: (usize, usize), direction: Direction, rows: &mut [Vec<FillSegment>]) {
    use Direction::*;
    use FillSegment::*;
    let dy = 0..rows.len();
    let dx = 0..(rows[0].len());
    let (lhs, rhs) = match direction {
        Up => ((-1, 0), (1, 0)),
        Down => ((1, 0), (-1, 0)),
        Left => ((0, -1), (0, 1)),
        Right => ((0, 1), (0, -1)),
    };
    let mut point = start_point;
    point.0 = (point.0 as i64 + lhs.0) as usize;
    point.1 = (point.1 as i64 + lhs.1) as usize;
    while dy.contains(&(point.1)) && dx.contains(&(point.0)) {
        let slot = &mut rows[point.1][point.0];
        match *slot {
            Unknown => *slot = LeftSide,
            Path => break,
            LeftSide => (),
            RightSide => panic!("inconsistent fill segments"),
        }
        point.0 = (point.0 as i64 + lhs.0) as usize;
        point.1 = (point.1 as i64 + lhs.1) as usize;
    }
    let mut point = start_point;
    point.0 = (point.0 as i64 + rhs.0) as usize;
    point.1 = (point.1 as i64 + rhs.1) as usize;
    while dy.contains(&(point.1)) && dx.contains(&(point.0)) {
        let slot = &mut rows[point.1][point.0];
        match *slot {
            Unknown => *slot = RightSide,
            Path => break,
            RightSide => (),
            LeftSide => panic!("inconsistent fill segments"),
        }
        point.0 = (point.0 as i64 + rhs.0) as usize;
        point.1 = (point.1 as i64 + rhs.1) as usize;
    }
}

fn trace_path<F>(trenches: &[Trench], start_point: (usize, usize), mut callback: F)
where
    F: FnMut((usize, usize), Direction),
{
    use Direction::*;
    let mut point = start_point;
    for trench in trenches.iter() {
        let vec = match trench.direction {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        };
        callback(point, trench.direction);
        for _ in 0..trench.length {
            point.0 = (point.0 as i64 + vec.0) as usize;
            point.1 = (point.1 as i64 + vec.1) as usize;
            callback(point, trench.direction);
        }
    }
}

fn get_bounds(trenches: &[Trench]) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    use Direction::*;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    for trench in trenches.iter() {
        match trench.direction {
            Up => y += trench.length as i64,
            Down => y -= trench.length as i64,
            Left => x -= trench.length as i64,
            Right => x += trench.length as i64,
        }
        min_x = x.min(min_x);
        min_y = y.min(min_y);
        max_x = x.max(max_x);
        max_y = y.max(max_y);
    }
    (min_x..=max_x, min_y..=max_y)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(c: &str) -> Self {
        match c {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown direction '{c}'"),
        }
    }
}

#[derive(Debug)]
struct Trench {
    direction: Direction,
    length: usize,
}

impl Trench {
    fn new(line: &str) -> Self {
        let mut toks = line.split(' ');
        let direction = Direction::new(
            toks.next()
                .unwrap_or_else(|| panic!("no tokens in line {line}")),
        );
        let length = toks
            .next()
            .unwrap_or_else(|| panic!("no length in {line}"))
            .parse::<usize>()
            .expect("non-numeric length");
        Self { direction, length }
    }

    fn new_from_color(line: &str) -> Self {
        let parse_color = |rgb: &str| {
            let rgb = rgb
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .collect::<String>();
            assert_eq!(6, rgb.len());
            let len = usize::from_str_radix(&rgb[0..5], 16).unwrap();
            use Direction::*;
            let dir = match &rgb[5..6] {
                "0" => Right,
                "1" => Down,
                "2" => Left,
                "3" => Up,
                _ => panic!("unknown direction {rgb}"),
            };
            (len, dir as Direction)
        };
        let mut toks = line.split(' ').skip(2);
        let (length, direction) =
            parse_color(toks.next().unwrap_or_else(|| panic!("no color in {line}")));
        Self { direction, length }
    }
}

fn area_from_shoelace_formula(vertices: &[(i64, i64)]) -> i64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let total: i64 = vertices
        .iter()
        .enumerate()
        .map(|(idx, xy)| {
            let xyprime = vertices[if idx == 0 { vertices.len() - 1 } else { idx - 1 }];
            xy.0 * xyprime.1 - xy.1 * xyprime.0
        })
        .sum();
    total / 2
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test18 {
    use super::*;

    #[test]
    fn GIVEN_valid_line_WHEN_constructing_trench_THEN_correct_struct_returned() {
        let dotest = |line, dir, len| {
            let trench = Trench::new(line);
            assert_eq!(dir, trench.direction);
            assert_eq!(len, trench.length);
        };
        dotest(r"U 4 (#640bb1)", Direction::Up, 4);
    }

    #[test]
    fn GIVEN_valid_line_WHEN_constructing_trench_from_color_THEN_correct_struct_returned() {
        let dotest = |line, len, dir| {
            let trench = Trench::new_from_color(line);
            assert_eq!(dir, trench.direction);
            assert_eq!(len, trench.length);
        };
        dotest(r"U 4 (#640bb1)", 0x640bb, Direction::Down);
    }

    #[test]
    fn GIVEN_small_grid_WHEN_flood_filling_THEN_expected_slots_filled() {
        let grid = r#"
U 3 (#FFFFF0) 
R 3 (#FFFFF0) 
D 3 (#FFFFF0) 
L 3 (#FFFFF0) "#;
        let mut rows = Vec::<Vec<FillSegment>>::new();
        for _ in 0..6 {
            rows.push(vec![FillSegment::Unknown; 6]);
        }
        let trenches = grid
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(Trench::new)
            .collect::<Vec<_>>();
        let start_point = (1, 1);
        trace_path(&trenches, start_point, |point, _dir| {
            rows[point.1][point.0] = FillSegment::Path;
        });

        trace_path(&trenches, start_point, |point, dir| {
            flood_fill(point, dir, &mut rows);
        });
        for i in 1..5 {
            assert_eq!(FillSegment::Path, rows[1][i]);
            assert_eq!(FillSegment::Path, rows[4][i]);
            assert_eq!(FillSegment::Path, rows[i][1]);
            assert_eq!(FillSegment::Path, rows[i][4]);
            assert_eq!(FillSegment::LeftSide, rows[0][i]);
            assert_eq!(FillSegment::LeftSide, rows[5][i]);
            assert_eq!(FillSegment::LeftSide, rows[i][0]);
            assert_eq!(FillSegment::LeftSide, rows[i][5]);
        }
        for i in 2..4 {
            assert_eq!(FillSegment::RightSide, rows[2][i]);
            assert_eq!(FillSegment::RightSide, rows[3][i]);
            assert_eq!(FillSegment::RightSide, rows[i][2]);
            assert_eq!(FillSegment::RightSide, rows[i][3]);
        }
    }

    #[test]
    fn GIVEN_example_polygon_WHEN_calcing_area_THEN_correct_answer_returned() {
        let square = [(0, 0), (0, 3), (3, 3), (3, 0)];
        assert_eq!(9, area_from_shoelace_formula(&square));
        let square = [(2, 2), (2, -2), (-2, -2), (-2, 2)];
        assert_eq!(16, area_from_shoelace_formula(&square));
    }

    static EXAMPLE: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(62, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_calcing_part1_area_THEN_matches_part1_answer() {
        let trenches = EXAMPLE
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(Trench::new)
            .collect::<Vec<_>>();
        let vertices = get_outside_vertices(&trenches);
        assert_eq!(62, area_from_shoelace_formula(&vertices));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(952408144115, part2(EXAMPLE));
    }
}
