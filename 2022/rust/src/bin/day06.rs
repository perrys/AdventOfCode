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
    find_distinct_set(contents, 4)
}

fn part2(contents: &str) -> usize {
    find_distinct_set(contents, 14)
}

fn find_distinct_set(contents: &str, test_length: usize) -> usize {
    let buffer = contents.as_bytes();

    let tester = |buf_idx: usize| -> bool {
        let mut bit_table: u64 = 0;
        for idx in 0..test_length {
            let ch = buffer[buf_idx + idx];
            let mask = get_mask(ch);
            if (mask & bit_table) > 0 {
                return false;
            }
            bit_table |= mask;
        }
        true
    };
    for buf_idx in 0..buffer.len() - test_length {
        if tester(buf_idx) {
            return buf_idx + test_length;
        }
    }
    panic!("not found");
}

fn get_mask(ch: u8) -> u64 {
    let position = ch % 64;
    1 << position
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

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
