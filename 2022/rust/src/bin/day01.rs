use aoc2022::timer;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("USAGE: {} <filename>", args[0]);
    }
    let contents = read_file(&args[1]);
    let mut largest: u32 = 0;
    let part1 = || {
        let mut answer: u32 = 0;
        let mut max_cb = |result| {
            if result > answer {
                answer = result;
            }
        };
        parse_file(&contents, &mut max_cb);
        largest = answer;
    };
    timer(part1);
    println!("Part1 answer is {largest}");
    let mut largest3: [u32; 3] = [0; 3];
    let part2 = || {
        let mut top3: [u32; 3] = [0; 3];
        let mut t3 = |result| {
            if result <= top3[0] {
                return;
            }
            top3[0] = result;
            if result <= top3[1] {
                return;
            }
            top3[0] = top3[1];
            top3[1] = result;
            if result <= top3[2] {
                return;
            }
            top3[1] = top3[2];
            top3[2] = result;
        };
        parse_file(&contents, &mut t3);
        largest3 = top3;
    };
    timer(part2);
    let sum3: u32 = largest3.iter().sum();
    println!("Part2 answer is {sum3}");
}

fn parse_file<F: FnMut(u32)>(contents: &str, mut cb: F) {
    let mut sum: u32 = 0;
    for line in contents.split('\n') {
        if !line.is_empty() {
            let val = line.parse::<u32>().unwrap();
            sum += val;
        } else {
            cb(sum);
            sum = 0;
        }
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("unable to open file")
}
