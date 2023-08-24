use aoc2022::timer;
use arrayvec::ArrayVec;
use std::fs;

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
            let parts = line.split(',');
            let [first, second] = parts
                .map(|p| {
                    let pair = p.split('-');
                    pair.map(|num| {
                        num.parse::<u32>().unwrap_or_else(|_| {
                            panic!("unable to parse number at line {linenumber}")
                        })
                    })
                    .collect::<ArrayVec<u32, 2>>()
                    .into_inner()
                    .unwrap_or_else(|_| panic!("not a pair at line {linenumber}"))
                })
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap_or_else(|_| panic!("not two pairs at line {linenumber}"));

            let contained = match first[0].cmp(&second[0]) {
                std::cmp::Ordering::Less => first[1] >= second[1],
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => first[1] <= second[1],
            };
            if contained {
                score += 1;
            }
        }
        part1_result = score;
    };
    timer(part1);
    println!("Part 1 score is {part1_result}");
}
