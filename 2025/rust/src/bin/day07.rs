use std::collections::HashMap;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
    println!("part2: {}", part2(&contents));
}

fn make_grid(contents: &str) -> Vec<Vec<char>> {
    let grid = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Vec::from_iter(line.chars()))
        .collect::<Vec<_>>();
    let ncols = grid[0].len();
    assert!(grid.iter().all(|line| line.len() == ncols));
    grid
}

fn part1(contents: &str) -> usize {
    let mut grid = make_grid(contents);
    let ncols = grid[0].len();

    let mut count = 0;
    for row_idx in 1..grid.len() {
        for col_idx in 0..ncols {
            let has_parent = match grid[row_idx - 1][col_idx] {
                '|' | 'S' => true,
                _ => false,
            };
            match grid[row_idx][col_idx] {
                '.' if has_parent => grid[row_idx][col_idx] = '|',
                '^' if has_parent => {
                    let mut d = &mut grid[row_idx][col_idx - 1];
                    if col_idx > 0 {
                        *d = '|';
                    }
                    d = &mut grid[row_idx][col_idx + 1];
                    if col_idx < ncols - 1 {
                        *d = '|';
                    }
                    count += 1;
                }
                _ => (),
            }
        }
    }
    for line in grid {
        println!("{}", String::from_iter(line.iter()));
    }

    count
}

type Point = (usize, usize);
type Cache = HashMap<Point, usize>;

fn part2(contents: &str) -> usize {
    let mut grid = make_grid(contents);
    let mut cache = Cache::new();
    count_paths(&mut grid, &mut cache, 1)
}

fn count_paths(grid: &mut Vec<Vec<char>>, cache: &mut Cache, row_idx: usize) -> usize {
    if row_idx == grid.len() {
        return 1;
    }
    let mut result = 0;
    let ncols = grid[0].len();
    for col_idx in 0..ncols {
        let mut paths_below = 0;
        let incoming_beam = match grid[row_idx - 1][col_idx] {
            '|' | 'S' => true,
            _ => false,
        };
        match grid[row_idx][col_idx] {
            '.' if incoming_beam => {
                // the splitter rows are is interspersed with blank rows,
                // so we can just memoize the straight beam locations:
                let key = (row_idx, col_idx);
                if let Some(entry) = cache.get(&key) {
                    paths_below = *entry;
                } else {
                    let copy = grid[row_idx][col_idx];
                    grid[row_idx][col_idx] = '|';
                    paths_below += count_paths(grid, cache, row_idx + 1);
                    grid[row_idx][col_idx] = copy;
                    cache.insert(key, paths_below);
                }
            }
            '^' if incoming_beam => {
                if col_idx > 0 {
                    let idx = col_idx - 1;
                    let copy = grid[row_idx][idx];
                    grid[row_idx][idx] = '|';
                    paths_below += count_paths(grid, cache, row_idx + 1);
                    grid[row_idx][idx] = copy;
                }
                if col_idx < ncols - 1 {
                    let idx = col_idx + 1;
                    let copy = grid[row_idx][idx];
                    grid[row_idx][idx] = '|';
                    paths_below += count_paths(grid, cache, row_idx + 1);
                    grid[row_idx][idx] = copy;
                }
            }
            _ => (),
        }
        result += paths_below;
    }
    result
}
