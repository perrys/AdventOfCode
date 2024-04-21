use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    println!("Part 1 answer is {}", part1(contents.as_str()));
    println!("Part 2 answer is {}", part2(contents.as_str()));
}

fn part1(_contents: &str) -> usize {
    let (grid, start_tile) = parse(_contents);
    dijkstra(grid, start_tile)
}

fn part2(_contents: &str) -> usize {
    0
}

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Start,
    End,
    Other,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tile {
    height: u8,
    kind: TileType,
    distance: usize,
    location: Coord,
}

impl Tile {
    fn new(c: char, location: Coord) -> Self {
        let (height, kind) = match c {
            'S' => ('a', TileType::Start),
            'E' => ('z', TileType::End),
            _ => (c, TileType::Other),
        };
        let distance = match kind {
            TileType::Start => 0,
            _ => usize::MAX,
        };
        Self {
            height: height as u8 - b'a',
            kind,
            distance,
            location,
        }
    }
}

type SharedTile = Rc<RefCell<Tile>>;

struct Grid {
    tiles: Vec<SharedTile>,
    n_rows: usize,
}

impl Grid {
    fn new(tiles: Vec<SharedTile>, n_rows: usize) -> Self {
        Self { tiles, n_rows }
    }

    fn neighbors(&self, tile: SharedTile) -> Vec<SharedTile> {
        let n_cols = self.tiles.len() / self.n_rows;
        let mut result = Vec::new();
        let loc = tile.borrow().location;
        if loc.row > 0 {
            if let Some(stile) = self.at(&Coord::new(loc.row - 1, loc.col)) {
                result.push(stile);
            }
        }
        if loc.row < self.n_rows - 1 {
            if let Some(stile) = self.at(&Coord::new(loc.row + 1, loc.col)) {
                result.push(stile);
            }
        }
        if loc.col > 0 {
            if let Some(stile) = self.at(&Coord::new(loc.row, loc.col - 1)) {
                result.push(stile);
            }
        }
        if loc.col < n_cols - 1 {
            if let Some(stile) = self.at(&Coord::new(loc.row, loc.col + 1)) {
                result.push(stile);
            }
        }
        result
            .into_iter()
            .filter(|neighbor| (neighbor.borrow().height as i16 - tile.borrow().height as i16) <= 1)
            .collect()
    }
    fn at(&self, loc: &Coord) -> Option<SharedTile> {
        if loc.row >= self.n_rows {
            return None;
        }
        let n_cols = self.tiles.len() / self.n_rows;
        let idx = loc.row * n_cols + loc.col;
        self.tiles.get(idx).cloned()
    }
}

fn parse(contents: &str) -> (Grid, SharedTile) {
    let mut start_tile: Option<SharedTile> = None;
    let grid = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, tile_chr)| {
                    let location = Coord::new(row_idx, col_idx);
                    let tile = Rc::new(RefCell::new(Tile::new(tile_chr, location)));
                    if let TileType::Start = tile.borrow().kind {
                        start_tile = Some(tile.clone());
                    }
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if let Some(start_tile) = start_tile {
        let n_rows = grid.len();
        let grid = Grid::new(grid.into_iter().flatten().collect(), n_rows);
        return (grid, start_tile);
    }
    panic!("No start tile found");
}

fn dijkstra(grid: Grid, start_tile: SharedTile) -> usize {
    let mut unvisited_list = Vec::<SharedTile>::new();
    unvisited_list.push(start_tile);
    let mut end: Option<SharedTile> = None;
    while !unvisited_list.is_empty() {
        // sort in reverse order:
        unvisited_list.sort_by(|lhs, rhs| rhs.borrow().distance.cmp(&lhs.borrow().distance));
        let current = unvisited_list.pop();
        if let Some(current) = current {
            let new_distance = current.borrow().distance + 1;
            for neighbor in grid.neighbors(current) {
                let old_distance = neighbor.borrow().distance;
                neighbor.borrow_mut().distance = old_distance.min(new_distance);
                if let TileType::End = neighbor.borrow().kind {
                    end = Some(neighbor.clone());
                } else if old_distance == usize::MAX {
                    unvisited_list.push(neighbor.clone());
                }
            }
        }
    }
    if let Some(end) = end {
        end.borrow().distance
    } else {
        panic!("No path to end tile found");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tester {
    use super::*;

    static EXAMPE: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(31, part1(EXAMPE));
    }
}
