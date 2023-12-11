//!
//! Advent of code challenge 2023 day 10.
//!
//! See <https://adventofcode.com/2023/day/10>
//!
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 answer is {}", part1(contents.as_str()));
    println!("part2 answer is {:#?}", part2(contents.as_str()));
}

type InputMap = Vec<Vec<Option<Pipe>>>;

fn part1(contents: &str) -> usize {
    let (map, start) = parse_file(contents);
    let (row_idx, col_idx, dir) = get_staring_direction(&map, start);
    let halfway = walk_path(&map, start, row_idx, col_idx, dir, |_, _, _| ()) as f64 * 0.5;
    halfway.ceil() as usize
}

fn part2(contents: &str) -> (usize, usize) {
    let (map, start) = parse_file(contents);
    let (row_idx, col_idx, dir) = get_staring_direction(&map, start);
    let m = FillMap::new(&map, start, row_idx, col_idx, dir);
    m.output();
    m.count()
}

fn get_staring_direction(map: &InputMap, start: (usize, usize)) -> (usize, usize, Direction) {
    use Direction::*;
    let (row_idx, col_idx) = start;
    for dir in [N, S, E, W] {
        if let Some((r, c)) = try_step(map, row_idx, col_idx, dir) {
            if let Some(pipe_segment) = map[r][c] {
                if pipe_segment.has_connection(dir) {
                    return (r, c, dir);
                }
            }
        }
    }
    panic!("couldn't find start direction from {start:#?}");
}

#[rustfmt::skip]
fn try_step<T>(
    map: &Vec<Vec<T>>,
    r_idx: usize,
    c_idx: usize,
    dir: Direction,
) -> Option<(usize, usize)> {
    use Direction::*;
    match dir {
        N => if r_idx > 0                {Some((r_idx - 1, c_idx))} else {None}, 
        S => if r_idx < map.len() - 1    {Some((r_idx + 1, c_idx))} else {None},
        E => if c_idx < map[0].len() - 1 {Some((r_idx, c_idx + 1))} else {None},
        W => if c_idx > 0                {Some((r_idx, c_idx - 1))} else {None},
    }
}

