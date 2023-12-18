//!
//! Advent of code challenge 2023 day 17.
//!
//! See <https://adventofcode.com/2023/day/17>
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
    let rows = parse(contents);
    let path = dijkstra_optimal_path(&rows);
    path.cost
}

fn part2(_contents: &str) -> usize {
    0
}

type Grid = Vec<Vec<usize>>;

fn parse(contents: &str) -> Grid {
    let parse_line = |l: &str| {
        l.chars()
            .map(|c| {
                if !c.is_ascii_digit() {
                    panic!("not a number: {c}");
                }
                let num = c as u8 - b'0';
                num as usize
            })
            .collect::<Vec<_>>()
    };
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(usize)]
enum Direction {
    North,
    South,
    East,
    West,
}

type CoOrd = (usize, usize);

fn get_next_block(rows: &Grid, (row, col): CoOrd, direction_of_travel: Direction) -> Option<CoOrd> {
    let nrows = rows.len();
    let ncols = rows[0].len();
    match direction_of_travel {
        Direction::North => match row {
            _ if row > 0 => Some((row - 1, col)),
            _ => None,
        },
        Direction::East => match row {
            _ if col < (ncols - 1) => Some((row, col + 1)),
            _ => None,
        },
        Direction::South => match row {
            _ if row < (nrows - 1) => Some((row + 1, col)),
            _ => None,
        },
        Direction::West => match row {
            _ if col > 0 => Some((row, col - 1)),
            _ => None,
        },
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathElement {
    position: CoOrd,
    previous: Option<Box<PathElement>>,
    directions: [Direction; 2],
    cost: usize,
}

impl PathElement {
    fn new(
        cost: usize,
        position: CoOrd,
        directions: [Direction; 2],
        previous: Option<Box<PathElement>>,
    ) -> Self {
        Self {
            cost,
            position,
            directions,
            previous,
        }
    }
}

impl Ord for PathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // lower is better
    }
}

impl PartialOrd for PathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

static MAX_STEPS_PER_DIRECTION: usize = 3;
static NORTHSOUTH: [Direction; 2] = [Direction::North, Direction::South];
static EASTWEST: [Direction; 2] = [Direction::East, Direction::West];

fn dijkstra_optimal_path(rows: &Grid) -> PathElement {
    type DistanceQueue = BinaryHeap<PathElement>;
    type CostMap = HashMap<(CoOrd, [Direction; 2]), usize>; // 2 separate costs per tile due to direction rules

    let mut queue = DistanceQueue::new();
    let mut cost_map = CostMap::new();

    let target_block = (rows.len() - 1, rows[0].len() - 1);

    queue.push(PathElement::new(0, (0, 0), NORTHSOUTH, None));
    queue.push(PathElement::new(0, (0, 0), EASTWEST, None));

    loop {
        let current_cheapest = queue
            .pop()
            .unwrap_or_else(|| panic!("no path to target block"));
        if current_cheapest.position == target_block {
            break current_cheapest;
        }
        let other_dir =
            if current_cheapest.directions == NORTHSOUTH { EASTWEST } else { NORTHSOUTH };
        'ldir: for dir in current_cheapest.directions.iter() {
            let mut steps_cost = current_cheapest.cost;
            let mut last_step = current_cheapest.position;
            for _steps in 1..=MAX_STEPS_PER_DIRECTION {
                if let Some(new_position @ (row, col)) = get_next_block(rows, last_step, *dir) {
                    last_step = new_position;
                    steps_cost += rows[row][col];
                    let key = (last_step, current_cheapest.directions);
                    let provisional_cost = *cost_map.get(&key).unwrap_or(&usize::MAX);
                    if steps_cost < provisional_cost {
                        cost_map.insert(key, steps_cost);
                        queue.push(PathElement::new(
                            steps_cost,
                            last_step,
                            other_dir,
                            Some(Box::new(current_cheapest.clone())),
                        ));
                    }
                } else {
                    continue 'ldir;
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test17 {
    use super::*;

    fn get_path(last_elt: &PathElement) -> Vec<CoOrd> {
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
        let dotest = |grid: &[&str], expected: &[CoOrd]| {
            let rows = parse(grid.join("\n").as_str());
            let last_elt = dijkstra_optimal_path(&rows);
            assert_eq!(expected, get_path(&last_elt));
        };
        dotest(&[r"02", "30"], &[(0, 0), (0, 1), (1, 1)]); // 2x2 grid
        dotest(&[r"123", "450"], &[(0, 0), (0, 2), (1, 2)]); // 2x3 grid
        dotest(&[r"123", "719", "650"], &[(0, 0), (0, 1), (2, 1), (2, 2)]); // 3x3 grid
        dotest(
            &["1323", "5623", "4254", "5452"],
            &[(0, 0), (0, 2), (1, 2), (1, 3), (3, 3)],
        );
    }

    #[test]
    fn GIVEN_small_grid_WHEN_applying_dijkstra_THEN_correct_path_cost_returned() {
        let dotest = |grid: &[&str], expected| {
            let rows = parse(grid.join("\n").as_str());
            let last_elt = dijkstra_optimal_path(&rows);
            assert_eq!(expected, last_elt.cost);
        };
        dotest(&[r"02", "30"], 2); // 2x2 grid
        dotest(&[r"123", "456"], 11); // 2x3 grid
        dotest(&[r"123", "719", "650"], 8); // 3x3 grid
        dotest(&["1323", "5623", "4254", "5452"], 16);
    }

    static EXAMPLE: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        let rows = parse(EXAMPLE);
        let last_elt = dijkstra_optimal_path(&rows);
        assert_eq!(102, last_elt.cost);
    }
}
