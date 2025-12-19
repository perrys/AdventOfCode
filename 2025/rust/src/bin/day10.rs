fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    // println!("part1: {}", part1(&contents));
}

#[derive(Debug, PartialEq)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    fn parse_line(line: &str) -> Self {
        let mut lights = 0;
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        let tokens = line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        for token in tokens {
            let mut chars = token.chars();
            match chars.next() {
                Some('[') => {
                    for c in chars {
                        match c {
                            '.' => lights = lights << 1,
                            '#' => {
                                lights = lights << 1;
                                lights |= 1;
                            }
                            _ => (),
                        }
                    }
                }
                Some('(') => {
                    let mut button = 0;
                    token[1..token.len() - 1].split(',').for_each(|s| {
                        let n = s.parse::<u16>().expect("non-integer");
                        let bits = 1 << n;
                        button |= bits;
                    });
                    buttons.push(button);
                }
                Some('{') => {
                    joltages = token[1..token.len() - 1]
                        .split(',')
                        .map(|s| s.parse::<u16>().expect("non-integer"))
                        .collect();
                }
                _ => (),
            }
        }
        Self {
            lights,
            buttons,
            joltages,
        }
    }

    fn parse_lines(contents: &str) -> Vec<Self> {
        contents
            .lines()
            .filter_map(|line| {
                let t = line.trim();
                if t.is_empty() { None } else { Some(t) }
            })
            .map(Self::parse_line)
            .collect()
    }
}

#[cfg(test)]
mod tester {
    use super::*;

    const TEST_DATA: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "#;

    #[test]
    fn test_parse() {
        let machines = Machine::parse_lines(TEST_DATA);
        assert_eq!(3, machines.len());
        assert_eq!(
            Machine {
                lights: 6,
                buttons: vec![8, 2 + 8, 4, 4 + 8, 1 + 4, 1 + 2],
                joltages: vec![3, 5, 4, 7]
            },
            machines[0]
        );
        assert_eq!(
            Machine {
                lights: 2,
                buttons: vec![1 + 4 + 8 + 16, 4 + 8, 1 + 16, 1 + 2 + 4, 2 + 4 + 8 + 16],
                joltages: vec![7, 5, 12, 7, 2]
            },
            machines[1]
        );
    }
}
