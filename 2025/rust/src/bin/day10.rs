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

    fn to_similtaneous_eqns(&self) -> Vec<Vec<i32>> {
        let nrows = self.joltages.len();
        let ncols = self.buttons.len() + 1;
        let mut rows = Vec::with_capacity(nrows);
        for i in 0..nrows {
            rows.push(vec![0; ncols]);
            rows[i][ncols - 1] = self.joltages[i] as i32;
        }
        for (j, button) in self.buttons.iter().enumerate() {
            for i in 0..nrows {
                let shifted = button >> i;
                let i_prime = nrows - i - 1; // bits are reversed
                rows[i_prime][j] = (shifted & 1) as i32;
            }
        }
        rows
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

#[allow(dead_code)]
fn print_matrix(rows: &[Vec<i32>]) {
    for row in rows.iter() {
        for c in row.iter() {
            print!("{c:4} ");
        }
        println!("");
    }
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
        // println!("col_idx={col_idx}, p_idx={p_idx}");
    }
}

fn isZero(row: &[i32]) -> bool {
    row.iter().all(|&v| v == 0)
}
/**
 * Solve by simple back-substitution
 */
fn solve(m: &[Vec<i32>]) -> Vec<i32> {
    let nrows = m.len();
    let ncols = m[0].len();
    let last = ncols - 2;
    let mut pivot = last;
    let mut x = vec![0; ncols - 1];

    for (i, row) in m.iter().enumerate().rev() {
        if isZero(row) {
            continue;
        }
        while row[pivot] == 0 {
            pivot -= 1;
        }
        let mut val = row[ncols - 1];
        for j in pivot + 1..=last {
            val -= row[j] * x[j];
        }
        val /= row[pivot]; // 1 or -1
        x[pivot] = val;
    }
    x
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
    fn test_bit_decoder() {
        let machines = Machine::parse_lines(TEST_DATA);
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        let expected = vec![
            vec![1, 0, 1, 1, 0, 7],
            vec![0, 0, 0, 1, 1, 5],
            vec![1, 1, 0, 1, 1, 12],
            vec![1, 1, 0, 0, 1, 7],
            vec![1, 0, 1, 0, 1, 2],
        ];
        assert_eq!(machines[1].to_similtaneous_eqns(), expected);
    }

    #[test]
    fn test_gauss_elim_unit() {
        let machines = Machine::parse_lines(TEST_DATA);
        let mut rows = machines[0].to_similtaneous_eqns();
        print_matrix(&rows);
        gauss_elim_unit(&mut rows);
        //       x1  x2    x3  x4    x5    x6
        //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let mut expected = vec![
            vec![1, 1, 0, 1, 0, 0, 7], // x1 + x2 + x4 = 7
            vec![0, 1, 0, 0, 0, 1, 5], // x2 + x6 = 5
            vec![0, 0, 1, 1, 1, 0, 4], // x3 + x4 + x5 = 4
            vec![0, 0, 0, 0, 1, 1, 3], // x5 + x6 = 3
        ];
        // x4 is free and x4 >= 0
        // x6 is free and x6 >= 5
        assert_eq!(expected, rows);
        rows = machines[1].to_similtaneous_eqns();
        print_matrix(&rows);
        gauss_elim_unit(&mut rows);
        //        x1        x2    x3    x4      x5
        //[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        expected = vec![
            vec![1, 0, 1, 1, 0, 7],  // x1 + x3 + x4 = 7
            vec![0, 1, -1, 0, 1, 5], // x2 - x3 + x5 = 5
            vec![0, 0, 0, 1, 1, 5],  // x4 + x5 = 5 hence x4 = 5
            vec![0, 0, 0, 0, 1, 0],  // x5 = 0
            vec![0, 0, 0, 0, 0, 0],
        ];
        // x4 = 5
        // x3 is free and 5 + x3 > 0
        // x3 > 0, set x3 = 0
        // x1 = 2
        // x2 = 5
        assert_eq!(expected, rows);

        //         x1          x2      x3          x4
        //[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        rows = machines[2].to_similtaneous_eqns();
        print_matrix(&rows);
        gauss_elim_unit(&mut rows);
        println!("soln");
        expected = vec![
            vec![1, 1, 1, 0, 10],  // x1 + x2 + x3 = 10
            vec![0, -1, 0, 1, 1],  // -x2 + x4 = 1
            vec![0, 0, -1, 0, -5], // -x3 = -5
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        // x3 = 5
        // x4 is free and x4 >= 1
        // set x4 = 1 => x2 = 0, x1 = 5
        assert_eq!(expected, rows);
    }

    #[test]
    fn test_solver() {
        let machines = Machine::parse_lines(TEST_DATA);
        let mut rows = machines[0].to_similtaneous_eqns();
        gauss_elim_unit(&mut rows);
        print_matrix(&rows);
        let solution = solve(&rows);
        println!("sol: {solution:?}");
    }
}
