use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");
    let contents: Vec<_> = contents.lines().collect();
    part1(&contents);
    //    part2(&contents);
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn is_adjacent_to_symbol(
    lines: &[&str],
    line_idx: usize,
    start_idx: usize,
    token_len: usize,
) -> bool {
    let line_len = lines[0].len(); // we know they are all the same length
    let start_col = start_idx - std::cmp::min(start_idx, 1);
    let end_col = std::cmp::min(start_idx + token_len, line_len - 1);
    if is_symbol(
        lines[line_idx]
            .chars()
            .nth(start_col)
            .expect("unable to get char at start col"),
    ) {
        return true;
    }
    if is_symbol(
        lines[line_idx]
            .chars()
            .nth(end_col)
            .expect("unable to get char at end col"),
    ) {
        return true;
    }
    if line_idx > 0
        && lines[line_idx - 1][start_col..=end_col]
            .chars()
            .any(is_symbol)
    {
        return true;
    }
    if (line_idx + 1) < lines.len()
        && lines[line_idx + 1][start_col..=end_col]
            .chars()
            .any(is_symbol)
    {
        return true;
    }
    false
}

fn part1(lines: &[&str]) {
    let mut total: usize = 0;
    let mut process_accumulator = |line_idx, col_idx, len, accum| {
        if is_adjacent_to_symbol(lines, line_idx, col_idx - len, len) {
            total += accum;
        }
    };
    for (line_idx, &line) in lines.iter().enumerate() {
        let mut accum: usize = 0;
        let mut len: usize = 0;
        for (col_idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                accum *= 10;
                accum += (c as u8 - b'0') as usize;
                len += 1;
            } else if accum > 0 {
                process_accumulator(line_idx, col_idx, len, accum);
                accum = 0;
                len = 0;
            }
        }
        if accum > 0 {
            process_accumulator(line_idx, line.len(), len, accum);
        }
    }
    println!("part1 total is {total}");
}
