//!
//! Advent of code challenge 2023 day 15.
//!
//! See <https://adventofcode.com/2023/day/15>
//!
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    contents.trim().split(',').map(|s| hash(s) as usize).sum()
}

fn part2(contents: &str) -> usize {
    let sequence = contents.trim().split(',').collect::<Vec<_>>();
    let mut boxes: Boxes = (0..256)
        .map(|_| Vec::<Lens>::new())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    for seq in sequence {
        if seq.contains('-') {
            let label = seq
                .split('-')
                .next()
                .unwrap_or_else(|| panic!("unable to parse {seq}"));
            let the_box = &mut boxes[hash(label) as usize];
            if let Some(idx) = the_box.iter().position(|l| l.label == label) {
                the_box.remove(idx);
            }
        } else {
            let mut tokens = seq.split('=');
            let label = tokens
                .next()
                .unwrap_or_else(|| panic!("unable to parse {seq}"));
            let f_len = tokens
                .next()
                .unwrap_or_else(|| panic!("unable to parse {seq}"))
                .parse::<u8>()
                .unwrap_or_else(|_| panic!("non-numeric focal len in {seq}"));
            let new_lens = Lens::new(label, f_len);
            let the_box = &mut boxes[hash(label) as usize];
            if let Some(idx) = the_box.iter().position(|l| l.label == label) {
                the_box[idx] = new_lens;
            } else {
                the_box.push(new_lens);
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, l)| focus_power(i, l))
        .sum()
}

fn focus_power(box_number: usize, the_box: &[Lens]) -> usize {
    (box_number + 1)
        * the_box
            .iter()
            .enumerate()
            .map(|(idx, lens)| (1 + idx) * lens.focal_len as usize)
            .sum::<usize>()
}

fn hash(s: &str) -> u8 {
    assert!(s.is_ascii());
    s.bytes()
        .fold(0, |accum, c| (((accum as u16 + c as u16) * 17) % 256) as u8)
}

type Boxes = [Vec<Lens>; 256];

#[derive(Debug)]
struct Lens {
    label: String,
    focal_len: u8,
}

impl Lens {
    fn new(label: &str, focal_len: u8) -> Self {
        let label = label.to_owned();
        Self { label, focal_len }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test15 {
    use super::*;

    #[test]
    fn GIVEN_known_strings_with_known_hashes_WHEN_hashing_THEN_matches_known_values() {
        let dotest = |s, h| assert_eq!(h, hash(s));
        dotest("HASH", 52);
    }

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(1320, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_examples_WHEN_calc_focus_power_run_THEN_matches_expected() {
        assert_eq!(5, focus_power(0, &[Lens::new("rn", 1), Lens::new("xx", 2)]));
        assert_eq!(
            140,
            focus_power(
                3,
                &[Lens::new("rn", 7), Lens::new("xx", 5), Lens::new("yy", 6)]
            )
        );
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(145, part2(EXAMPLE));
    }
}
