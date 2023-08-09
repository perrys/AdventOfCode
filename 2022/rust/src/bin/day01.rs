use std::time::Instant;
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

fn timer<F: FnMut()>(mut func: F) {
    const N_ITERS: u32 = 10000;
    let mut sum: u128 = 0;
    let mut sumsq: u128 = 0;
    let mut opt_min_dur: Option<u128> = None;
    for _ in 0..N_ITERS {
        let now = Instant::now();
        func();
        let duration = now.elapsed().as_nanos();
        sum += duration;
        sumsq += duration * duration;
        match opt_min_dur {
            Some(ref mut min_dur) => {
                if duration < *min_dur {
                    *min_dur = duration;
                }
            }
            None => opt_min_dur = Some(duration),
        }
    }
    let avg = sum as f64 / (N_ITERS as f64);
    let stddev = ((sumsq as f64 / (N_ITERS as f64)) - (avg * avg)).sqrt();
    let min_time = opt_min_dur.unwrap();
    println!("Elapsed time - min: {min_time:}ns, avg: {avg:.0}, stddev: {stddev:.0}");
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
