//!
//! Advent of code challenge 2023 day 23.
//!
//! See <https://adventofcode.com/2023/day/23>
//!
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

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
    find_optimal_path(contents, false).cost
}

fn part2(contents: &str) -> usize {
    find_optimal_path(contents, true).cost
}

fn find_optimal_path(contents: &str, ignore_slopes: bool) -> PathElement {
    let grid = parse(contents);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let start = (0, 1);
    let target = (nrows - 1, ncols - 2);
    dijkstra_longest_path(&grid, start, target, ignore_slopes)
}

fn parse(contents: &str) -> Vec<Vec<Tile>> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

type Grid = Vec<Vec<Tile>>;

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars()
        .map(|c| match c {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '>' => Tile::Slope(Direction::East),
            '<' => Tile::Slope(Direction::West),
            '^' => Tile::Slope(Direction::North),
            'v' => Tile::Slope(Direction::South),
            _ => panic!("unknown tile {c} in \"{line}\""),
        })
        .collect()
}

type RowAndCol = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathElement {
    position: RowAndCol,
    previous: Option<Box<PathElement>>,
    cost: usize,
}

impl PathElement {
    fn is_previous_step(&self, coord: RowAndCol) -> bool {
        let mut elt = &self.previous;
        while let Some(e) = elt {
            if e.position == coord {
                return true;
            }
            elt = &e.previous;
        }
        false
    }
}

impl Ord for PathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost) // higher is better
    }
}

impl PartialOrd for PathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_longest_path(
    rows: &Grid,
    start: RowAndCol,
    target: RowAndCol,
    ignore_slopes: bool,
) -> PathElement {
    use Direction::*;
    let max_col = rows.len() - 1;
    let max_row = rows[0].len() - 1;
    let get_next_block = |last_step: RowAndCol, dir_of_travel, ignore_slopes| {
        let (row_idx, col_idx) = match dir_of_travel {
            North if last_step.0 > 0 => (last_step.0 - 1, last_step.1),
            South if last_step.0 < max_col => (last_step.0 + 1, last_step.1),
            East if last_step.1 < max_row => (last_step.0, last_step.1 + 1),
            West if last_step.1 > 0 => (last_step.0, last_step.1 - 1),
            _ => return None,
        };
        match rows[row_idx][col_idx] {
            Tile::Path => Some((row_idx, col_idx)),
            Tile::Slope(_) if ignore_slopes => Some((row_idx, col_idx)),
            Tile::Forest => None,
            Tile::Slope(direction) => {
                if dir_of_travel == direction {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            }
        }
    };

    type DistanceQueue = BinaryHeap<PathElement>;
    type CostMap = HashMap<RowAndCol, usize>;

    let mut queue = DistanceQueue::new();
    let mut cost_map = CostMap::new();

    queue.push(PathElement {
        position: start,
        cost: 0,
        previous: None,
    });
    let directions = [North, South, East, West];

    let mut candidate_paths = Vec::new();
    'l0: loop {
        let current = queue.pop();
        let current = match current {
            Some(elt) => elt,
            None if candidate_paths.is_empty() => panic!("no path to target block"),
            None => break 'l0,
        };
        if current.position == target {
            candidate_paths.push(current);
            continue 'l0;
        }
        'ldir: for dir in directions.iter() {
            let mut steps_cost = current.cost;
            let mut last_step = current.position;
            if let Some(new_position) = get_next_block(last_step, *dir, ignore_slopes) {
                if current.is_previous_step(new_position) {
                    continue 'ldir;
                }
                last_step = new_position;
                steps_cost += 1;
                let provisional_cost = *cost_map.get(&last_step).unwrap_or(&usize::MIN);
                if steps_cost > provisional_cost {
                    cost_map.insert(last_step, steps_cost);
                    queue.push(PathElement {
                        previous: Some(Box::new(current.clone())),
                        position: new_position,
                        cost: steps_cost,
                    });
                }
            } else {
                continue 'ldir;
            }
        }
    }
    candidate_paths.sort_by_key(|path| path.cost);
    candidate_paths.pop().unwrap()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test23 {
    use super::*;

    fn get_path(last_elt: &PathElement) -> Vec<RowAndCol> {
        let mut elt = last_elt;
        let mut elts = Vec::new();
        elts.push(elt.position);
        while let Some(e) = &elt.previous {
            elts.push(e.position);
            elt = e;
        }
        elts.into_iter().rev().collect::<Vec<_>>()
    }

    #[test]
    fn GIVEN_small_grid_WHEN_applying_dijkstra_THEN_optimal_paths_returned() {
        let dotest = |grid: &[&str], expected: &[RowAndCol]| {
            let rows = parse(grid.join("\n").as_str());
            let last_elt =
                dijkstra_longest_path(&rows, (0, 1), (grid.len() - 1, grid[0].len() - 2), false);
            assert_eq!(expected, get_path(&last_elt));
        };
        dotest(&[r"#.", ".."], &[(0, 1), (1, 1), (1, 0)]); // 2x2 grid
        dotest(
            &[r"#.#", "...", "..#"],
            &[(0, 1), (1, 1), (1, 0), (2, 0), (2, 1)],
        ); // 3x3 grid
    }
    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(94, part1(EXAMMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(154, part2(EXAMMPLE));
    }

    static EXAMMPLE: &str = r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;
}
