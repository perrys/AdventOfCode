use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    println!("Part 1 answer is {}", part1(contents.as_str()));
    println!("Part 2 answer is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    let buffer = contents.as_bytes();
    const BUFFSIZE: usize = 4;

    let tester = |buf_idx: usize| -> bool {
        let mut bit_table: u64 = 0;
        for idx in 0..BUFFSIZE {
            let ch = buffer[buf_idx + idx];
            let mask = get_mask(ch);
            if (mask & bit_table) > 0 {
                return false;
            }
            bit_table |= mask;
        }
        true
    };
    for buf_idx in 0..buffer.len() - BUFFSIZE {
        if tester(buf_idx) {
            return buf_idx + BUFFSIZE;
        }
    }
    panic!("not found");
}

fn get_mask(ch: u8) -> u64 {
    let position = ch % 64;
    1 << position
}

fn part2(_contents: &str) -> usize {
    0
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test06 {
    use super::*;

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
