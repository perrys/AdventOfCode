use std::fs;

#[derive(PartialEq, Clone)]
enum Throw {
    Rock,
    Paper,
    Sciscors,
}

#[derive(Clone)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn get_required_throw(theirs: &Throw, outcome: &Result) -> Throw {
    match outcome {
        Result::Draw => theirs.clone(),
        Result::Lose => match theirs {
            Throw::Sciscors => Throw::Paper,
            Throw::Rock => Throw::Sciscors,
            Throw::Paper => Throw::Rock,
        },
        Result::Win => match theirs {
            Throw::Sciscors => Throw::Rock,
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Sciscors,
        },
    }
}

fn round_outcome(mine: &Throw, theirs: &Throw) -> Result {
    if mine == theirs {
        return Result::Draw;
    }
    match mine {
        Throw::Rock => {
            if let Throw::Sciscors = theirs {
                return Result::Win;
            }
        }
        Throw::Paper => {
            if let Throw::Rock = theirs {
                return Result::Win;
            }
        }
        Throw::Sciscors => {
            if let Throw::Paper = theirs {
                return Result::Win;
            }
        }
    }
    Result::Lose
}

fn normalize_to_result(outcome: u8, offset: u8) -> Result {
    match outcome - offset {
        0 => Result::Lose,
        1 => Result::Draw,
        2 => Result::Win,
        _ => panic!("Unexpected outcome symbol {outcome}"),
    }
}
fn normalize(throw: u8, offset: u8) -> Throw {
    match throw - offset {
        0 => Throw::Rock,
        1 => Throw::Paper,
        2 => Throw::Sciscors,
        _ => panic!("Unexpected throw symbol {throw}"),
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
    part1(&contents);
    part2(&contents);
}

fn part2(contents: &str) {
    let mut score: u32 = 0;
    for line in contents.split("\n") {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let theirs = normalize(tokens[0].as_bytes()[0], 'A' as u8);
        let outcome = normalize_to_result(tokens[1].as_bytes()[0], 'X' as u8);
        match outcome {
            Result::Win => score += 6,
            Result::Draw => score += 3,
            Result::Lose => {}
        }
        let mine = get_required_throw(&theirs, &outcome);
        match mine {
            Throw::Rock => score += 1,
            Throw::Paper => score += 2,
            Throw::Sciscors => score += 3,
        }
    }
    println!("Part 2 score is {score}");
}

fn part1(contents: &str) {
    let mut score: u32 = 0;
    for line in contents.split("\n") {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let theirs = normalize(tokens[0].as_bytes()[0], 'A' as u8);
        let mine = normalize(tokens[1].as_bytes()[0], 'X' as u8);
        match round_outcome(&mine, &theirs) {
            Result::Win => score += 6,
            Result::Draw => score += 3,
            Result::Lose => {}
        }
        match mine {
            Throw::Rock => score += 1,
            Throw::Paper => score += 2,
            Throw::Sciscors => score += 3,
        }
    }
    println!("Part 1 score is {score}");
}
