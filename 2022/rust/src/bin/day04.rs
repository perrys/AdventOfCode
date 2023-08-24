use aoc2022::timer;
use arrayvec::ArrayVec;
use std::fs;

struct Range {
    low: u32,
    high: u32,
}

/// Split the line into two sets of ranges.
fn parse_line(line: &str, linenumber: usize) -> [Range; 2] {
    line.split(',')
        .map(|range_pair| {
            let [low, high] = range_pair
                .split('-')
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap_or_else(|_| panic!("unable to parse number at line {linenumber}"))
                })
                .collect::<ArrayVec<u32, 2>>()
                .into_inner()
                .unwrap_or_else(|_| panic!("not a range-pair at line {linenumber}"));
            Range { low, high }
        })
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap_or_else(|_| panic!("not two ranges at line {linenumber}"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
 
    let mut part1_result: u32 = 0;
    let part1 = || {
        let mut score: u32 = 0;
        for (linenumber, line) in contents.split('\n').enumerate() {
            if line.is_empty() {
                break;
            }
            let [first, second] = parse_line(line, linenumber);
            let contained = match first.low.cmp(&second.low) {
                std::cmp::Ordering::Less => first.high >= second.high,
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => first.high <= second.high,
            };
            if contained {
                score += 1;
            }
        }
        part1_result = score;
    };
    timer(part1);
    println!("Part 1 score is {part1_result}");

    let mut part2_result: u32 = 0;
    let part2 = || {
        let mut score: u32 = 0;
        for (linenumber, line) in contents.split('\n').enumerate() {
            if line.is_empty() {
                break;
            }
            let [first, second] = parse_line(line, linenumber);
            let overlapping = match first.low.cmp(&second.low) {
                std::cmp::Ordering::Less => first.high >= second.low,
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => first.low <= second.high,
            };
            if overlapping {
                score += 1;
            }
        }
        part2_result = score;
    };
    timer(part2);
    println!("Part 2 score is {part2_result}");
}
