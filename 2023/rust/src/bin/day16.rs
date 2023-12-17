//!
//! Advent of code challenge 2023 day 16.
//!
//! See <https://adventofcode.com/2023/day/16>
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
    let rows = parse(contents);
    count_engergized_tiles(rows, (0, 0), Direction::East)
}

fn part2(contents: &str) -> usize {
    let mut max_score = 0;
    let rows = parse(contents);
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut dir = Direction::West;
    max_score = max_score.max(
        (0..nrows)
            .map(|idx| count_engergized_tiles(rows.clone(), (idx, 0), dir))
            .max()
            .unwrap_or_else(|| panic!("unable to find max of {:?} rows", dir)),
    );
    dir = Direction::East;
    max_score = max_score.max(
        (0..nrows)
            .map(|idx| count_engergized_tiles(rows.clone(), (idx, ncols - 1), dir))
            .max()
            .unwrap_or_else(|| panic!("unable to find max of {:?} rows", dir)),
    );
    dir = Direction::North;
    max_score = max_score.max(
        (0..nrows)
            .map(|idx| count_engergized_tiles(rows.clone(), (nrows - 1, idx), dir))
            .max()
            .unwrap_or_else(|| panic!("unable to find max of {:?} rows", dir)),
    );
    dir = Direction::South;
    max_score = max_score.max(
        (0..nrows)
            .map(|idx| count_engergized_tiles(rows.clone(), (0, idx), dir))
            .max()
            .unwrap_or_else(|| panic!("unable to find max of {:?} rows", dir)),
    );
    max_score
}

