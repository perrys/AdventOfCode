use std::{env, fs};

fn get(grid: &[Vec<char>], x: usize, y: usize, dx: i32, dy: i32) -> Option<char> {
    let xp = x as i32 + dx;
    let yp = y as i32 + dy;
    if xp < 0 || yp < 0 || xp as usize >= grid[0].len() || yp as usize >= grid.len() {
        return None;
    }
    Some(grid[yp as usize][xp as usize])
}

const DIRS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn part1(grid: &[Vec<char>]) -> Vec<[usize; 2]> {
    let mut result = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point != '@' {
                // print!("{}", *point);
                continue;
            }
            let mut count = 0;
            for [dx, dy] in DIRS {
                if get(grid, x, y, dx, dy) == Some('@') {
                    count += 1;
                }
            }
            if count < 4 {
                result.push([x, y]);
                // print!("{}", 'x');
            } else {
                // print!("{}", *point);
            }
        }
        // println!("");
    }
    result
}

fn make_grid(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .filter_map(|l| {
            let line = l.trim();
            if line.is_empty() {
                return None;
            }
            Some(line.chars().collect::<Vec<_>>())
        })
        .collect()
}

fn part2(grid: &mut [Vec<char>]) -> usize {
    let mut p2_score = 0;
    loop {
        let positions = part1(grid);
        let nremoved = positions.len();
        if nremoved == 0 {
            break;
        }
        p2_score += nremoved;
        for [x, y] in positions {
            grid[y][x] = '.';
        }
    }
    p2_score
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("invalid filename");
    let mut grid = make_grid(&contents);
    println!("part1: {}", part1(&grid).len());
    println!("part2: {}", part2(&mut grid));
}

#[cfg(test)]
mod tester {
    use super::*;

    const TEST_DATA: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn part1_test() {
        let grid = make_grid(TEST_DATA);
        assert_eq!(part1(&grid).len(), 13);
    }

    #[test]
    fn part2_test() {
        let mut grid = make_grid(TEST_DATA);
        assert_eq!(part2(&mut grid), 43);
    }
}
