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

fn part1(contents: &str) -> i64 {
    let lines = parse_file(contents);
    lines.iter().map(|l| predict(l, true)).sum()
}

fn part2(contents: &str) -> i64 {
    let lines = parse_file(contents);
    lines.iter().map(|l| predict(l, false)).sum()
}

fn predict(points: &[i64], forwards: bool) -> i64 {
    if points.iter().all(|n| 0 == *n) {
        0
    } else {
        let derivative = differentiate(points);
        let delta = predict(&derivative, forwards);
        if forwards {
            points[points.len() - 1] + delta
        } else {
            points[0] - delta
        }
    }
}

fn differentiate(points: &[i64]) -> Vec<i64> {
    points
        .iter()
        .enumerate()
        .skip(1)
        .map(|(idx, point)| point - points[idx - 1])
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|n| n.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("couldn't parse line \"{line}\""))
}

fn parse_file(contents: &str) -> Vec<Vec<i64>> {
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect::<Vec<_>>()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test09 {
    use super::*;

    #[test]
    fn GIVEN_polynomial_points_WHEN_differentiating_THEN_derivative_points_produced() {
        let dotest = |points: &[i64], derivative| {
            assert_eq!(derivative, differentiate(points));
        };
        // y = x^2
        dotest(&[9, 16, 25, 36, 49, 64], vec![7, 9, 11, 13, 15]);
        dotest(&[7, 9, 11, 13, 15], vec![2, 2, 2, 2]);
        dotest(&[2, 2, 2, 2], vec![0, 0, 0]);
    }

    static EXAMPLE_INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(114, part1(EXAMPLE_INPUT));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(2, part2(EXAMPLE_INPUT));
    }
}
