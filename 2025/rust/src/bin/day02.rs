use itertools::Itertools;
use std::{env, fs};

struct NumRange {
    lo: u64,
    hi: u64,
    lo_digits: Vec<char>,
    hi_digits: Vec<char>,
}

impl NumRange {
    fn parse(tok: &str) -> Self {
        let mut toks = tok.split('-');
        let first = toks.next().expect("first element missing");
        let second = toks.next().expect("second element missing");
        let lo = first.parse().expect("non integer");
        let hi = second.parse().expect("non integer");
        assert!(lo < hi);
        Self {
            lo,
            hi,
            lo_digits: first.chars().collect(),
            hi_digits: second.chars().collect(),
        }
    }

    fn collect_prefix(&self) -> Vec<char> {
        let mut result = Vec::<char>::new();
        if self.lo_digits.len() == self.hi_digits.len() {
            for (l, r) in self.lo_digits.iter().zip(self.hi_digits.iter()) {
                if l == r {
                    result.push(*l);
                } else {
                    break;
                }
            }
        }
        result
    }
}

fn factors(n: usize) -> Vec<usize> {
    let mut result = vec![n];
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n.is_multiple_of(i) {
            result.push(i);
            if i != 1 && i * i != n {
                result.push(n / i);
            }
        }
    }
    result.sort();
    result
}

fn process_digits(ndigits: usize, divisor: usize, prefix: &[char], result: &mut Vec<Vec<char>>) {
    //println!("nd: {}, div: {}", ndigits, divisor);
    assert!(ndigits.is_multiple_of(divisor));
    let pattern_length = ndigits / divisor;
    if prefix.len() < pattern_length {
        let range = if prefix.is_empty() {
            '1'..='9'
        } else {
            '0'..='9'
        };
        let mut new_prefix = Vec::with_capacity(prefix.len() + 1);
        new_prefix.extend_from_slice(prefix);
        new_prefix.push('0');
        for i in range {
            *new_prefix.last_mut().unwrap() = i;
            process_digits(ndigits, divisor, &new_prefix, result);
        }
    } else if prefix.len() == pattern_length {
        let mut v = Vec::<_>::with_capacity(ndigits);
        for _i in 0..divisor {
            v.extend_from_slice(prefix);
        }
        result.push(v);
    } else {
        // prefix is more than pattern_length
        let mut v = Vec::<_>::with_capacity(ndigits);
        v.extend_from_slice(prefix);
        v.resize(ndigits, ' ');
        for i in 0..pattern_length {
            for j in 1..divisor {
                let idx = i + pattern_length * j;
                if idx < prefix.len() {
                    if prefix[idx] != prefix[i] {
                        return;
                    }
                } else {
                    v[idx] = v[i];
                }
            }
        }
        result.push(v);
    }
}

fn p1_process(range: &NumRange) -> Vec<u64> {
    let common_prefix: Vec<char> = range.collect_prefix();
    let mut candidates = Vec::<_>::new();
    let mut ndigits = range.lo_digits.len();
    while ndigits <= range.hi_digits.len() {
        if ndigits.is_multiple_of(2) {
            process_digits(ndigits, 2, &common_prefix, &mut candidates);
        }
        ndigits += 1;
    }
    candidates
        .into_iter()
        .filter_map(|v| {
            let val: u64 = v.iter().collect::<String>().parse().unwrap();
            if val >= range.lo && val <= range.hi {
                Some(val)
            } else {
                None
            }
        })
        .collect()
}

fn p2_process(range: &NumRange) -> Vec<u64> {
    let common_prefix: Vec<char> = range.collect_prefix();
    let mut candidates = Vec::<_>::new();
    let mut ndigits = range.lo_digits.len();
    while ndigits <= range.hi_digits.len() {
        for i in factors(ndigits) {
            if i > 1 {
                process_digits(ndigits, i, &common_prefix, &mut candidates);
            }
        }
        ndigits += 1;
    }
    candidates
        .into_iter()
        .filter_map(|v| {
            let val: u64 = v.iter().collect::<String>().parse().unwrap();
            if val >= range.lo && val <= range.hi {
                Some(val)
            } else {
                None
            }
        })
        .unique()
        .collect()
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file");
    let ranges: Vec<NumRange> = contents.split(',').map(NumRange::parse).collect();
    let mut p1_score = 0;
    let mut p2_score = 0;
    for range in ranges {
        // println!("{}-{}", range.lo, range.hi);
        p1_score += p1_process(&range).iter().sum::<u64>();
        p2_score += p2_process(&range).iter().sum::<u64>();
    }
    println!("part1: {}", p1_score);
    println!("part2: {}", p2_score);
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn prefix_test() {
        let r = NumRange::parse("123456-123567");
        assert_eq!(r.collect_prefix(), "123".chars().collect::<Vec<_>>());
    }

    #[test]
    fn p1_process_test() {
        assert_eq!(p1_process(&NumRange::parse("11-22")), vec![11, 22]);
        assert_eq!(p1_process(&NumRange::parse("99-115")), vec![99]);
        assert_eq!(p1_process(&NumRange::parse("998-1012")), vec![1010]);
        assert_eq!(
            p1_process(&NumRange::parse("1188511880-1188511890")),
            vec![1188511885]
        );
        assert_eq!(p1_process(&NumRange::parse("222220-222224")), vec![222222]);
        assert_eq!(p1_process(&NumRange::parse("1698522-1698528")), vec![]);
        assert_eq!(p1_process(&NumRange::parse("446443-446449")), vec![446446]);
        assert_eq!(
            p1_process(&NumRange::parse("38593856-38593862")),
            vec![38593859]
        );
    }

    #[test]
    fn p2_process_test() {
        assert_eq!(
            p2_process(&NumRange::parse("2727216511-2727316897")),
            vec![2727227272, 2727272727]
        );
        assert_eq!(p2_process(&NumRange::parse("1-22")), vec![11, 22]);
        assert_eq!(p2_process(&NumRange::parse("11-22")), vec![11, 22]);
        assert_eq!(p2_process(&NumRange::parse("99-115")), vec![99, 111]);
        assert_eq!(p2_process(&NumRange::parse("998-1012")), vec![999, 1010]);
        assert_eq!(
            p2_process(&NumRange::parse("1188511880-1188511890")),
            vec![1188511885]
        );
        assert_eq!(p2_process(&NumRange::parse("222220-222224")), vec![222222]);
        assert_eq!(p2_process(&NumRange::parse("1698522-1698528")), vec![]);
        assert_eq!(p2_process(&NumRange::parse("446443-446449")), vec![446446]);
        assert_eq!(
            p2_process(&NumRange::parse("38593856-38593862")),
            vec![38593859]
        );
        assert_eq!(p2_process(&NumRange::parse("565653-565659")), vec![565656]);
        assert_eq!(
            p2_process(&NumRange::parse("824824821-824824827")),
            vec![824824824]
        );
        assert_eq!(
            p2_process(&NumRange::parse("2121212118-2121212124")),
            vec![2121212121]
        );
    }

    #[test]
    fn factors_test() {
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(2), vec![2]);
        assert_eq!(factors(3), vec![3]);
        assert_eq!(factors(4), vec![2, 4]);
        assert_eq!(factors(5), vec![5]);
        assert_eq!(factors(6), vec![2, 3, 6]);
        assert_eq!(factors(7), vec![7]);
        assert_eq!(factors(8), vec![2, 4, 8]);
        assert_eq!(factors(9), vec![3, 9]);
        assert_eq!(factors(10), vec![2, 5, 10]);
    }
}
