enum Operation {
    Add,
    Mult,
}

impl Operation {
    fn parse(tok: char) -> Self {
        match tok {
            '+' => Self::Add,
            '*' => Self::Mult,
            _ => panic!("unrecognized operation \"{tok}\""),
        }
    }
}

struct MathProblem {
    numbers: Vec<i64>,
    op: Option<Operation>,
}

impl MathProblem {
    fn answer(&self) -> i64 {
        if let Some(op) = &self.op {
            match op {
                Operation::Add => self.numbers.iter().sum(),
                Operation::Mult => self.numbers.iter().fold(1, |a, b| a * *b),
            }
        } else {
            panic!("operation on incomplete problem");
        }
    }
}

fn get_tokens_list(contents: &str) -> Vec<Vec<&str>> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(' ')
                .filter_map(|tok| {
                    let t = tok.trim();
                    if t.is_empty() {
                        return None;
                    }
                    Some(t)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(tokens_list: &[Vec<&str>]) -> i64 {
    let line_count = tokens_list.len();
    let mut problems = Vec::<MathProblem>::new();
    for (i, tokens) in tokens_list.iter().enumerate() {
        if i < line_count - 1 {
            for (j, number) in tokens
                .iter()
                .map(|t| t.parse::<i64>().expect("non-integer"))
                .enumerate()
            {
                if i > 0 {
                    problems[j].numbers.push(number);
                } else {
                    let mut numbers = Vec::<i64>::with_capacity(line_count - 1);
                    numbers.push(number);
                    problems.push(MathProblem { numbers, op: None });
                }
            }
        } else {
            for (j, op) in tokens
                .iter()
                .map(|&s| Operation::parse(s.chars().next().unwrap()))
                .enumerate()
            {
                problems[j].op = Some(op);
            }
        }
    }
    problems.iter().map(MathProblem::answer).sum()
}

fn part2(contents: &str) -> i64 {
    let mut lines = contents
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut op_idx_list = Vec::new();
    for (idx, ch) in lines.last().expect("not enough lines").iter().enumerate() {
        if *ch != ' ' {
            op_idx_list.push((Operation::parse(*ch), idx));
        }
    }
    lines.truncate(lines.len() - 1);
    let mut score = 0;
    for (op, idx) in op_idx_list {
        let mut digit_list = Vec::new();
        let mut col_idx = idx;
        'outer: loop {
            let mut blank = true;
            for line in lines.iter() {
                if line.len() > col_idx && line[col_idx] != ' ' {
                    blank = false;
                    break;
                }
            }
            if !blank {
                let mut col_digits = Vec::<char>::with_capacity(lines.len());
                for line in lines.iter() {
                    col_digits.push(line[col_idx]);
                }
                digit_list.push(col_digits);
                col_idx += 1;
            } else {
                break 'outer;
            }
        }
        let mp = MathProblem {
            numbers: digit_list.into_iter().map(|col| to_int(&col)).collect(),
            op: Some(op),
        };
        score += mp.answer();
    }
    score
}

fn to_int(chars: &[char]) -> i64 {
    let mut result = 0;
    let mut mult = 1;
    let zero = '0' as i64;
    for c in chars.iter().rev() {
        if *c == ' ' {
            continue;
        }
        result += (*c as i64 - zero) * mult;
        mult *= 10;
    }
    result
}

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }

    let contents = std::fs::read_to_string(&argv[1]).expect("non existent filename");
    let tokens_list = get_tokens_list(&contents);

    println!("part1: {}", part1(&tokens_list));
    println!("part2: {}", part2(&contents));
}
