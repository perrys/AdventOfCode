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
    part2(&contents);
}

fn parse_line_groups<F1>(groups: &str, color_tester: &mut F1) -> bool
where
    F1: FnMut(&str, u32) -> bool,
{
    for group in groups.split(';') {
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
                return false;
            }
        }
    }
    true
}

fn parse_lines<F1>(lines: &[&str], mut line_processor: F1)
where
    F1: FnMut(u32, &str),
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
            let _ = iter.by_ref().take_while(|c| !c.is_numeric()); // eat whitespace
            line_processor(game_id, iter.as_str());
        }
    }
}

fn part1(lines: &[&str]) {
    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)];
    let mut test_cube_count = |cube_color: &str, cube_count| {
        for &(color, max_n) in max_cubes.iter() {
            if cube_color == color {
                return cube_count <= max_n;
            }
        }
        panic!("unexpected color \"{cube_color}\"");
    };

    let mut total: u32 = 0;
    parse_lines(lines, |game_id, groups: &str| {
        if parse_line_groups(groups, &mut test_cube_count) {
            total += game_id;
        }
    });
    println!("part1 total is {total}")
}

struct Cube {
    pub color: &'static str,
    pub max_count: u32,
}

impl Cube {
    fn new(color: &'static str) -> Self {
        Self {
            color,
            max_count: 0,
        }
    }
    fn record_max(&mut self, count: u32) {
        self.max_count = std::cmp::max(self.max_count, count);
    }
}

fn part2(lines: &[&str]) {
    let mut total: u32 = 0;
    let line_parser = |_, groups: &str| {
        let mut cube_colors = [Cube::new("red"), Cube::new("green"), Cube::new("blue")];
        let mut cube_max_recorder = |cube_color: &str, cube_count: u32| {
            for cube in cube_colors.iter_mut() {
                if cube_color == cube.color {
                    cube.record_max(cube_count);
                    return true;
                }
            }
            panic!("unexpected color \"{cube_color}\"");
        };
        parse_line_groups(groups, &mut cube_max_recorder);
        let power = cube_colors
            .iter()
            .fold(1, |total, cube| cube.max_count * total);
        total += power;
    };
    parse_lines(lines, line_parser);
    println!("part2 total is {total}")
}
