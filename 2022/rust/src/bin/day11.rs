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

fn part1(_contents: &str) -> usize {
    let monkeys = parse(_contents);
    run_simulation(monkeys, 20, 3)
}

fn part2(_contents: &str) -> usize {
    let monkeys = parse(_contents);
    run_simulation(monkeys, 10000, 1)
}

fn parse(contents: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    let mut group = Vec::<&str>::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            if group.len() != 6 {
                panic!("Monkey group is not 6 lines at {idx}");
            }
            let lines: [&str; 5] = group[1..6].try_into().unwrap();

            let monkey = Monkey::new(&lines);
            monkeys.push(monkey);
            group.clear();
        } else {
            group.push(line);
        }
    }
    monkeys
}

fn run_simulation(mut monkeys: Vec<Monkey>, n_rounds: usize, divisor: usize) -> usize {
    let common_multiple = monkeys.iter().map(|m| m.test_divisor).product();
    execute_rounds(n_rounds, &mut monkeys, divisor, common_multiple);
    monkeys.sort_by_key(|monkey| monkey.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product()
}

fn execute_rounds(n_rounds: usize, monkeys: &mut Vec<Monkey>, divisor: usize, subtractor: usize) {
    for _round in 0..n_rounds {
        for idx in 0..monkeys.len() {
            while let Some(result) = monkeys[idx].inspect_item(divisor, subtractor) {
                monkeys[result.destination].items.push(result.item);
            }
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    inspections: usize,
    test_divisor: usize,
}

#[derive(Debug, PartialEq)]
struct OpResult {
    item: usize,
    destination: usize,
}

impl Monkey {
    fn new(lines: &[&str; 5]) -> Self {
        let items = lines[0]
            .split(": ")
            .nth(1)
            .expect("couldn't tokenize items line")
            .split(", ")
            .map(|tok| tok.parse::<usize>().expect("non-integer item"))
            .collect::<Vec<_>>();
        let mut tokens = lines[1]
            .split(" = old ")
            .nth(1)
            .expect("couldn't tokenize operation line")
            .split(' ');
        let op = tokens.next().expect("no operator found").to_owned();
        let arg = tokens.next().expect("no argument found").to_owned();
        let operation = move |old: usize| {
            let arg = match arg.as_str() {
                "old" => old,
                n => n.parse::<usize>().expect("non-integer argument"),
            };
            match op.as_str() {
                "+" => old + arg,
                "-" => old - arg,
                "*" => old * arg,
                "/" => old / arg,
                _ => panic!("unknown operator {op}"),
            }
        };
        let test_divisor = lines[2]
            .split("divisible by ")
            .nth(1)
            .expect("unable to parse divisor line")
            .parse::<usize>()
            .expect("non-integer divisor");
        let true_dest = lines[3]
            .split("throw to monkey ")
            .nth(1)
            .expect("unable to parse true dest line")
            .parse::<usize>()
            .expect("non-integer destination");
        let false_dest = lines[4]
            .split("throw to monkey ")
            .nth(1)
            .expect("unable to parse false dest line")
            .parse::<usize>()
            .expect("non-integer destination");
        let test = move |n| match n % test_divisor {
            0 => true_dest,
            _ => false_dest,
        };
        Self {
            items,
            operation: Box::new(operation),
            test: Box::new(test),
            inspections: 0,
            test_divisor,
        }
    }

    fn inspect_item(&mut self, divisor: usize, common_multiple: usize) -> Option<OpResult> {
        if self.items.is_empty() {
            None
        } else {
            self.inspections += 1;
            let mut item = self.items.remove(0);
            item = (self.operation)(item);
            item /= divisor;
            let num = item / common_multiple;
            item -= num * common_multiple;
            let destination = (self.test)(item);
            Some(OpResult { item, destination })
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::Monkey;

    use super::*;

    static EXAMPLE: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

"#;

    #[test]
    fn GIVEN_sample_lines_when_parsing_THEN_monkey_produced() {
        let lines: [&str; 5] = EXAMPLE
            .lines()
            .skip(1)
            .take(5)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let monkey = Monkey::new(&lines);
        assert_eq!(vec![79, 98], monkey.items);
        assert_eq!(19, (monkey.operation)(1));
        assert_eq!(2, (monkey.test)(23));
        assert_eq!(3, (monkey.test)(24));
    }

    #[test]
    fn GIVEN_sample_lines_when_operating_on_items_THEN_correct_item_and_dest_returned() {
        let lines: [&str; 5] = EXAMPLE
            .lines()
            .skip(1)
            .take(5)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut monkey = Monkey::new(&lines);
        assert_eq!(
            OpResult {
                item: 17,
                destination: 3
            },
            monkey.inspect_item(3, monkey.test_divisor).unwrap()
        );
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_1_THEN_expected_answers_returned() {
        assert_eq!(10605, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_running_part_2_THEN_expected_answers_returned() {
        assert_eq!(2713310158, part2(EXAMPLE));
    }
}
