use aoc2022::timer;
use std::fs;

fn get_priority(ch: u8) -> u8 {
    if ch >= b'a' {
        ch - b'a' + 1
    } else {
        ch - b'A' + 27
    }
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
            // the range of ascii from 'A' to 'z' is (65 to 122)
            // so presence can be recorded in a 64-length bitvector
            let mut mask: u64 = 0;
            let lineb = line.as_bytes();
            for idx in 0..lineb.len() / 2 {
                let ch = lineb[idx];
                let position = ch % 64;
                let flag = 1 << position;
                mask |= flag;
            }
            let mut done = false;
            for idx in line.len() / 2..line.len() {
                let ch = lineb[idx];
                let position = ch % 64;
                let flag = 1 << position;
                if mask & flag > 0 {
                    let pri = get_priority(ch);
                    score += pri as u32;
                    done = true;
                    break;
                }
            }
            if !done {
                panic!("duplicate not found on line {}", linenumber + 1);
            }
        }
        part1_result = score;
    };
    timer(part1);
    println!("Part 1 score is {part1_result}");
}
