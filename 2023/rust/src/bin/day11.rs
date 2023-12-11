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
    calculate(contents, 2)
}

fn part2(contents: &str) -> usize {
    calculate(contents, 1_000_000)
}

fn calculate(contents: &str, multiplier: usize) -> usize {
    let (nrows, ncols, galaxies) = parse_file(contents);
    let (row_weights, col_weights) = get_weights(&galaxies, nrows, ncols, multiplier);
    let mut total = 0;
    for (idx1, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(idx1 + 1) {
            total += row_weights[g1.0.min(g2.0)..=g1.0.max(g2.0)]
                .iter()
                .sum::<usize>()
                - 1;
            total += col_weights[g1.1.min(g2.1)..=g1.1.max(g2.1)]
                .iter()
                .sum::<usize>()
                - 1;
        }
    }
    total
}

type CoOrd = (usize, usize);

fn parse_file(contents: &str) -> (usize, usize, Vec<CoOrd>) {
    let mut result = Vec::new();
    let mut nrows = 0;
    let mut ncols = 0;
    let parse_line = |(row_idx, line): (usize, &str)| {
        line.chars().enumerate().for_each(|(col_idx, c)| {
            if '#' == c {
                result.push((row_idx, col_idx))
            }
            ncols = ncols.max(col_idx);
        });
        nrows = nrows.max(row_idx);
    };
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .for_each(parse_line);
    (nrows + 1, ncols + 1, result)
}

fn get_weights(
    galaxies: &[CoOrd],
    nrows: usize,
    ncols: usize,
    multiplier: usize,
) -> (Vec<usize>, Vec<usize>) {
    let weights_fn = |num, pred: fn(&CoOrd, usize) -> bool| {
        (0..num)
            .map(|idx| {
                if galaxies.iter().any(|g| pred(g, idx)) {
                    1
                } else {
                    multiplier
                }
            })
            .collect::<Vec<_>>()
    };
    let row_weights = weights_fn(nrows, |g, idx| g.0 == idx);
    let col_weights = weights_fn(ncols, |g, idx| g.1 == idx);
    (row_weights, col_weights)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test11 {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn GIVEN_small_file_WHEN_parsing_THEN_correct_coords_produced() {
        let (nrows, ncols, galaxies) = parse_file(EXAMPLE_INPUT);
        assert_eq!(10, nrows);
        assert_eq!(10, ncols);
        assert!(galaxies.contains(&(0, 3)));
        assert!(galaxies.contains(&(6, 9)));
        assert!(galaxies.contains(&(9, 0)));
        assert!(!galaxies.contains(&(6, 6)));
    }

    #[test]
    fn GIVEN_galaxy_map_WHEN_calcing_wieghts_THEN_empty_rows_doubled() {
        let galaxies = vec![(0, 0), (0, 2), (3, 2)];
        let (rw, cw) = get_weights(&galaxies, 4, 5, 2);
        assert_eq!(vec![1, 2, 2, 1], rw);
        assert_eq!(vec![1, 2, 1, 2, 2], cw);

        let (nrows, ncols, galaxies) = parse_file(EXAMPLE_INPUT);
        let (rw, cw) = get_weights(&galaxies, nrows, ncols, 2);
        assert_eq!(nrows + 2, rw.iter().sum());
        assert_eq!(ncols + 3, cw.iter().sum());
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(374, part1(EXAMPLE_INPUT));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(1030, calculate(EXAMPLE_INPUT, 10));
        assert_eq!(8410, calculate(EXAMPLE_INPUT, 100));
    }
}
