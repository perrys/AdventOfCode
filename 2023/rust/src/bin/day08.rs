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

    println!("part 1 answer is {}", part1(contents.as_str()));
    println!("part 2 answer is {}", part2(&contents));
}

type NodeName = [char; 3];
type Map = HashMap<NodeName, (NodeName, NodeName)>;

fn part1(contents: &str) -> usize {
    let (map, directions) = parse_file(contents);
    walk_map(&map, &directions, &['A', 'A', 'A'], |node: &NodeName| {
        *node == ['Z', 'Z', 'Z']
    })
}

fn part2(contents: &str) -> usize {
    let (map, directions) = parse_file(contents);
    let start_nodes = map
        .keys()
        .filter_map(|n| if 'A' == n[2] { Some(*n) } else { None })
        .collect::<Vec<_>>();
    walk_map_simultaneously(map, directions, start_nodes, |node: &NodeName| {
        'Z' == node[2]
    })
}

fn walk_map(
    map: &Map,
    directions: &Vec<bool>,
    start: &NodeName,
    predicate: impl Fn(&[char; 3]) -> bool,
) -> usize {
    let mut next = start;
    let mut nsteps = 0;
    while !predicate(next) {
        let entry = map.get(next).unwrap_or_else(|| {
            panic!(
                "unable to find network node for \"{}\"",
                String::from_iter(next)
            )
        });
        next = match directions[nsteps % directions.len()] {
            true => &entry.0,
            false => &entry.1,
        };
        nsteps += 1
    }
    nsteps
}

fn walk_map_simultaneously(
    route_map: Map,
    directions: Vec<bool>,
    start: Vec<NodeName>,
    predicate: impl Fn(&[char; 3]) -> bool,
) -> usize {
    // The AoC data is arranged so that each lane loops back over itself,
    // therefore the answer is the lowest common multiple of the individual lane
    // loop counts. This doesn't seem to be a general solution to the problem,
    // but the brute-force parallel search would take days to run.
    let mut common_multiple: Option<usize> = None;
    start.iter().for_each(|key| {
        let nsteps = walk_map(&route_map, &directions, key, &predicate);
        common_multiple = match common_multiple {
            Some(n) => Some(num::integer::lcm(n, nsteps)),
            None => Some(nsteps),
        }
    });
    if let Some(n) = common_multiple {
        n
    } else {
        panic!("No routes in map!");
    }
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
        let parts: [NodeName; 3] = line
            .split([' ', '=', ',', '(', ')'])
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    return None;
                };
                assert_eq!(s.len(), 3);
                let a: NodeName = s.chars().collect::<Vec<_>>().try_into().unwrap();
                Some(a)
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("line should have 3 parts");
        map.insert(parts[0], (parts[1], parts[2]));
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
        assert_eq!(2, part1(EXAMPLE_INPUT));
    }

    static EXAMPLE_2: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn GIVEN_aoc_example_input_WHEN_part2_run_THEN_expected_total_returned() {
        assert_eq!(6, part2(EXAMPLE_2));
    }
}
