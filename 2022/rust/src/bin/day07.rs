use std::cell::RefCell;
use std::{fs, rc::Rc};

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
    static THRESHOLD: usize = 100000;
    let mut parser = Reader { cwd: Vec::new() };
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        parser.parse_line(line);
    }

    let mut answer: usize = 0;
    fn walk(node: &DirNode, answer: &mut usize) -> usize {
        let mut total: usize = 0;
        for subdir in node.borrow().subdirs.iter() {
            total += walk(subdir, answer);
        }
        for file in node.borrow().files.iter() {
            total += file.1;
        }
        if total <= THRESHOLD {
            *answer += total;
        }
        total
    }
    walk(&parser.cwd[0], &mut answer);
    answer
}

fn part2(_contents: &str) -> usize {
    0
}

type DirNode = Rc<RefCell<Directory>>;

struct Directory {
    #[allow(dead_code)]
    name: String,
    files: Vec<(String, usize)>,
    subdirs: Vec<DirNode>,
}

impl Directory {
    fn new(name: &str) -> DirNode {
        Rc::new(RefCell::new(Self {
            name: name.to_owned(),
            files: Vec::new(),
            subdirs: Vec::new(),
        }))
    }
}

struct Reader {
    cwd: Vec<DirNode>,
}

impl Reader {
    fn parse_line(&mut self, line: &str) {
        let toks = line
            .split(' ')
            .filter(|&s| !s.is_empty())
            .collect::<Vec<_>>();
        match toks[0] {
            "$" => self.parse_command(&toks[1..]),
            "dir" => (),
            size_str => {
                let filename = toks[1];
                let size = size_str.parse::<usize>();
                let working_dir = self.cwd.last().expect("working directory unset");
                let mut working_dir = working_dir.borrow_mut();
                working_dir.files.push((
                    filename.to_string(),
                    size.expect("unable to parse file size"),
                ));
            }
        }
    }

    fn parse_command(&mut self, toks: &[&str]) {
        match toks[0] {
            "cd" => match toks[1] {
                "/" => {
                    let rootdir = Directory::new("<root>");
                    self.cwd.push(rootdir);
                }
                ".." => {
                    self.cwd.pop();
                }
                subdir_name => {
                    let subdir = Directory::new(subdir_name);
                    self.cwd.push(subdir.clone());
                    let len = self.cwd.len();
                    let working_dir = &self.cwd[len - 2];
                    let mut working_dir = working_dir.borrow_mut();
                    working_dir.subdirs.push(subdir);
                }
            },
            "ls" => (),
            other_cmd => {
                panic!("unrecognized command: \"{other_cmd}\"");
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tester {
    use super::*;

    #[test]
    fn GIVEN_initial_commands_WHEN_reader_parsing_THEN_state_changes_as_expected() {
        let mut parser = Reader { cwd: Vec::new() };
        assert_eq!(0, parser.cwd.len());
        parser.parse_line("$ cd /");
        assert_eq!(1, parser.cwd.len());
        {
            let dir = parser.cwd.last().unwrap();
            assert_eq!("<root>", dir.borrow().name);
            assert_eq!(0, dir.borrow().files.len());
        }
        parser.parse_line("126880 fmftdzrp.fwt");
        {
            let dir = parser.cwd.last().unwrap();
            assert_eq!(1, dir.borrow().files.len()); //
            assert_eq!(126880, dir.borrow().files[0].1);
            assert_eq!("fmftdzrp.fwt", dir.borrow().files[0].0);
        }
        parser.parse_line("$ cd a");
        parser.parse_line("29116 f");
        assert_eq!(2, parser.cwd.len());
        {
            let root = parser.cwd.first().unwrap();
            assert_eq!(1, root.borrow().subdirs.len());
            let dir = parser.cwd.last().unwrap();
            assert_eq!("a", dir.borrow().name);
            assert_eq!(1, dir.borrow().files.len()); //
            assert_eq!(29116, dir.borrow().files[0].1);
            assert_eq!("f", dir.borrow().files[0].0);
        }
    }

    static EXAMPLE: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(95437, part1(EXAMPLE));
    }
}
