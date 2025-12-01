use std::{env, fs};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file");
    let mut clock = 50;
    let mut p1_score = 0;
    let mut p2_score = 0;
    for line in contents.lines() {
        let ticks: i32 = line[1..].parse().expect("unable to parse number");
        let dir = line.chars().next().expect("empty line");
        let rotations = ticks / 100;
        let pve_ticks = match dir {
            'L' => 100 - (ticks % 100),
            'R' => ticks,
            _ => panic!("unknown direction"),
        };
        let old_clock = clock;
        clock += pve_ticks;
        clock %= 100;
        p2_score += rotations;
        if 0 == clock {
            p1_score += 1;
            p2_score += 1;
        } else if old_clock != 0 {
            match dir {
                'L' if clock > old_clock => p2_score += 1,
                'R' if clock < old_clock => p2_score += 1,
                _ => (),
            };
        }
    }
    println!("part1: {}", p1_score);
    println!("part2: {}", p2_score);
}
