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
}

fn parse_lines<F1, F2>(lines: &[&str], color_tester: F1, mut totalizer: F2)
where
    F1: Fn(&str, u32) -> bool,
    F2: FnMut(bool, u32),
{
    for &line in lines {
        if let Some(remain) = line.strip_prefix("Game ") {
            let mut iter = remain.chars();
            let game_id: u32 = iter
                .by_ref()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .expect("unable to parse game ID");
            let _ = iter.by_ref().take_while(|c| !c.is_numeric());
            let groups = iter.as_str();
            let mut game_possible = true;
            'outer: for group in groups.split(';') {
                for cube_count in group.split(',') {
                    let mut toks = cube_count
                        .strip_prefix(' ')
                        .expect("not whitespace padded")
                        .split(' ');
                    let count = toks
                        .next()
                        .expect("couldn't tokenize block count \"{color}\"")
                        .parse::<u32>()
                        .expect("unable to parse cube count in \"{group}\"");
                    let cube_color = toks.next().expect("missing color token in group");
                    if !color_tester(cube_color, count) {
                        game_possible = false;
                        break 'outer;
                    }
                }
            }
            totalizer(game_possible, game_id);
        }
    }
}

fn part1(lines: &[&str]) {
    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)];
    let test_cube_count = |cube_color: &str, cube_count| {
        for &(color, max_n) in max_cubes.iter() {
            if cube_color == color {
                return cube_count <= max_n;
            }
        }
        panic!("unexpected color \"{cube_color}\"");
    };

    let mut total: u32 = 0;
    parse_lines(lines, test_cube_count, |game_possible, game_id| {
        if game_possible {
            total += game_id;
        }
    });
    println!("part1 total is {total}")
}