fn count_engergized_tiles(
    mut rows: Vec<Vec<Tile>>,
    start: CoOrd,
    direction_of_travel: Direction,
) -> usize {
    trace_beam(&mut rows, start, direction_of_travel);
    rows.iter()
        .map(|row| row.iter().filter(|t| !t.beams.is_empty()).count())
        .sum()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TileType {
    Empty,
    MirrorFwdSlash,
    MirrorBackSlash,
    SplitterH,
    SplitterV,
}

impl TileType {
    fn new(c: char) -> Self {
        match c {
            '/' => Self::MirrorFwdSlash,
            '\\' => Self::MirrorBackSlash,
            '.' => Self::Empty,
            '|' => Self::SplitterV,
            '-' => Self::SplitterH,
            _ => panic!("Unknown tile '{c}'"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Tile {
    kind: TileType,
    beams: Vec<Direction>,
}

impl Tile {
    fn new(c: char) -> Self {
        let kind = TileType::new(c);
        let beams = Vec::new();
        Self { kind, beams }
    }
}

fn parse(contents: &str) -> Vec<Vec<Tile>> {
    let parse_line = |l: &str| l.chars().map(Tile::new).collect::<Vec<_>>();
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}

type CoOrd = (usize, usize);

fn get_next_tile(
    grid: &Vec<Vec<Tile>>,
    start: CoOrd,
    direction_of_travel: Direction,
) -> Option<CoOrd> {
    match direction_of_travel {
        Direction::North => {
            if start.0 > 0 {
                Some((start.0 - 1, start.1))
            } else {
                None
            }
        }
        Direction::East => {
            if start.1 < (grid[0].len() - 1) {
                Some((start.0, start.1 + 1))
            } else {
                None
            }
        }
        Direction::South => {
            if start.0 < (grid.len() - 1) {
                Some((start.0 + 1, start.1))
            } else {
                None
            }
        }
        Direction::West => {
            if start.1 > 0 {
                Some((start.0, start.1 - 1))
            } else {
                None
            }
        }
    }
}

fn trace_beam(grid: &mut Vec<Vec<Tile>>, start: CoOrd, mut direction_of_travel: Direction) {
    let mut current = start;
    'l1: loop {
        let current_tile = &mut grid[current.0][current.1];
        if current_tile.beams.contains(&direction_of_travel) {
            break 'l1; // don't enter an infinite loop
        }
        current_tile.beams.push(direction_of_travel);
        let mut new_direction: Option<Direction> = None;
        match current_tile.kind {
            TileType::Empty => new_direction = Some(direction_of_travel),
            TileType::MirrorFwdSlash => {
                new_direction = Some(match direction_of_travel {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                });
            }
            TileType::MirrorBackSlash => {
                new_direction = Some(match direction_of_travel {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                });
            }
            TileType::SplitterH => match direction_of_travel {
                Direction::North | Direction::South => {
                    let mut dir = Direction::East;
                    if let Some(next) = get_next_tile(grid, current, dir) {
                        trace_beam(grid, next, dir);
                    }
                    dir = Direction::West;
                    if let Some(next) = get_next_tile(grid, current, dir) {
                        trace_beam(grid, next, dir);
                    }
                }
                Direction::East | Direction::West => new_direction = Some(direction_of_travel),
            },
            TileType::SplitterV => match direction_of_travel {
                Direction::West | Direction::East => {
                    let mut dir = Direction::South;
                    if let Some(next) = get_next_tile(grid, current, dir) {
                        trace_beam(grid, next, dir);
                    }
                    dir = Direction::North;
                    if let Some(next) = get_next_tile(grid, current, dir) {
                        trace_beam(grid, next, dir);
                    }
                }
                Direction::North | Direction::South => new_direction = Some(direction_of_travel),
            },
        }
        if let Some(dir) = new_direction {
            direction_of_travel = dir;
            if let Some(next) = get_next_tile(grid, current, direction_of_travel) {
                current = next;
                continue 'l1;
            }
        }
        break 'l1;
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test16 {
    use super::*;

    #[test]
    fn GIVEN_small_grid_WHEN_constructing_THEN_expected_tiles_created() {
        let grid = [r"\/|-", "...."].join("\n");
        let rows = parse(grid.as_str());
        use TileType::*;
        assert_eq!(2, rows.len());
        assert_eq!(
            vec![MirrorBackSlash, MirrorFwdSlash, SplitterV, SplitterH],
            rows[0].iter().map(|t| t.kind).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![Empty; 4],
            rows[1].iter().map(|t| t.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn GIVEN_small_grids_WHEN_tracing_beam_THEN_beam_changes_direction_as_expected() {
        let dotest = |rows: &[&str], test_coord: (usize, usize), expected_dir| {
            let mut grid = parse(rows.join("\n").as_str());
            trace_beam(&mut grid, (0, 0), Direction::East);
            let tile = &grid[test_coord.0][test_coord.1];
            assert_eq!(1, tile.beams.len());
            assert_eq!(expected_dir, tile.beams[0]);
        };
        dotest(&[r".\", r".."], (1, 1), Direction::South); // mirror 1
        dotest(&[r"\.", r"\."], (1, 1), Direction::East); // mirror 2
        dotest(&[r".\.", r".-."], (1, 0), Direction::West); // horizontal splitter pt1
        dotest(&[r".\.", r".-."], (1, 2), Direction::East); // horizontal splitter pt2
        dotest(&[r"\.", r"\|", r".."], (0, 1), Direction::North); // vertical splitter pt1
        dotest(&[r"\.", r"\|", r".."], (2, 1), Direction::South); // vertical splitter pt2
    }

    static EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(46, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_example_input_WHEN_locating_max_THEN_tile_matches_example_given() {
        let rows = parse(EXAMPLE);
        let mut max_tile = (0, 0);
        let mut max = 0;
        for col_idx in 0..rows[0].len() {
            let num = count_engergized_tiles(rows.clone(), (0, col_idx), Direction::South);
            if num > max {
                max = num;
                max_tile = (0, col_idx);
            }
        }
        // should be 4th tile on top row..
        assert_eq!((0, 3), max_tile);
        assert_eq!(51, max);
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(51, part2(EXAMPLE));
    }
}
