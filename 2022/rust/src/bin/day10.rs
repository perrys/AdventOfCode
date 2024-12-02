use std::fs;

/* start with line:

~/dev/advent_of_code/2022/rust/target/debug/day10 /home/stu/dev/advent_of_code/2022/input/day10.dat

*/

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    println!("Part 1 answer is {}", part1(contents.as_str()));
    println!("Part 2 answer is:");
    let lines = part2(contents.as_str());
    lines.iter().for_each(|line| println!("{}", line));
}

fn part1(_contents: &str) -> i32 {
    let instructions = parse_instructions(_contents);
    let observed_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut cpu = Cpu::new();
    let observations = cpu.execute(&instructions, &observed_cycles, None);
    observations.into_iter().sum()
}

fn part2(_contents: &str) -> Vec<String> {
    let instructions = parse_instructions(_contents);
    let mut cpu = Cpu::new();
    let mut crt = Crt::new(40, 6);
    let _ = cpu.execute(&instructions, &Vec::new(), Some(&mut crt));
    crt.to_strings()
}

fn parse_instructions(_contents: &str) -> Vec<Instruction> {
    let instructions = _contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(Instruction::new)
        .collect::<Vec<_>>();
    instructions
}

#[allow(non_camel_case_types)]
enum Instruction {
    nop,
    addx(i32),
}

impl Instruction {
    fn new(line: &str) -> Self {
        let toks = line.split(' ').collect::<Vec<_>>();
        match toks[0] {
            "noop" => Self::nop,
            "addx" => Self::addx(toks[1].parse::<i32>().unwrap()),
            _ => panic!("unknown instruction \"{}\"", toks[0]),
        }
    }
}

struct Crt {
    x_position: usize,
    y_position: usize,
    n_columns: usize,
    n_rows: usize,
    pixels: Vec<char>,
}

impl Crt {
    fn new(n_columns: usize, n_rows: usize) -> Self {
        Self {
            x_position: 0,
            y_position: 0,
            n_columns,
            n_rows,
            pixels: vec![' '; n_rows * n_columns + 1],
        }
    }
    fn advance(&mut self, x_register: i32) {
        let pixel = self.x_position + self.y_position * self.n_columns;
        self.pixels[pixel] = match (self.x_position as i32 - x_register).abs() {
            distance if distance < 2 => '#',
            _ => ' ',
        };
        self.x_position += 1;
        if self.x_position == self.n_columns {
            self.x_position = 0;
            self.y_position = (self.y_position + 1) % self.n_rows;
        }
    }
    fn to_strings(&self) -> Vec<String> {
        self.pixels
            .chunks(self.n_columns)
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<_>>()
    }
}

struct Cpu {
    x_register: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x_register: 1,
            cycle: 0,
        }
    }
    fn execute(
        &mut self,
        program: &[Instruction],
        observed_cycles: &[usize],
        mut crt: Option<&mut Crt>,
    ) -> Vec<i32> {
        let mut watch_points = Vec::<i32>::new();
        program.iter().for_each(|instruction| {
            let (n_cycles, value_change) = match instruction {
                Instruction::addx(value) => (2, *value),
                Instruction::nop => (1, 0),
            };
            for _i in 0..n_cycles {
                if let Some(ref mut crt) = crt {
                    crt.advance(self.x_register);
                }
                self.cycle += 1;
                if observed_cycles.contains(&self.cycle) {
                    watch_points.push(self.cycle as i32 * self.x_register);
                }
            }
            self.x_register += value_change;
        });
        watch_points
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(13140, part1(EXAMPLE));
    }
    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        let expected = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
 "#;
        let expected = expected
            .replace('.', " ")
            .split('\n')
            .map(str::to_owned)
            .collect::<Vec<_>>();

        assert_eq!(expected, part2(EXAMPLE));
    }
}
