use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    //println!("part2 total is {}", part2(contents.as_str()));
}

type InputMap = Vec<Vec<Option<Pipe>>>;

fn part1(contents: &str) -> usize {
    use Direction::*;
    let (map, start) = parse_file(contents);
    let (row_idx, col_idx) = start;
    for dir in [N, S, E, W] {
        if let Some((r, c)) = try_step(&map, row_idx, col_idx, dir) {
            if let Some(pipe_segment) = map[r][c] {
                if pipe_segment.has_connection(dir) {
                    let halfway = walk(&map, start, r, c, dir) as f64 * 0.5;
                    return halfway.ceil() as usize;
                }
            }
        }
    }
    panic!("couldn't find start direction from {start:#?}");
}

fn try_step(
    map: &InputMap,
    row_idx: usize,
    col_idx: usize,
    dir: Direction,
) -> Option<(usize, usize)> {
    use Direction::*;
    match dir {
        N => {
            if row_idx > 0 {
                Some((row_idx - 1, col_idx))
            } else {
                None
            }
        }
        S => {
            if row_idx < map.len() - 1 {
                Some((row_idx + 1, col_idx))
            } else {
                None
            }
        }
        E => {
            if col_idx < map[0].len() - 1 {
                Some((row_idx, col_idx + 1))
            } else {
                None
            }
        }
        W => {
            if col_idx > 0 {
                Some((row_idx, col_idx - 1))
            } else {
                None
            }
        }
    }
}

fn walk(
    map: &InputMap,
    start: (usize, usize),
    mut row_idx: usize,
    mut col_idx: usize,
    mut dir: Direction,
) -> usize {
    use Direction::*;
    let mut nsteps: usize = 0;
    while (row_idx, col_idx) != start {
        let pipe = map[row_idx][col_idx].expect("walked onto unmapped segment!");
        let other = pipe.other_dir(dir.opposite());
        match other {
            N => row_idx -= 1,
            S => row_idx += 1,
            E => col_idx += 1,
            W => col_idx -= 1,
        };
        dir = other;
        nsteps += 1;
    }
    nsteps
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match *self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn new(c: char) -> Option<Self> {
        use Direction::*;
        match c {
            '|' => Some(Self(N, S)),
            '-' => Some(Self(E, W)),
            'L' => Some(Self(N, E)),
            'J' => Some(Self(N, W)),
            'F' => Some(Self(S, E)),
            '7' => Some(Self(S, W)),
            _ => None,
        }
    }

    fn other_dir(&self, dir: Direction) -> Direction {
        if dir == self.0 {
            self.1
        } else {
            assert!(dir == self.1);
            self.0
        }
    }

    fn has_connection(&self, dir: Direction) -> bool {
        self.0 == dir || self.1 == dir
    }

    #[allow(dead_code)] // for testing
    fn connects_to(&self, other: Self, dir: Direction) -> bool {
        use Direction::*;
        if !self.has_connection(dir) {
            return false;
        }
        match dir {
            N => other.has_connection(S),
            S => other.has_connection(N),
            E => other.has_connection(W),
            W => other.has_connection(E),
        }
    }
}

fn parse_file(contents: &str) -> (InputMap, (usize, usize)) {
    let mut start: Option<(usize, usize)> = None;
    let parse_line = |(line_idx, line): (usize, &str)| {
        line.chars()
            .enumerate()
            .map(|(col_idx, c)| {
                if 'S' == c {
                    start = Some((line_idx, col_idx));
                    None
                } else {
                    Pipe::new(c)
                }
            })
            .collect::<Vec<_>>()
    };
    let tiles = contents
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(parse_line)
        .collect::<Vec<_>>();
    if let Some(coords) = start {
        (tiles, coords)
    } else {
        panic!("couldn't fine start tile");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test10 {
    use super::*;

    #[test]
    fn GIVEN_some_pipes_WHEN_testing_connectivity_THEN_correct() {
        use Direction::*;
        assert!(Pipe(N, W).has_connection(N));
        assert!(!Pipe(N, W).has_connection(S));

        assert!(Pipe(S, E).connects_to(Pipe(N, E), S));
        assert!(!Pipe(S, E).connects_to(Pipe(N, E), W));
        assert!(!Pipe(S, E).connects_to(Pipe(S, E), E));
        assert!(!Pipe(S, E).connects_to(Pipe(N, S), E));
    }

    static EXAMPLE_INPUT1: &str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
"#;

    #[test]
    fn GIVEN_small_file_WHEN_parsing_THEN_correct_pipes_produced() {
        use Direction::*;
        let (map, start) = parse_file(EXAMPLE_INPUT1);
        assert_eq!((1, 1), start);
        assert_eq!(None, map[0][0]);
        assert_eq!(None, map[4][4]);
        assert_eq!(Some(Pipe(E, W)), map[1][2]);
        assert_eq!(Some(Pipe(S, W)), map[1][3]);
        assert_eq!(Some(Pipe(N, S)), map[2][3]);
        assert_eq!(Some(Pipe(N, W)), map[3][3]);
        assert_eq!(Some(Pipe(E, W)), map[3][2]);
        assert_eq!(Some(Pipe(N, E)), map[3][1]);
        assert_eq!(Some(Pipe(N, S)), map[2][1]);
    }

    static EXAMPLE_INPUT2: &str = r#"
.....
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(4, part1(EXAMPLE_INPUT1));
        assert_eq!(8, part1(EXAMPLE_INPUT2));
    }
}
