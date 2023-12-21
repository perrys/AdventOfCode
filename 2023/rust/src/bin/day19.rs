//!
//! Advent of code challenge 2023 day 19.
//!
//! See <https://adventofcode.com/2023/day/19>
//!
use std::{collections::HashMap, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> i32 {
    let workflows = contents
        .lines()
        .filter(|l| !l.trim().is_empty() && !(l.chars().nth(0).unwrap() == '{'))
        .map(Workflow::new)
        .collect::<Vec<_>>();
    let workflows = HashMap::from_iter(workflows.into_iter().map(|wf| (wf.id.clone(), wf)));
    let parts = contents
        .lines()
        .filter(|l| !l.trim().is_empty() && (l.chars().nth(0).unwrap() == '{'))
        .map(Part::new)
        .collect::<Vec<_>>();
    parts.iter().map(|part| process(part, &workflows)).sum()
}

fn part2(_contents: &str) -> usize {
    0
}

#[derive(Debug, Eq, PartialEq)]
struct Predicate {
    attribute: char,
    value: i32,
    is_less_than: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    ToWorkflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    predicate: Option<Predicate>,
    action: Action,
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Eq, PartialEq)]
struct Part(HashMap<char, i32>);

impl Part {
    fn new(line: &str) -> Self {
        assert_eq!(Some('{'), line.chars().nth(0));
        assert_eq!(Some('}'), line.chars().last());
        let mut attrs = HashMap::new();
        line[1..line.len() - 1].split(',').for_each(|s| {
            let mut kv = s.split('=');
            let key = kv
                .next()
                .unwrap_or_else(|| panic!("couldn't parse key in {s}"))
                .chars()
                .nth(0)
                .unwrap();

            let val = kv
                .next()
                .unwrap_or_else(|| panic!("couldn't get value in {s}"))
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("couldn't parse value in {s}"));
            attrs.insert(key.to_owned(), val);
        });
        Self(attrs)
    }
}

impl Rule {
    fn new(s: &str) -> Self {
        let mut predicate = None;
        let mut action = s;
        if s.contains(':') {
            let parts = s.split(':').collect::<Vec<_>>();
            if parts.len() != 2 {
                panic!("unable to parse rule {s}");
            }
            let pred_str = parts[0];
            action = parts[1];
            let mut chars = pred_str.chars();
            let attribute = chars.next().expect("no attribute in predicate");
            let is_less_than = match chars.next().expect("no operator in predicate") {
                '<' => true,
                '>' => false,
                _ => panic!("unrecognized operator in predicate {pred_str}"),
            };
            let value = pred_str[2..]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("couldn't parse number in predicate {pred_str}"));
            predicate = Some(Predicate {
                attribute,
                value,
                is_less_than,
            });
        }
        let action = match action {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::ToWorkflow(action.to_owned()),
        };
        Self { predicate, action }
    }
}

impl Workflow {
    fn new(line: &str) -> Self {
        let parts = line.split('{').collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("unable to parse workflow {line}");
        }
        let id = parts[0].to_owned();
        assert_eq!(parts[1].chars().last(), Some('}'));
        let rules = &parts[1][..parts[1].len() - 1];
        let rules = rules.split(',').map(Rule::new).collect::<Vec<_>>();
        Self { id, rules }
    }
}

fn process(part: &Part, workflows: &HashMap<String, Workflow>) -> i32 {
    let mut workflow = workflows.get("in").expect("starting workflow not found");
    'l1: loop {
        for rule in workflow.rules.iter() {
            let mut p_result = true;
            if let Some(p_fn) = &rule.predicate {
                let tesval = part.0[&p_fn.attribute];
                p_result =
                    if p_fn.is_less_than { tesval < p_fn.value } else { tesval > p_fn.value };
            }
            if p_result {
                match &rule.action {
                    Action::ToWorkflow(id) => {
                        workflow = workflows
                            .get(id)
                            .unwrap_or_else(|| panic!("workflow {id} not found - {rule:?}"));
                        continue 'l1;
                    }
                    Action::Accept => return part.0.values().sum(),
                    Action::Reject => return 0,
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test19 {
    use super::*;

    #[test]
    fn GIVEN_valid_rule_string_WHEN_parsing_THEN_correct_rule_returned() {
        let dotest = |s, expected| {
            let rule = Rule::new(s);
            assert_eq!(expected, rule);
        };
        dotest(
            "ztf",
            Rule {
                predicate: None,
                action: Action::ToWorkflow("ztf".to_owned()),
            },
        );
        dotest(
            "A",
            Rule {
                predicate: None,
                action: Action::Accept,
            },
        );
        dotest(
            "a<2006:qkq",
            Rule {
                predicate: Some(Predicate {
                    attribute: 'a',
                    is_less_than: true,
                    value: 2006,
                }),
                action: Action::ToWorkflow(String::from("qkq")),
            },
        );
        dotest(
            "a>2006:R",
            Rule {
                predicate: Some(Predicate {
                    attribute: 'a',
                    is_less_than: false,
                    value: 2006,
                }),
                action: Action::Reject,
            },
        );
    }

    #[test]
    fn GIVEN_valid_workflow_line_WHEN_parsing_THEN_correct_workflow_returned() {
        let dotest = |s, expected| {
            let wf = Workflow::new(s);
            assert_eq!(expected, wf);
        };
        dotest(
            "pv{A,R}",
            Workflow {
                id: "pv".to_owned(),
                rules: vec![
                    Rule {
                        predicate: None,
                        action: Action::Accept,
                    },
                    Rule {
                        predicate: None,
                        action: Action::Reject,
                    },
                ],
            },
        );
    }

    #[test]
    fn GIVEN_valid_part_line_WHEN_parsing_THEN_correct_part_returned() {
        let dotest = |s, expected| {
            let part = Part::new(s);
            assert_eq!(Part(expected), part);
        };
        dotest(
            "{x=1679,m=44,a=2067,s=496}",
            HashMap::from([('x', 1679), ('m', 44), ('a', 2067), ('s', 496)]),
        );
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(19114, part1(&EXAMPLE[1..]));
    }

    static EXAMPLE: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
}
