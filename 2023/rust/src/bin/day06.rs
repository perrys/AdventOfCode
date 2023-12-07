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
    println!("part2 total is {}", part2(&contents));
}

fn quadratic_solver(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant: f64 = b * b - (4.0 * a * c);
    if discriminant >= 0.0 {
        let sqrtd = discriminant.sqrt();
        let scale = 1.0 / (2.0 * a);
        let mut first = (-b + sqrtd) * scale;
        let mut second = (-b - sqrtd) * scale;
        if first > second {
            (first, second) = (second, first);
        }
        Some((first, second))
    } else {
        None
    }
}

fn discrete_number_above(time: u64, distance: u64) -> u64 {
    // x(t-x) = d
    let winner = |x| (x * (time - x)) > distance;
    match quadratic_solver(-1., time as f64, distance as f64 * -1.0) {
        Some((r1, r2)) => {
            let mut u1 = r1.round() as u64;
            let mut u2 = r2.round() as u64;
            while !winner(u1) {
                u1 += 1;
            }
            while winner(u2) {
                u2 += 1;
            }
            u2 - u1
        }
        None => panic!("no roots found for time {time} distance {distance}"),
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .expect("second part")
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("unable to parse line \"{line}\""))
}

fn part1(contents: &str) -> u64 {
    let mut lines = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line);
    let times = lines.next().expect("empty file");
    let distances = lines.next().expect("incomplete file");
    times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| discrete_number_above(t, d))
        .product::<u64>()
}

fn part2(contents: &str) -> u64 {
    let parse_line2 = |line: &str| {
        line.split(':')
            .nth(1)
            .expect("second part")
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .collect::<String>()
    };

    let mut lines = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line2);
    let time = lines.next().expect("empty file").parse::<u64>().unwrap();
    let distance = lines
        .next()
        .expect("incomplete file")
        .parse::<u64>()
        .unwrap();
    discrete_number_above(time, distance)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_quatratic_solver_WHEN_testing_known_inputs_THEN_expected_outputs_returned() {
        let dotest = |a, b, c, expected| {
            let result = quadratic_solver(a, b, c);
            if let Some((r1, r2)) = result {
                for r in [r1, r2] {
                    assert_eq!(0.0, a * r * r + b * r + c); // TODO: fp comparison should be replaced with epsilon test
                }
            }
            assert_eq!(expected, quadratic_solver(a, b, c));
        };
        dotest(1., 0., 0., Some((0.0, 0.0)));
        dotest(1., 0., 1., None);
        dotest(-1., 3., -1.25, Some((0.5, 2.5)));
    }

    #[test]
    fn GIVEN_time_and_distance_WHEN_calc_winners_THEN_known_correct_answer_returned() {
        assert_eq!(4, discrete_number_above(7, 9));
        assert_eq!(8, discrete_number_above(15, 40));
        assert_eq!(9, discrete_number_above(30, 200));
    }

    static TEST_EXAMPLE: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_expected_total_returned() {
        assert_eq!(288, part1(TEST_EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part2_run_THEN_expected_total_returned() {
        assert_eq!(71503, part2(TEST_EXAMPLE));
    }
}
