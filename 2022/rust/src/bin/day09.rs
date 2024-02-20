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
    let mut rope = Rope::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope.move_head(instruction.0);
            visited.insert(rope.tail_pos);
        }
    }
    visited.len()
}

fn part2(_contents: &str) -> usize {
    0
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

struct Rope {
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
}

impl Rope {
    fn new() -> Self {
        Rope {
            head_pos: (0, 0),
            tail_pos: (0, 0),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        match dir {
            Direction::L => self.head_pos.0 -= 1,
            Direction::R => self.head_pos.0 += 1,
            Direction::U => self.head_pos.1 -= 1,
            Direction::D => self.head_pos.1 += 1,
        }
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let dx = self.head_pos.0 - self.tail_pos.0;
        let dy = self.head_pos.1 - self.tail_pos.1;
        if dx.abs() > 1 {
            self.tail_pos.1 = self.head_pos.1;
            self.tail_pos.0 += dx / 2;
        }
        if dy.abs() > 1 {
            self.tail_pos.0 = self.head_pos.0;
            self.tail_pos.1 += dy / 2;
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
}
