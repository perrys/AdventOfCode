//!
//! Advent of code challenge 2023 day 22.
//!
//! See <https://adventofcode.com/2023/day/22>
//!
use std::{cell::Cell, collections::HashMap, fs, ops::RangeInclusive};

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
    let bricks = parse_file(contents);
    let maxx = bricks.iter().fold(usize::MIN, |a, b| *b.x.end().max(&a));
    let maxy = bricks.iter().fold(usize::MIN, |a, b| *b.y.end().max(&a));
    println!("{maxx}, {maxy}");
    // let map = map_z_values(&bricks);
    bricks
        .iter()
        .filter(|&brick| {
            let mut okay = true;
            // if brick.z.get().start() == brick.z.end() {
            //     'l1: for x in brick.x.clone() {
            //         for y in brick.y.clone() {
            //             if map[&(x, y)].last().expect("empty stack").0 > *brick.z.end() {
            //                 okay = false;
            //                 break 'l1;
            //             }
            //         }
            //     }
            // } else if map[&(*brick.x.end(), *brick.y.end())]
            //     .last()
            //     .expect("empty stack")
            //     .0
            //     > *brick.z.end()
            // {
            //     okay = false;
            // }
            okay
        })
        .count()
}
fn part2(_contents: &str) -> usize {
    0
}

fn parse_file(contents: &str) -> Vec<Brick> {
    let mut result = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Brick::new)
        .collect::<Vec<_>>();
    result.sort_by_key(|b| b.z_hi.get());
    result
}

#[derive(Debug, Eq, PartialEq)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z_len: usize,
    z_hi: Cell<usize>,
}

impl Brick {
    fn new(line: &str) -> Self {
        let mut parts = line.split('~');
        let first = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let second = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let parser = |part: &str| -> [usize; 3] {
            part.split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|_| panic!("parse error for {line}"))
                .try_into()
                .unwrap_or_else(|_| panic!("unexpected number of coords for {line}"))
        };
        let first = parser(first);
        let second = parser(second);
        if first[2] != second[2] && (first[1] != second[1] || first[0] != second[0]) {
            panic!("Brick is not perfectly horizontal or vertical for {line}");
        }
        let z = first[2].min(second[2])..=second[2].max(second[2]);
        Self {
            x: first[0].min(second[0])..=second[0].max(second[0]),
            y: first[1].min(second[1])..=second[1].max(second[1]),
            z_len: z.end() - z.start() + 1,
            z_hi: Cell::new(*z.end()),
        }
    }
}

fn drop_bricks(bricks: &mut [Brick]) -> HashMap<(usize, usize), Vec<(usize, &Brick)>> {
    let mut map: HashMap<(usize, usize), Vec<(usize, &Brick)>> = HashMap::new();
    for (idx, brick) in bricks.iter().enumerate() {
        let mut max_z = 0usize;
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                if let Some(stack) = &map.get(&(x, y)) {
                    let z = stack.last().unwrap().0;
                    max_z = max_z.max(z);
                }
            }
        }
        brick.z_hi.set(max_z + brick.z_len);
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let stack = map.entry((x, y)).or_default();
                stack.push((max_z + 1, brick));
            }
        }
    }
    map
}

// fn map_z_values<'a>(bricks: &'a [Brick]) -> HashMap<(usize, usize), Vec<(usize, &'a Brick)>> {
//     let mut map = HashMap::new();
//     for brick in bricks {
//         for x in brick.x.clone() {
//             for y in brick.y.clone() {
//                 for z in brick.z.clone() {
//                     let stack = map
//                         .entry((x, y))
//                         .or_insert_with(Vec::<(usize, &Brick)>::new);
//                     stack.push((z, brick));
//                 }
//             }
//         }
//     }
//     for stack in map.values_mut() {
//         stack.sort_by_key(|k| k.0);
//     }
//     map
// }

#[cfg(test)]
#[allow(non_snake_case)]
mod test22 {
    use super::*;

    // #[test]
    // fn GIVEN_small_brick_set_WHEN_mapping_z_values_THEN_correct_map_produced() {
    //     let dotest = |lines, expected: Vec<((usize, usize), Vec<(usize, usize)>)>| {
    //         let bricks = parse_file(lines);
    //         let make_entry = |(c, v): ((usize, usize), Vec<(usize, usize)>)| {
    //             let refvec = v
    //                 .iter()
    //                 .map(|(z, idx)| (*z, &bricks[*idx]))
    //                 .collect::<Vec<_>>();
    //             (c, refvec)
    //         };
    //         let exmap = HashMap::from_iter(expected.into_iter().map(make_entry));
    //         assert_eq!(exmap, map_z_values(&bricks));
    //     };
    //     // single stack
    //     dotest("1,1,1~1,1,2", vec![((1, 1), vec![(1, 0), (2, 0)])]);
    //     // stacked bricks, requires z-sorting
    //     dotest(
    //         "1,1,6~2,1,6\n1,1,1~1,1,2",
    //         vec![
    //             ((1, 1), vec![(1, 0), (2, 0), (6, 1)]),
    //             ((2, 1), vec![(6, 1)]),
    //         ],
    //     );
    // }

    #[test]
    #[ignore]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(5, part1(EXAMPLE));
    }

    static EXAMPLE: &str = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
}
