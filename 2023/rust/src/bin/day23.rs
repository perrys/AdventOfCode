//!
//! Advent of code challenge 2023 day 23.
//!
//! See <https://adventofcode.com/2023/day/23>
//!
use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
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
    let grid = Grid::new(contents);
    let start = (0, 1);
    let target = (grid.nrows - 1, grid.ncols - 2);
    let neighbor_fn = |g: &Grid, tile| get_valid_neighbours(g, tile, SlopeTreatment::Normal);
    dijkstra_longest_path(&grid, start, target, neighbor_fn)
        .expect("no paths to target")
        .cost
}

fn part2(contents: &str) -> usize {
    let grid = Grid::new(contents);
    let start = (0, 1);
    let target = (grid.nrows - 1, grid.ncols - 2);
    let possible_paths = paths_between_intersections(&grid, start, target);
    breadth_first_search(&possible_paths, start, target)
        .expect("no paths to target")
        .cost
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SlopeTreatment {
    Normal,
    Path,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    nrows: usize,
    ncols: usize,
}

impl Grid {
    fn new(contents: &str) -> Self {
        let tiles = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_line)
            .collect::<Vec<_>>();
        let nrows = tiles.len();
        let ncols = tiles[0].len();
        Self {
            tiles,
            nrows,
            ncols,
        }
    }
}

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
        // this has horrible cache performance and really slows down the BFS in
        // part2. It would certainly be much faster if these were stored in a
        // Vec.
        let mut elt = &self.previous;
        while let Some(e) = elt {
            if e.position == coord {
                return true;
            }
            elt = &e.previous;
        }
        false
    }
    #[allow(dead_code)] // for debugging
    fn to_string(&self) -> String {
        let mut result = format!("{:?}, {} ", self.position, self.cost).to_owned();
        if self.previous.is_none() {
            return result;
        }
        let mut prev = self.previous.as_ref().unwrap();
        result = format!("{:?}, {} ", prev.position, prev.cost).to_owned() + &result;
        while let Some(p) = &prev.previous {
            prev = p;
            result = format!("{:?}, {} ", prev.position, prev.cost).to_owned() + &result;
        }
        result
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

/**
 * Find points on the graph where there is a choice of direction. This is where
 * the point is surrounded by forest on less than two sides.
 */
fn find_junction_points(grid: &Grid) -> Vec<RowAndCol> {
    (1..grid.nrows - 1)
        .flat_map(|row_idx| {
            (1..grid.ncols - 1).filter_map(move |col_idx| match grid.tiles[row_idx][col_idx] {
                Tile::Forest => None,
                _ => {
                    let n_forests = [(1, 0), (-1isize, 0), (0, 1), (0, -1isize)]
                        .iter()
                        .filter(|(delta_r, delta_c)| {
                            let (r, c): RowAndCol = (
                                (row_idx as isize + delta_r) as usize,
                                (col_idx as isize + delta_c) as usize,
                            );
                            matches!(grid.tiles[r][c], Tile::Forest)
                        })
                        .count();
                    if n_forests < 2 {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                }
            })
        })
        .collect::<Vec<_>>()
}

fn paths_between_intersections(
    grid: &Grid,
    start: (usize, usize),
    target: (usize, usize),
) -> Vec<(RowAndCol, RowAndCol, usize)> {
    let mut junctions = find_junction_points(grid);
    junctions.extend_from_slice(&[start, target]);
    let possible_paths = junctions
        .iter()
        .enumerate()
        .flat_map(|(idx, j1)| {
            let slice = &junctions[(idx + 1)..];
            slice.iter().map(|j2| (*j1, *j2))
        })
        .collect::<Vec<_>>();
    let mut paths = Vec::new();
    for (j1, j2) in possible_paths.iter() {
        let valid_neighbors_between_junctions_ignore_slopes = |g: &Grid, tile: RowAndCol| {
            if tile == *j1 || tile == *j2 {
                get_valid_neighbours(g, tile, SlopeTreatment::Path)
            } else if junctions.contains(&tile) {
                Vec::<RowAndCol>::new() // prevents passing through another junction
            } else {
                get_valid_neighbours(g, tile, SlopeTreatment::Path)
            }
        };
        if let Some(path) = dijkstra_longest_path(
            grid,
            *j1,
            *j2,
            valid_neighbors_between_junctions_ignore_slopes,
        ) {
            paths.push((*j1, *j2, path.cost));
        }
    }
    paths
}

fn dijkstra_longest_path<F>(
    grid: &Grid,
    start: RowAndCol,
    target: RowAndCol,
    get_neighbours: F,
) -> Option<PathElement>
where
    F: Fn(&Grid, RowAndCol) -> Vec<RowAndCol>,
{
    type DistanceQueue = BinaryHeap<PathElement>;
    type CostMap = HashMap<RowAndCol, usize>;

    let mut queue = DistanceQueue::new();
    let mut cost_map = CostMap::new();

    queue.push(PathElement {
        position: start,
        cost: 0,
        previous: None,
    });

    let mut candidate_paths = Vec::new();
    'l0: loop {
        let current = queue.pop();
        let current = match current {
            Some(elt) => elt,
            None if candidate_paths.is_empty() => return None,
            None => break 'l0,
        };
        if current.position == target {
            candidate_paths.push(current);
            continue 'l0;
        }
        for new_position in get_neighbours(grid, current.position) {
            let mut steps_cost = current.cost;
            if current.is_previous_step(new_position) {
                continue;
            }
            steps_cost += 1;
            let provisional_cost = *cost_map.get(&new_position).unwrap_or(&usize::MIN);
            if steps_cost > provisional_cost {
                cost_map.insert(new_position, steps_cost);
                queue.push(PathElement {
                    previous: Some(Box::new(current.clone())),
                    position: new_position,
                    cost: steps_cost,
                });
            }
        }
    }
    candidate_paths.sort_by_key(|path| path.cost);
    candidate_paths.pop()
}

fn breadth_first_search(
    edges: &[(RowAndCol, RowAndCol, usize)],
    start: RowAndCol,
    target: RowAndCol,
) -> Option<PathElement> {
    type Queue = VecDeque<PathElement>;
    let mut queue = Queue::new();
    queue.push_back(PathElement {
        position: start,
        cost: 0,
        previous: None,
    });

    let mut candidate_paths = Vec::new();
    'l0: while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.position == target {
            candidate_paths.push(current);
            continue 'l0;
        }
        let valid_edges = edges
            .iter()
            .filter(|edge| edge.0 == current.position || edge.1 == current.position);
        'l1: for &(j1, j2, cost) in valid_edges {
            let new_position = if j1 == current.position { j2 } else { j1 };
            let mut steps_cost = current.cost;
            if current.is_previous_step(new_position) {
                continue 'l1;
            }
            steps_cost += cost;
            queue.push_back(PathElement {
                previous: Some(Box::new(current.clone())),
                position: new_position,
                cost: steps_cost,
            });
        }
    }
    candidate_paths.sort_by_key(|path| path.cost);
    candidate_paths.pop()
}

