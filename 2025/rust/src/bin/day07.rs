fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
}

fn part1(contents: &str) -> usize {
    let mut grid = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Vec::from_iter(line.chars()))
        .collect::<Vec<_>>();
    let ncols = grid[0].len();
    assert!(grid.iter().all(|line| line.len() == ncols));

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
