enum Operation {
    Add,
    Mult,
}

impl Operation {
    fn parse(tok: &str) -> Self {
        match tok {
            "+" => Self::Add,
            "*" => Self::Mult,
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

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }

    let contents = std::fs::read_to_string(&argv[1]).expect("non existent filename");
    let tokens_list = contents
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
        });
    let line_count = tokens_list.clone().count();
    let mut problems = Vec::<MathProblem>::new();
    for (i, tokens) in tokens_list.enumerate() {
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
            for (j, op) in tokens.iter().map(|&s| Operation::parse(s)).enumerate() {
                problems[j].op = Some(op);
            }
        }
    }
    let p1: i64 = problems.iter().map(MathProblem::answer).sum();
    println!("part1: {p1}");
}