fn get_valid_neighbours(grid: &Grid, tile: RowAndCol, st: SlopeTreatment) -> Vec<RowAndCol> {
    use Direction::*;
    let max_col = grid.ncols - 1;
    let max_row = grid.nrows - 1;
    let directions = [North, South, East, West];
    directions
        .iter()
        .filter_map(|dir_of_travel| {
            let (row_idx, col_idx) = match dir_of_travel {
                North if tile.0 > 0 => (tile.0 - 1, tile.1),
                South if tile.0 < max_col => (tile.0 + 1, tile.1),
                East if tile.1 < max_row => (tile.0, tile.1 + 1),
                West if tile.1 > 0 => (tile.0, tile.1 - 1),
                _ => return None,
            };
            match grid.tiles[row_idx][col_idx] {
                Tile::Path => Some((row_idx, col_idx)),
                Tile::Slope(_) if matches!(st, SlopeTreatment::Path) => Some((row_idx, col_idx)),
                Tile::Forest => None,
                Tile::Slope(direction) => {
                    if *dir_of_travel == direction {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                }
            }
        })
        .collect::<Vec<_>>()
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
            let grid = Grid::new(grid.join("\n").as_str());
            let last_elt = dijkstra_longest_path(
                &grid,
                (0, 1),
                (grid.nrows - 1, grid.ncols - 2),
                |grid, tile| get_valid_neighbours(grid, tile, SlopeTreatment::Normal),
            )
            .expect("no path to target");
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
    fn GIVEN_example_grid_WHEN_finding_intersections_THEN_correctly_identified() {
        let grid = Grid::new(EXAMMPLE);
        let expected = vec![
            (3, 11),
            (5, 3),
            (11, 21),
            (13, 5),
            (13, 13),
            (19, 13),
            (19, 19),
        ];
        assert_eq!(expected, find_junction_points(&grid));
    }

    #[test]
    fn GIVEN_example_grid_WHEN_routing_between_intersections_THEN_valid_routes_found() {
        let grid = Grid::new(EXAMMPLE);
        let possible_paths =
            paths_between_intersections(&grid, (0, 1), (grid.nrows - 1, grid.ncols - 2));
        let valid_paths = vec![
            ((3, 11), (5, 3)),
            ((3, 11), (11, 21)),
            ((3, 11), (13, 13)),
            ((5, 3), (13, 5)),
            ((5, 3), (0, 1)),
            ((11, 21), (13, 13)),
            ((11, 21), (19, 19)),
            ((13, 5), (13, 13)),
            ((13, 5), (19, 13)),
            ((13, 13), (19, 13)),
            ((19, 13), (19, 19)),
            ((19, 19), (22, 21)),
        ];
        assert_eq!(
            valid_paths,
            possible_paths
                .into_iter()
                .map(|(j1, j2, _cost)| (j1, j2))
                .collect::<Vec<_>>()
        );
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
