use std::time::Instant;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("USAGE: {} <filename>", args[0]);
    }
    let contents = read_file(&args[1]);
    let mut answer: u32 = 0;
    const N_ITERS: u32 = 10000;
    let mut sum: u128 = 0;
    let mut sumsq: u128 = 0;
    let mut opt_min_dur: Option<u128> = None;
    for _ in 0..N_ITERS {
        let now = Instant::now();
        answer = find_largest(&contents);
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
    println!("The largest is {answer}");
    let avg = sum as f64 / (N_ITERS as f64);
    let stddev = ((sumsq as f64 / (N_ITERS as f64)) - (avg * avg)).sqrt();
    let min_time = opt_min_dur.unwrap();
    println!("Min time was {min_time:}ns, avg = {avg:.0}+/-{stddev:.0}");
}

fn find_largest(contents: &str) -> u32 {
    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for line in contents.split("\n") {
        if line.len() > 0 {
            let val = line.parse::<u32>().unwrap();
            sum += val;
        } else {
            if 0 == max || sum > max {
                max = sum;
            }
            sum = 0;
        }
    }
    return max;
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("unable to open file")
}