fn walk_path<F>(
    map: &InputMap,
    start: (usize, usize),
    mut row_idx: usize,
    mut col_idx: usize,
    mut direction_of_travel: Direction,
    mut callback: F,
) -> usize
where
    F: FnMut(usize, usize, Direction),
{
    use Direction::*;
    let mut nsteps: usize = 0;
    while (row_idx, col_idx) != start {
        callback(row_idx, col_idx, direction_of_travel);
        let pipe = map[row_idx][col_idx].expect("walked onto unmapped segment!");
        let other = pipe.other_dir(direction_of_travel.opposite());
        match other {
            N => row_idx -= 1,
            S => row_idx += 1,
            E => col_idx += 1,
            W => col_idx -= 1,
        };
        direction_of_travel = other;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FillSegment {
    Left,
    Right,
    Path,
    Unknown,
}

struct FillMap(Vec<Vec<FillSegment>>);

impl FillMap {
    fn new(
        input: &InputMap,
        start: (usize, usize),
        row_idx: usize,
        col_idx: usize,
        dir: Direction,
    ) -> Self {
        let nrows = input.len();
        let ncols = input[0].len();
        let mut fillmap = (0..nrows)
            .map(|_| vec![FillSegment::Unknown; ncols])
            .collect::<Vec<_>>();
        Self::fill(&mut fillmap, input, start, row_idx, col_idx, dir);
        Self(fillmap)
    }

    fn fill(
        fillmap: &mut Vec<Vec<FillSegment>>,
        input: &InputMap,
        start: (usize, usize),
        row_idx: usize,
        col_idx: usize,
        dir: Direction,
    ) {
        fillmap[start.0][start.1] = FillSegment::Path;
        let trace_path = |r: usize, c: usize, _| {
            fillmap[r][c] = FillSegment::Path;
        };
        walk_path(input, start, row_idx, col_idx, dir, trace_path);

        let mut flood_fill_line = |r: usize, c: usize, d: Direction, filltype, flip| {
            let mut r = r;
            let mut c = c;
            let d = if flip { d.opposite() } else { d };
            while let Some((r1, c1)) = try_step(fillmap, r, c, d) {
                let segment = fillmap[r1][c1];
                match segment {
                    FillSegment::Unknown => fillmap[r1][c1] = filltype,
                    FillSegment::Path => break,
                    _ if segment != filltype => panic!("conflicting filltypes for {r1},{c1}"),
                    _ => (),
                }
                r = r1;
                c = c1;
            }
        };

        let flood_fill = |r_idx: usize, c_idx: usize, dir_of_t| {
            use Direction::*;
            if let Some(p) = input[r_idx][c_idx] {
                match p {
                    Pipe(N, S) => {
                        let flip = match dir_of_t {
                            N => false,
                            S => true,
                            _ => panic!("unexpected direction of travel for NS"),
                        };
                        flood_fill_line(r_idx, c_idx, W, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, E, FillSegment::Right, flip);
                    }
                    Pipe(E, W) => {
                        let flip = match dir_of_t {
                            E => false,
                            W => true,
                            _ => panic!("unexpected direction of travel for EW"),
                        };
                        flood_fill_line(r_idx, c_idx, N, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, S, FillSegment::Right, flip);
                    }
                    Pipe(N, E) => {
                        let flip = match dir_of_t {
                            S => false,
                            W => true,
                            _ => panic!("unexpected direction of travel for NE"),
                        };
                        flood_fill_line(r_idx, c_idx, N, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, S, FillSegment::Right, flip);
                        flood_fill_line(r_idx, c_idx, E, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, W, FillSegment::Right, flip);
                    }
                    Pipe(N, W) => {
                        let flip = match dir_of_t {
                            S => false,
                            E => true,
                            _ => panic!("unexpected direction of travel for NW"),
                        };
                        flood_fill_line(r_idx, c_idx, S, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, N, FillSegment::Right, flip);
                        flood_fill_line(r_idx, c_idx, E, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, W, FillSegment::Right, flip);
                    }
                    Pipe(S, E) => {
                        let flip = match dir_of_t {
                            N => false,
                            W => true,
                            _ => panic!("unexpected direction of travel for SE"),
                        };
                        flood_fill_line(r_idx, c_idx, N, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, S, FillSegment::Right, flip);
                        flood_fill_line(r_idx, c_idx, W, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, E, FillSegment::Right, flip);
                    }
                    Pipe(S, W) => {
                        let flip = match dir_of_t {
                            N => false,
                            E => true,
                            _ => panic!("unexpected direction of travel for SW"),
                        };
                        flood_fill_line(r_idx, c_idx, S, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, N, FillSegment::Right, flip);
                        flood_fill_line(r_idx, c_idx, W, FillSegment::Left, flip);
                        flood_fill_line(r_idx, c_idx, E, FillSegment::Right, flip);
                    }
                    _ => (),
                }
            }
        };
        walk_path(input, start, row_idx, col_idx, dir, flood_fill);
    }

    fn count(&self) -> (usize, usize) {
        let left = self
            .0
            .iter()
            .map(|l| l.iter().filter(|&s| *s == FillSegment::Left).count())
            .sum();
        let right = self
            .0
            .iter()
            .map(|l| l.iter().filter(|&s| *s == FillSegment::Right).count())
            .sum();
        (left, right)
    }

    fn output(&self) {
        for row in self.0.iter() {
            println!(
                "{}",
                String::from_iter(row.iter().map(|s| match s {
                    FillSegment::Unknown => '.',
                    FillSegment::Left => 'L',
                    FillSegment::Right => 'R',
                    FillSegment::Path => '*',
                }))
            );
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

    static EXAMPLE_INPUT3: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!((30, 10), part2(EXAMPLE_INPUT3));
    }
}
