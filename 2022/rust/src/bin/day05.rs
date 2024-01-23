use regex::Regex;
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

fn part1(contents: &str) -> String {
    run(contents, true)
}

fn part2(contents: &str) -> String {
    run(contents, false)
}

fn run(contents: &str, reverse: bool) -> String {
    let (stack_lines, inst_lines): (Vec<_>, Vec<_>) = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .partition(|line| !line.starts_with("move"));
    let mut stacks = Stacks::new(&stack_lines);
    let instructions = Instruction::parse_lines(inst_lines.into_iter());
    for instruction in instructions.into_iter() {
        stacks.execute(&instruction, reverse);
    }
    stacks.tops()
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl Instruction {
    fn parse_lines<'a>(iter: impl std::iter::Iterator<Item = &'a str>) -> Vec<Instruction> {
        let matcher = Regex::new(r"move (\d+) from (\d+) to (\d+)")
            .expect("couldn't compile instruction regex");
        iter.enumerate()
            .map(|(idx, line)| {
                let caps = matcher
                    .captures(line)
                    .unwrap_or_else(|| panic!("unexpected formatting for instruction {idx}"));
                let count: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();
                Instruction { count, from, to }
            })
            .collect()
    }
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(lines: &[&str]) -> Self {
        let mut stacks = Vec::<Vec<char>>::new();
        let matcher: Regex = Regex::new(r"\[[A-Z]\]").expect("Couldn't compile stack regex");
        for &line in lines {
            matcher.find_iter(line).for_each(|m| {
                let mut pos = m.range().start;
                let token = m.as_str().chars().nth(1).unwrap();
                pos /= 4;
                while stacks.len() < (pos + 1) {
                    stacks.push(Vec::new());
                }
                stacks[pos].push(token);
            });
        }
        stacks.iter_mut().for_each(|stack| stack.reverse());
        Stacks { stacks }
    }
    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|v| v.last().expect("unexpected empty stack"))
            .collect()
    }
    fn execute(&mut self, instruction: &Instruction, reverse: bool) {
        let count = instruction.count;
        let from_stack = self
            .stacks
            .get_mut(instruction.from - 1)
            .expect("unable to get \"from\" stack");
        let len = from_stack.len();
        let items = from_stack.drain((len - count)..len);
        let items: Vec<_> = match reverse {
            true => items.rev().collect(),
            false => items.collect(),
        };
        self.stacks
            .get_mut(instruction.to - 1)
            .expect("unable to get \"to\" stack")
            .extend(items);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test05 {
    use super::*;

    #[test]
    fn GIVEN_lines_WHEN_creating_stacks_THEN_correct_stacks_produced() {
        let text = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

"#;
        let lines: Vec<_> = text
            .lines()
            .filter(|&line| !line.trim().is_empty())
            .collect();
        let stacks = Stacks::new(&lines);
        assert_eq!(3, stacks.stacks.len());

        assert_eq!(vec!['Z', 'N'], stacks.stacks[0]);
        assert_eq!(vec!['M', 'C', 'D'], stacks.stacks[1]);
        assert_eq!(vec!['P'], stacks.stacks[2]);
    }

    static AOC_EXAMPLE_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_running_part1_THEN_final_answer_agrees() {
        let tops = part1(AOC_EXAMPLE_INPUT);
        assert_eq!("CMZ".to_owned(), tops);
    }

    #[test]
    fn GIVEN_aoc_example_input_WHEN_running_part2_THEN_final_answer_agrees() {
        let tops = part2(AOC_EXAMPLE_INPUT);
        assert_eq!("MCD".to_owned(), tops);
    }
}
