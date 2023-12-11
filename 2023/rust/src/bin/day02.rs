//!
//! Advent of code challenge 2023 day 2.
//!
//! See <https://adventofcode.com/2023/day/2>
//!
use std::{collections::HashMap, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    println!("part1 total is {}", part1(&contents));
    println!("part2 total is {}", part2(&contents));
}

fn part1(contents: &str) -> usize {
    let games = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Game::new)
        .collect::<Vec<_>>();
    let max_cubes = [
        (CubeColor::Red, 12),
        (CubeColor::Green, 13),
        (CubeColor::Blue, 14),
    ];
    let predicate = |game: &Game| {
        let counts = HashMap::from(game.max_counts());
        max_cubes.iter().all(|(k, v)| counts.get(k).unwrap() <= v)
    };
    games.into_iter().filter(predicate).map(|g| g.game_id).sum()
}

fn part2(contents: &str) -> usize {
    let games = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Game::new)
        .collect::<Vec<_>>();
    games
        .iter()
        .map(|game| game.max_counts().iter().map(|c| c.1).product::<usize>())
        .sum()
}

#[derive(Debug)]
struct Game {
    game_id: usize,
    groups: Vec<Vec<CubeCount>>,
}

impl Game {
    fn new(line: &str) -> Self {
        let remain = line
            .strip_prefix("Game ")
            .unwrap_or_else(|| panic!("corrupt line {line}"));
        let mut iter = remain.chars();
        let game_id: usize = iter
            .by_ref()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .expect("unable to parse game ID");
        let _ = iter.by_ref().take_while(|c| !c.is_numeric()); // eat whitespace
        let parse_group = |s: &str| s.split(',').map(CubeCount::new).collect::<Vec<_>>();
        let groups = iter
            .as_str()
            .split(';')
            .map(parse_group)
            .collect::<Vec<_>>();
        Self { game_id, groups }
    }

    fn max_counts(&self) -> [(CubeColor, usize); 3] {
        let mut counts = HashMap::from([
            (CubeColor::Green, 0),
            (CubeColor::Blue, 0),
            (CubeColor::Red, 0),
        ]);
        for group in self.groups.iter() {
            for cc in group.iter() {
                let entry = counts.get_mut(&cc.color).expect("unknown color");
                *entry = cc.count.max(*entry);
            }
        }
        counts.into_iter().collect::<Vec<_>>().try_into().unwrap()
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn new(s: &str) -> Self {
        match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("unsupported color \"{s}\""),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct CubeCount {
    pub color: CubeColor,
    pub count: usize,
}

impl CubeCount {
    fn new(cc: &str) -> Self {
        let mut toks = cc.trim().split(' ');
        let count = toks
            .next()
            .unwrap_or_else(|| panic!("couldn't tokenize block count \"{cc}\""))
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("unable to parse cube count in \"{cc}\""));
        let color = CubeColor::new(
            toks.next()
                .unwrap_or_else(|| panic!("couldn't get color for \"{cc}\"")),
        );
        Self { color, count }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_valid_line_WHEN_parsing_game_THEN_populated_accurately() {
        let g = Game::new("Game 5: 6 red, 1 blue; 2 blue, 1 red, 2 green");
        assert_eq!(5, g.game_id);
        assert_eq!(2, g.groups.len());
        assert_eq!(2, g.groups[0].len());
        assert_eq!(3, g.groups[1].len());
        assert!(g.groups[0].contains(&CubeCount {
            color: CubeColor::Red,
            count: 6
        }));
        assert!(g.groups[1].contains(&CubeCount {
            color: CubeColor::Green,
            count: 2
        }));
    }

    #[test]
    #[should_panic]
    fn GIVEN_invalid_line_WHEN_parsing_game_THEN_panics() {
        let g = Game::new("Game 5: 6 orange, 1 blue; 2 blue, 1 red, 2 green");
        assert_eq!(5, g.game_id);
    }

    #[test]
    #[should_panic]
    fn GIVEN_invalid_line_2_WHEN_parsing_game_THEN_panics() {
        let g = Game::new("Game 5: 6 red, five blue; 2 blue, 1 red, 2 green");
        assert_eq!(5, g.game_id);
    }

    #[test]
    fn GIVEN_cube_groups_WHEN_calcing_max_per_group_THEN_correct_answers_returned() {
        let g = Game::new("Game 5: 6 red, 1 blue; 3 blue, 1 red, 2 green");
        let counts = g.max_counts();
        let map = HashMap::from(counts);
        assert_eq!(6, *map.get(&CubeColor::Red).unwrap());
        assert_eq!(3, *map.get(&CubeColor::Blue).unwrap());
        assert_eq!(2, *map.get(&CubeColor::Green).unwrap());
    }

    static EXAMPLE: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(8, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(2286, part2(EXAMPLE));
    }
}
