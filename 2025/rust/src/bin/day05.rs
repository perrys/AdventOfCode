fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    assert!(argv.len() == 2);
    let contents = std::fs::read_to_string(&argv[1]).expect("missing input data");
    println!("part1 score: {}", part1(&contents));
    println!("part2 score: {}", part2(&contents));
}

fn parse(contents: &str) -> (Vec<[u64; 2]>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut first_block = true;
    for line in contents.lines().map(str::trim) {
        if line.is_empty() {
            first_block = false;
            continue;
        }
        if first_block {
            let mut toks = line.split("-").map(|t| {
                let val: u64 = t.parse().expect("non-integer in range");
                val
            });
            let range = [
                toks.next().expect("lo range"),
                toks.next().expect("hi range"),
            ];
            assert!(range[0] <= range[1]);
            ranges.push(range);
        } else {
            ids.push(line.parse::<u64>().expect("non-integer id"));
        }
    }
    (ranges, ids)
}

fn part1(contents: &str) -> u64 {
    let (ranges, ids) = parse(contents);
    let mut p1_score = 0;
    'outer: for id in ids {
        for range in &ranges {
            if id >= range[0] && id <= range[1] {
                // println!("range: {range:?}, id: {id}");
                p1_score += 1;
                continue 'outer;
            }
        }
    }
    p1_score
}

fn part2(contents: &str) -> u64 {
    let (mut ranges, _) = parse(contents);
    let mut unique_ranges = Vec::new();
    'outer: while !ranges.is_empty() {
        let last = ranges.last().unwrap();
        for i in 0..(ranges.len() - 1) {
            if let Some(merged) = merge_ranges(last, &ranges[i]) {
                ranges[i] = merged;
                ranges.truncate(ranges.len() - 1);
                continue 'outer;
            }
        }
        unique_ranges.push(*last);
        ranges.truncate(ranges.len() - 1);
    }
    unique_ranges
        .into_iter()
        .map(|range| range[1] - range[0] + 1)
        .sum()
}

fn merge_ranges(lhs: &[u64; 2], rhs: &[u64; 2]) -> Option<[u64; 2]> {
    if (lhs[0] >= rhs[0] && lhs[0] <= rhs[1])
        || (lhs[1] >= rhs[0] && lhs[1] <= rhs[1])
        || (rhs[0] >= lhs[0] && rhs[0] <= lhs[1])
    {
        return Some([std::cmp::min(lhs[0], rhs[0]), std::cmp::max(lhs[1], rhs[1])]);
    }
    None
}

#[cfg(test)]
mod tester {
    use super::*;
    const TEST_DATA: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA), 3);
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge_ranges(&[1, 5], &[5, 10]), Some([1, 10]));
        assert_eq!(merge_ranges(&[1, 5], &[6, 10]), None);
        assert_eq!(merge_ranges(&[5, 9], &[2, 5]), Some([2, 9]));
        assert_eq!(merge_ranges(&[1, 7], &[5, 10]), Some([1, 10]));
        assert_eq!(merge_ranges(&[3, 9], &[2, 5]), Some([2, 9]));
        assert_eq!(merge_ranges(&[3, 9], &[4, 4]), Some([3, 9]));
        assert_eq!(merge_ranges(&[4, 5], &[3, 9]), Some([3, 9]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_DATA), 14);
    }
}
