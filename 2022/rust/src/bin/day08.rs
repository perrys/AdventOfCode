use std::fs;

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

fn part1(contents: &str) -> usize {
    let mut forest = parse(contents);
    for row_idx in 0..forest.n_rows {
        walk_line(forest.row_mut_iter(row_idx), |tree: &mut Tree| {
            tree.vis_east = true
        });
        walk_line(forest.row_mut_iter(row_idx).rev(), |tree: &mut Tree| {
            tree.vis_west = true
        });
    }
    for col_idx in 0..forest.n_cols {
        walk_line(forest.col_mut_iter(col_idx), |tree: &mut Tree| {
            tree.vis_north = true
        });
        walk_line(forest.col_mut_iter(col_idx).rev(), |tree: &mut Tree| {
            tree.vis_south = true
        });
    }
    forest
        .grid
        .into_iter()
        .filter(|tree| tree.visible())
        .count()
}

fn part2(contents: &str) -> usize {
    let forest = parse(contents);
    let mut max_score = 0;
    for row_idx in 0..forest.n_rows {
        for col_idx in 0..forest.n_cols {
            let mut scenic_score = 1;
            let tree_height = forest.tree_at(row_idx, col_idx).height;
            scenic_score *= forest.count_trees(tree_height, row_idx, col_idx, Direction::North);
            scenic_score *= forest.count_trees(tree_height, row_idx, col_idx, Direction::South);
            scenic_score *= forest.count_trees(tree_height, row_idx, col_idx, Direction::East);
            scenic_score *= forest.count_trees(tree_height, row_idx, col_idx, Direction::West);
            max_score = max_score.max(scenic_score);
        }
    }
    max_score
}

fn walk_line<'a, IterMut, Setter>(forest_line: IterMut, cb: Setter)
where
    IterMut: Iterator<Item = &'a mut Tree>,
    Setter: Fn(&'a mut Tree),
{
    let mut highest: Option<u8> = None;
    'l1: for tree in forest_line {
        let tree_height = tree.height;
        if let Some(h) = highest {
            if tree_height <= h {
                continue 'l1;
            }
        }
        highest = Some(tree_height);
        cb(tree);
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Forest {
    grid: Vec<Tree>,
    n_rows: usize,
    n_cols: usize,
}

impl Forest {
    fn new(grid: Vec<Tree>, n_cols: usize) -> Self {
        let n_rows = grid.len() / n_cols;
        if n_rows * n_cols != grid.len() {
            panic!("not a square grid");
        }
        Self {
            grid,
            n_rows,
            n_cols,
        }
    }
    fn tree_at(&self, row_idx: usize, col_idx: usize) -> &Tree {
        self.grid.get(row_idx * self.n_cols + col_idx).unwrap()
    }

    fn row_mut_iter(&mut self, row_idx: usize) -> impl DoubleEndedIterator<Item = &mut Tree> {
        let start = row_idx * self.n_cols;
        self.grid[start..start + self.n_cols].iter_mut()
    }
    fn col_mut_iter(&mut self, col_idx: usize) -> impl DoubleEndedIterator<Item = &mut Tree> {
        let start = col_idx;
        self.grid[start..].iter_mut().step_by(self.n_cols)
    }
    fn direction_iter(
        &self,
        row_idx: usize,
        col_idx: usize,
        dir: Direction,
    ) -> Box<dyn Iterator<Item = &Tree> + '_> {
        let fwd_skip = row_idx * self.n_cols + col_idx;
        match dir {
            Direction::North => {
                let back_skip = self.grid.len() - fwd_skip - 1;
                Box::new(self.grid.iter().rev().skip(back_skip).step_by(self.n_cols))
            }
            Direction::South => Box::new(self.grid.iter().skip(fwd_skip).step_by(self.n_cols)),
            Direction::East => {
                let remain = self.n_cols - col_idx;
                Box::new(self.grid[fwd_skip..(fwd_skip + remain)].iter())
            }
            Direction::West => {
                Box::new(self.grid[(fwd_skip - col_idx)..(fwd_skip + 1)].iter().rev())
            }
        }
    }
    fn count_trees(&self, height: u8, row_idx: usize, col_idx: usize, dir: Direction) -> usize {
        let mut count = 0;
        let iter = self.direction_iter(row_idx, col_idx, dir).skip(1);
        for tree in iter {
            count += 1;
            if tree.height >= height {
                break;
            }
        }
        count
    }
}

#[derive(Clone, Debug)]
struct Tree {
    height: u8,
    vis_north: bool,
    vis_south: bool,
    vis_east: bool,
    vis_west: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height,
            vis_north: false,
            vis_south: false,
            vis_east: false,
            vis_west: false,
        }
    }
    fn visible(&self) -> bool {
        self.vis_north || self.vis_south || self.vis_east || self.vis_west
    }
}

fn parse(contents: &str) -> Forest {
    let grid = contents
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| Tree::new(c as u8 - b'0'))
        .collect::<Vec<_>>();
    match contents.find('\n') {
        Some(len) if len != 0 => Forest::new(grid, len),
        _ => panic!("no newline in file"),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn GIVEN_small_grid_WHEN_getting_dir_iter_THEN_expected_counts_returned() {
        let grid = "123\n345\n567";
        let forest = parse(grid);
        assert_eq!(3, forest.n_cols);
        assert_eq!(3, forest.n_rows);
        assert_eq!(3, forest.direction_iter(1, 0, Direction::East).count());
        assert_eq!(2, forest.direction_iter(1, 1, Direction::East).count());
        assert_eq!(1, forest.direction_iter(1, 2, Direction::East).count());
        assert_eq!(1, forest.direction_iter(1, 0, Direction::West).count());
        assert_eq!(2, forest.direction_iter(1, 1, Direction::West).count());
        assert_eq!(3, forest.direction_iter(1, 2, Direction::West).count());
        assert_eq!(1, forest.direction_iter(0, 1, Direction::North).count());
        assert_eq!(2, forest.direction_iter(1, 1, Direction::North).count());
        assert_eq!(3, forest.direction_iter(2, 1, Direction::North).count());
        assert_eq!(3, forest.direction_iter(0, 1, Direction::South).count());
        assert_eq!(2, forest.direction_iter(1, 1, Direction::South).count());
        assert_eq!(1, forest.direction_iter(2, 1, Direction::South).count());
    }

    static EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(21, part1(EXAMPLE));
    }
    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(8, part2(EXAMPLE));
    }
}
