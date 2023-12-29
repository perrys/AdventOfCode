//!
//! Advent of code challenge 2024 day 24.
//!
//! See <https://adventofcode.com/2024/day/24>
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

    println!("part1 total is {}", part1(contents.as_str()));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    let range = 200000000000000f64..=400000000000000f64;
    get_valid_intercepts(contents, range)
}

fn part2(_contents: &str) -> usize {
    0
}

fn get_valid_intercepts(contents: &str, range: std::ops::RangeInclusive<f64>) -> usize {
    let test = |hs1: &HailStone, hs2| {
        let point = hs1.intercept_point(hs2);
        match point {
            None => false,
            Some((x, y)) => {
                range.contains(&x)
                    && range.contains(&y)
                    // the "forward in time" clause:
                    && if hs1.vy == 0.0 {
                        y == hs1.y
                    } else if hs1.vy > 0.0 {
                        y > hs1.y
                    } else {
                        y < hs1.y
                    }
                    && if hs2.vy == 0.0 {
                        y == hs2.y
                    } else if hs2.vy > 0.0 {
                        y > hs2.y
                    } else {
                        y < hs2.y
                    }
            }
        }
    };
    let hailstones = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(HailStone::new)
        .collect::<Vec<_>>();
    let mut count = 0usize;
    for (idx, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones[idx + 1..].iter() {
            if test(hs1, hs2) {
                count += 1
            }
        }
    }
    count
}

#[derive(Debug, PartialEq)]
struct HailStone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl HailStone {
    fn new(line: &str) -> Self {
        let values: [f64; 6] = line
            .split(&[' ', ',', '@'])
            .filter(|t| !t.is_empty())
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| panic!("uanble to parse line \"{line}\""))
            .try_into()
            .unwrap_or_else(|_| panic!("incorrect number of tokens in line \"{line}\""));
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
            vx: values[3],
            vy: values[4],
            vz: values[5],
        }
    }

    fn slope_and_y_intercept(&self) -> (f64, f64) {
        let m = self.vy / self.vx;
        let c = self.y - self.x * m;
        (m, c)
    }

    fn intercept_point(&self, other: &HailStone) -> Option<(f64, f64)> {
        let (m1, c1) = self.slope_and_y_intercept();
        let (m2, c2) = other.slope_and_y_intercept();
        if m1 == m2 {
            None
        } else {
            let x0 = (c2 - c1) / (m1 - m2);
            let y0 = (c1 * m2 - c2 * m1) / (m2 - m1);
            Some((x0, y0))
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test24 {
    use super::*;

    #[test]
    fn GIVEN_simple_coords_WHEN_calcing_slope_and_y_intercept_THEN_correct() {
        let dotest = |hs, expected| {
            let hs = HailStone::new(hs);
            assert_eq!(expected, hs.slope_and_y_intercept());
        };
        dotest("1, 1, 0 @ 1, 2, 0", (2.0, -1.0));
        dotest("10, -15, 0 @ 1, -3, 0", (-3.0, 15.0));
        dotest("10, -45, 0 @ 2, -6, 0", (-3.0, -15.0));
    }

    #[test]
    fn GIVEN_two_lines_WHEN_calcing_intercept_point_THEN_correct() {
        let dotest = |hs1, hs2, expected| {
            let hs1 = HailStone::new(hs1);
            let hs2 = HailStone::new(hs2);
            assert_eq!(expected, hs1.intercept_point(&hs2));
        };
        dotest("0, 0, 0 @ 2, 2, 0", "0, 0, 0 @ 1, 2, 0", Some((0.0, 0.0)));
        dotest("1, 1, 0 @ 2, 2, 0", "2, 3, 0 @ 1, 2, 0", Some((1.0, 1.0)));
        dotest("1, 1, 0 @ 2, 2, 0", "2, 3, 0 @ 2, 2, 0", None);
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(2, get_valid_intercepts(EXAMPLE, 7.0..=27.0));
    }

    static EXAMPLE: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
}
