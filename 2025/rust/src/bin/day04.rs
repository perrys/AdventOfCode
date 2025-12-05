use std::{env, fs};

fn get(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> Option<char> {
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

fn part1(contents: &str) -> usize {
    let grid: Vec<_> = contents
        .lines()
        .filter_map(|l| {
            let line = l.trim();
            if line.is_empty() {
                return None;
            }
            Some(line.chars().collect::<Vec<_>>())
        })
        .collect();
    let mut p1score = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point != '@' {
                // print!("{}", *point);
                continue;
            }
            let mut count = 0;
            for [dx, dy] in DIRS {
                if get(&grid, x, y, dx, dy) == Some('@') {
                    count += 1;
                }
            }
            if count < 4 {
                p1score += 1;
                // print!("{}", 'x');
            } else {
                // print!("{}", *point);
            }
        }
        println!("");
    }
    p1score
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
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
        assert_eq!(part1(TEST_DATA), 13);
    }
}
