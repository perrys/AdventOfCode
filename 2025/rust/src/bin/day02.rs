use std::{env, fs};

struct Range {
    lo: u64,
    hi: u64,
    lo_digits: Vec<char>,
    hi_digits: Vec<char>,
}

impl Range {
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

fn process_digits(ndigits: usize, prefix: &[char], result: &mut Vec<Vec<char>>) {
    if 1 == (ndigits & 1) {
        return;
    }
    let half = ndigits / 2;
    if prefix.len() < half {
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
            process_digits(ndigits, &new_prefix, result);
        }
    } else if prefix.len() == half {
        let mut v = Vec::<_>::with_capacity(ndigits);
        v.extend_from_slice(prefix);
        v.extend_from_slice(prefix);
        result.push(v);
    } else {
        // prefix is more than half
        let mut v = Vec::<_>::with_capacity(ndigits);
        v.extend_from_slice(prefix);
        for i in 0..half {
            let idx = i + half;
            if idx < prefix.len() {
                if prefix[idx] != prefix[i] {
                    return;
                }
            } else {
                v.push(v[i]);
            }
        }
        result.push(v);
    }
}

fn process_range(range: &Range) -> Vec<u64> {
    let common_prefix: Vec<char> = range.collect_prefix();
    let mut candidates = Vec::<_>::new();
    let mut ndigits = range.lo_digits.len();
    while ndigits <= range.hi_digits.len() {
        process_digits(ndigits, &common_prefix, &mut candidates);
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

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = fs::read_to_string(&argv[1]).expect("unable to read file");
    let ranges: Vec<Range> = contents.split(',').map(Range::parse).collect();
    let mut p1_score = 0;
    let p2_score: i32 = 0;
    for range in ranges {
        p1_score += process_range(&range).iter().sum::<u64>();
    }
    println!("part1: {}", p1_score);
    println!("part2: {}", p2_score);
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn prefix_test() {
        let r = Range::parse("123456-123567");
        assert_eq!(r.collect_prefix(), "123".chars().collect::<Vec<_>>());
    }

    #[test]
    fn p1_process_test() {
        assert_eq!(process_range(&Range::parse("11-22")), vec![11, 22]);
        assert_eq!(process_range(&Range::parse("99-115")), vec![99]);
        assert_eq!(process_range(&Range::parse("998-1012")), vec![1010]);
        assert_eq!(
            process_range(&Range::parse("1188511880-1188511890")),
            vec![1188511885]
        );
        assert_eq!(process_range(&Range::parse("222220-222224")), vec![222222]);
        assert_eq!(process_range(&Range::parse("1698522-1698528")), vec![]);
        assert_eq!(process_range(&Range::parse("446443-446449")), vec![446446]);
        assert_eq!(
            process_range(&Range::parse("38593856-38593862")),
            vec![38593859]
        );
    }
}
