fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
}

type Pair = (usize, usize);

fn parse_line(line: &str) -> Pair {
    let mut iter = line.split(',');
    let result = (
        iter.next()
            .expect("val 1")
            .parse::<usize>()
            .expect("non-int 1"),
        iter.next()
            .expect("val 2")
            .parse::<usize>()
            .expect("non-int 2"),
    );
    assert!(iter.next().is_none());
    result
}

fn area(p1: Pair, p2: Pair) -> usize {
    (1 + p1.0.abs_diff(p2.0)) * (1 + p1.1.abs_diff(p2.1))
}

fn part1(contents: &str) -> usize {
    let pairs = contents.lines().map(parse_line).collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..pairs.len() {
        for j in (i + 1)..pairs.len() {
            max = max.max(area(pairs[i], pairs[j]));
        }
    }
    max
}
