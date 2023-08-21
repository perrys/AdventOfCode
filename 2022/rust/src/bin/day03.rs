use aoc2022::timer;
use std::fs;

fn get_priority(ch: u8) -> u8 {
    if ch >= b'a' {
        ch - b'a' + 1
    } else {
        ch - b'A' + 27
    }
}

fn get_mask(ch: u8) -> u64 {
    let position = ch % 64;
    1 << position
}

fn get_mask_for_slice<'a, I>(iter: I) -> u64
where
    I: std::iter::Iterator<Item = &'a u8>,
{
    let mut mask: u64 = 0;
    for ch in iter {
        mask |= get_mask(*ch);
    }
    mask
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
            let lineb = line.as_bytes();
            let mask: u64 = get_mask_for_slice(lineb.iter().take(lineb.len() / 2));
            let mut done = false;
            for ch in lineb.iter().take(lineb.len()).skip(line.len() / 2) {
                let position = ch % 64;
                let flag = 1 << position;
                if mask & flag > 0 {
                    let pri = get_priority(*ch);
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

    let mut part2_result: u32 = 0;
    let part2 = || {
        let mut lookup_table: [char; 64] = ['\0'; 64];
        for ch in 'A'..='z' {
            let m = ch as u8 & 63;
            lookup_table[m as usize] = ch;
        }
        let get_char = |m: u64| {
            let position = m.trailing_zeros() as usize;
            lookup_table[position]
        };

        let mut score: u32 = 0;
        let lines: Vec<&str> = contents.split('\n').collect();
        for idx in (0..lines.len()).step_by(3) {
            if lines[idx].is_empty() {
                break;
            }
            let chunk = [lines[idx], lines[idx + 1], lines[idx + 2]];
            let masks = chunk
                .iter()
                .map(|s| get_mask_for_slice(s.as_bytes().iter()));
            let mut mask: u64 = u64::MAX;
            for m in masks {
                mask &= m;
            }
            let ch = get_char(mask);
            let pri = get_priority(ch as u8) as u32;
            score += pri;
        }
        part2_result = score;
    };
    timer(part2);
    println!("Part 2 score is {part2_result}");
}
