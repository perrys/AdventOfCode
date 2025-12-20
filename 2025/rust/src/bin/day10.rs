fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
}

fn part1(contents: &str) -> usize {
    let machines = Machine::parse_lines(contents);
    machines.iter().map(bfs).sum()
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
        let mut nbits = 0;
        let tokens = line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        for token in tokens {
            let mut chars = token.chars();
            match chars.next() {
                Some('[') => {
                    for c in chars {
                        match c {
                            '.' => {
                                nbits += 1;
                                lights <<= 1
                            }
                            '#' => {
                                nbits += 1;
                                lights <<= 1;
                                lights |= 1;
                            }
                            _ => (),
                        }
                    }
                }
                Some('(') => {
                    let mut button = 0;
                    token[1..token.len() - 1].split(',').for_each(|s| {
                        // read backwards:
                        let n = nbits - s.parse::<u16>().expect("non-integer") - 1;
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

#[allow(dead_code)]
fn print_presses(presses: &[u16]) {
    println!("solution found!");
    for p in presses.iter() {
        println!("{p} - {p:b}")
    }
}

fn bfs(machine: &Machine) -> usize {
    let start = machine.buttons.iter().map(|b| vec![*b]);
    let mut queue = std::collections::VecDeque::from_iter(start);
    let depth_limit = 10;
    while !queue.is_empty() {
        let presses = queue.pop_front().unwrap();
        if presses.len() > depth_limit {
            panic!("reached depth limit");
        }
        let mut lights = machine.lights;
        for p in presses.iter() {
            lights ^= p
        }
        if lights == 0 {
            // print_presses(&presses);
            return presses.len();
        }
        for p in machine.buttons.iter() {
            if !presses.contains(p) {
                let mut next = presses.clone();
                next.push(*p);
                queue.push_back(next);
            }
        }
    }
    panic!("no solution found");
}

fn gauss_elim_unit(rows: &mut [Vec<i32>]) {
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut p_idx = 0;
    for col_idx in 0..ncols {
        if let Some(pivot_row) = (p_idx..nrows).find(|&row_idx| rows[row_idx][col_idx] != 0) {
            if pivot_row != p_idx {
                rows.swap(p_idx, pivot_row);
            }
            for i in (p_idx + 1)..nrows {
                if rows[i][col_idx] == 0 {
                    continue;
                }
                let factor = rows[i][col_idx] / rows[p_idx][col_idx]; // 1 or -1
                for j in 0..ncols {
                    rows[i][j] -= factor * rows[p_idx][j];
                }
            }
            p_idx += 1;
        }
        println!("col_idx={col_idx}, p_idx={p_idx}");
        for row in rows.iter() {
            for c in row.iter() {
                print!("{c:4} ");
            }
            println!("");
        }
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
                buttons: vec![1, 4 + 1, 2, 2 + 1, 8 + 2, 8 + 4],
                joltages: vec![3, 5, 4, 7]
            },
            machines[0]
        );
        assert_eq!(
            Machine {
                lights: 2,
                buttons: vec![16 + 4 + 2 + 1, 4 + 2, 1 + 16, 16 + 8 + 4, 8 + 4 + 2 + 1],
                joltages: vec![7, 5, 12, 7, 2]
            },
            machines[1]
        );
    }

    #[test]
    fn test_ordering() {
        let machines = Machine::parse_lines(TEST_DATA);
        let test = &machines[1];
        for (i, button) in test.buttons.iter().enumerate() {
            println!("button_{i}: {button:05b}");
        }
        let mut lights = test.lights;
        println!("{lights:05b}");
        lights ^= test.buttons[2];
        println!("{lights:05b}");
        lights ^= test.buttons[3];
        println!("{lights:05b}");
        lights ^= test.buttons[4];
        println!("{lights:05b}");
        assert_eq!(0, lights);
    }

    #[test]
    fn test_bfs() {
        let machines = Machine::parse_lines(TEST_DATA);
        assert_eq!(bfs(&machines[0]), 2);
        assert_eq!(bfs(&machines[1]), 3);
        assert_eq!(bfs(&machines[2]), 2);
    }

    #[test]
    fn test_gauss_elim_unit() {
        let mut rows = vec![
            vec![1, 0, 1, 1, 0, 7],
            vec![0, 0, 0, 1, 1, 5],
            vec![1, 1, 0, 1, 1, 12],
            vec![1, 1, 0, 0, 1, 7],
            vec![1, 0, 1, 0, 1, 2],
        ];
        gauss_elim_unit(&mut rows);
        let expected = vec![
            vec![1, 0, 1, 1, 0, 7],
            vec![0, 1, -1, 0, 1, 5],
            vec![0, 0, 0, 1, 1, 5],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(expected, rows);
    }
}
