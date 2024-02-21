use std::{collections::HashSet, fs};

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
    solve::<2>(contents)
}

fn part2(contents: &str) -> usize {
    solve::<10>(contents)
}

fn solve<const N: usize>(contents: &str) -> usize {
    let instructions = parse_instructions(contents);
    let mut rope = Rope::<N>::new();
    let mut tail_visited = HashSet::<(i32, i32)>::new();
    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope.move_head(instruction.0);
            tail_visited.insert(rope.vertices[N - 1]);
        }
    }
    tail_visited.len()
}

fn parse_instructions(contents: &str) -> Vec<(Direction, usize)> {
    let instructions = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut toks = line.split(' ');
            let dir = Direction::new(toks.next().unwrap().chars().next().unwrap());
            let len = toks.next().unwrap().parse::<usize>().unwrap();
            (dir, len)
        })
        .collect::<Vec<_>>();
    instructions
}

#[derive(Copy, Debug, Clone)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn new(l: char) -> Self {
        match l {
            'L' => Self::L,
            'R' => Self::R,
            'U' => Self::U,
            'D' => Self::D,
            _ => panic!("unknown direction"),
        }
    }
}

struct Rope<const N: usize> {
    vertices: [(i32, i32); N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        let vertices = [(0, 0); N];
        Self { vertices }
    }

    fn move_head(&mut self, dir: Direction) {
        let head_pos = &mut self.vertices[0];
        match dir {
            Direction::L => head_pos.0 -= 1,
            Direction::R => head_pos.0 += 1,
            Direction::U => head_pos.1 -= 1,
            Direction::D => head_pos.1 += 1,
        }
        self.move_tail();
    }

    fn move_tail(&mut self) {
        for i in 1..N {
            let head_pos = self.vertices[i - 1];
            let tail_pos = &mut self.vertices[i];

            let dx = head_pos.0 - tail_pos.0;
            let dy = head_pos.1 - tail_pos.1;
            if dx.abs() > 1 && dy.abs() > 1 {
                tail_pos.0 += dx / dx.abs();
                tail_pos.1 += dy / dy.abs();
            } else if dx.abs() > 1 {
                tail_pos.0 += dx / 2;
                tail_pos.1 = head_pos.1;
            } else if dy.abs() > 1 {
                tail_pos.0 = head_pos.0;
                tail_pos.1 += dy / 2;
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tester {
    use super::*;

    static EXAMPLE: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(13, part1(EXAMPLE));
    }
    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(1, part2(EXAMPLE));
    }
    static EXAMPLE2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;
    #[test]
    fn GIVEN_aoc_example2_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(36, part2(EXAMPLE2));
    }
}
