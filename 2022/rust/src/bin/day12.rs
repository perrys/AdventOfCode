use std::{cell::Cell, fs, rc::Rc};

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

fn part1(_contents: &str) -> u32 {
    let (grid, start_tile) = parse(_contents);
    dijkstra(&grid, start_tile).expect("no path to end tile")
}

fn part2(_contents: &str) -> u32 {
    let (mut grid, _start_tile) = parse(_contents);
    let start_candidates = grid
        .tiles
        .iter()
        .filter_map(
            |tile| {
                if tile.get().height == 0 {
                    Some(tile.clone())
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();
    // there's probably a smarter way to do this where the start points are
    // connected to each other in groups. The following is a brute-force, from
    // scratch search from each start candidate, and takes a second or two for
    // the full puzzle input:
    start_candidates
        .into_iter()
        .filter_map(|start_tile| {
            grid.reset(&start_tile);
            dijkstra(&grid, start_tile)
        })
        .min()
        .expect("unable to find minimum path")
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
enum TileType {
    Start,
    End,
    Other,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Tile {
    height: u8,
    kind: TileType,
    distance: u32,
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
            _ => u32::MAX,
        };
        Self {
            height: height as u8 - b'a',
            kind,
            distance,
            location,
        }
    }
}

type SharedTile = Rc<Cell<Tile>>;

struct Grid {
    tiles: Vec<SharedTile>,
    n_rows: u32,
}

impl Grid {
    fn new(tiles: Vec<SharedTile>, n_rows: u32) -> Self {
        Self { tiles, n_rows }
    }

    fn neighbors(&self, tile: SharedTile) -> Vec<SharedTile> {
        let n_cols = self.tiles.len() as u32 / self.n_rows;
        let mut result = Vec::new();
        let loc = tile.get().location;
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
            .filter(|neighbor| (neighbor.get().height as i16 - tile.get().height as i16) <= 1)
            .collect()
    }

    fn at(&self, loc: &Coord) -> Option<SharedTile> {
        if loc.row >= self.n_rows {
            return None;
        }
        let n_cols = self.tiles.len() as u32 / self.n_rows;
        let idx = loc.row * n_cols + loc.col;
        self.tiles.get(idx as usize).cloned()
    }

    fn reset(&mut self, start_tile: &SharedTile) {
        for tile in self.tiles.iter() {
            let mut tile_copy = tile.get();
            match tile_copy.kind {
                TileType::End => (),
                _ if tile_copy.location == start_tile.get().location => tile_copy.distance = 0,
                _ => tile_copy.distance = u32::MAX,
            }
            tile.set(tile_copy);
        }
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
                    let location = Coord::new(row_idx as u32, col_idx as u32);
                    let tile = Rc::new(Cell::new(Tile::new(tile_chr, location)));
                    if let TileType::Start = tile.get().kind {
                        start_tile = Some(tile.clone());
                    }
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if let Some(start_tile) = start_tile {
        let n_rows = grid.len() as u32;
        let grid = Grid::new(grid.into_iter().flatten().collect(), n_rows);
        return (grid, start_tile);
    }
    panic!("No start tile found");
}

fn dijkstra(grid: &Grid, start_tile: SharedTile) -> Option<u32> {
    let mut unvisited_list = Vec::<SharedTile>::new();
    unvisited_list.push(start_tile);
    let mut end: Option<SharedTile> = None;
    while !unvisited_list.is_empty() {
        // sort in reverse order:
        unvisited_list.sort_by(|lhs, rhs| rhs.get().distance.cmp(&lhs.get().distance));
        let current = unvisited_list.pop();
        if let Some(current) = current {
            let new_distance = current.get().distance + 1;
            for neighbor in grid.neighbors(current) {
                let mut n_copy = neighbor.get();
                let old_distance = n_copy.distance;
                n_copy.distance = old_distance.min(new_distance);
                neighbor.set(n_copy);
                if let TileType::End = neighbor.get().kind {
                    end = Some(neighbor.clone());
                } else if old_distance == u32::MAX {
                    unvisited_list.push(neighbor.clone());
                }
            }
        }
    }
    end.map(|end_tile| end_tile.get().distance)
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
    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(29, part2(EXAMPE));
    }
}
