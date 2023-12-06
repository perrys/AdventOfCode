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
    // println!("part2 total is {}", part2(&contents));
}

fn split_line(line: &str) -> [&str; 2] {
    let prefix = line.find(": ").expect("Couldn't find first colon");
    line[1 + prefix..]
        .split("|")
        .map(str::trim)
        .collect::<Vec<_>>()
        .try_into()
        .expect("didn't split into 2 parts")
}

fn part1_parse_line(line: &str) -> usize {
    let [win_nums, my_nums] = split_line(line);
    let mut win_nums = win_nums
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<usize>().expect("unparsable number in {win_nums}"))
        .collect::<Vec<_>>();
    win_nums.sort();
    let mut total: usize = 0;
    my_nums
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .for_each(|nstr| {
            let n = nstr
                .parse::<usize>()
                .expect("unable to parse number in {my_nums}");
            if let Some(_) = win_nums.binary_search(&n).ok() {
                total = match total {
                    0 => 1,
                    _ => total * 2,
                }
            };
        });
    total
}

fn part1(content: &str) -> usize {
    content
        .lines()
        .filter(|&s| !s.trim().is_empty())
        .map(part1_parse_line)
        .sum()
}
#[allow(non_snake_case)]
#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_valid_line_WHEN_splitting_THEN_parts_returned() {
        let line = "c1: 1234 | 4567 ";
        let parts = split_line(line);
        assert_eq!(parts[0], "1234");
        assert_eq!(parts[1], "4567");
    }

    #[test]
    fn GIVEN_valid_lines_WHEN_run_part1_parse_THEN_expecte_total_returned() {
        let dotest = |line, expected| {
            assert_eq!(expected, part1_parse_line(line));
        };
        dotest("c1: 1234 | 4567 ", 0);
        dotest("c1: 1234 | 1234 ", 1);
        dotest("c1: 3 2 4 |  2 4 5 67 ", 2);
        dotest("c1: 1 2 3 4 | 4 3 2 1 ", 8);
    }

    static EXAMPLE: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_expected_total_returned() {
        assert_eq!(13, part1(EXAMPLE));
    }
}
