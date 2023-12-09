use std::collections::HashMap;
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
    //    println!("part2 total is {}", part2(&contents));
}

fn part1(contents: &str) -> usize {
    let (map, directions) = parse_file(contents);
    walk_map(map, directions, "AAA".to_owned())
}

type Map = HashMap<String, (String, String)>;

fn walk_map(map: Map, directions: Vec<bool>, start: String) -> usize {
    let mut next = &start;
    let mut nsteps = 0;
    while next != "ZZZ" {
        let entry = map
            .get(next)
            .unwrap_or_else(|| panic!("unable to find network node for \"{next}\""));
        next = match directions[nsteps % directions.len()] {
            true => &entry.0,
            false => &entry.1,
        };
        nsteps += 1
    }
    nsteps
}

fn parse_file(contents: &str) -> (Map, Vec<bool>) {
    let mut lines = contents.lines().filter(|l| !l.trim().is_empty());
    let directions = lines
        .next()
        .expect("empty file")
        .chars()
        .map(|c| c == 'L')
        .collect::<Vec<_>>();
    let mut map = Map::new();
    for line in lines {
        let parts: [&str; 3] = line
            .split([' ', '=', ',', '(', ')'])
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    return None;
                };
                Some(s)
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("line should have 3 parts");
        map.insert(parts[0].into(), (parts[1].into(), parts[2].into()));
    }
    (map, directions)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test08 {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part1_run_THEN_expected_total_returned() {
        let (map, directions) = parse_file(EXAMPLE_INPUT);
        assert_eq!(2, walk_map(map, directions, "AAA".to_owned()));
    }
}
