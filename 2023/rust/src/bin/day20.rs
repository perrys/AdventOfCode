//!
//! Advent of code challenge 2023 day 20.
//!
//! See <https://adventofcode.com/2023/day/20>
//!
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!(
        "part1 total is {:?}",
        part1(contents.as_str()).iter().product::<usize>()
    );
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> [usize; 2] {
    let modules = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Module::new)
        .collect::<Vec<_>>();
    let mut modules = HashMap::from_iter(modules.into_iter().map(|m| match m {
        Module::FlipFlop {
            ref name,
            last_input: _,
            outputs: _,
        } => (name.clone(), m),
        Module::Conjunction {
            ref name,
            last_inputs: _,
            outputs: _,
        } => (name.clone(), m),
        Module::Broadcaster { outputs: _ } => ("broadcaster".to_owned(), m),
        Module::OutputSink => ("output".to_owned(), m),
    }));
    set_conjunction_states(&mut modules);
    let mut hi = 0;
    let mut lo = 0;
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            high: false,
            sender: String::new(),
            destination: "broadcaster".to_owned(),
        });
        let (high, low) = process_queue(&mut modules, queue);
        hi += high;
        lo += low;
    }
    [hi, lo]
}

fn set_conjunction_states(modules: &mut HashMap<String, Module>) {
    let mut conjunction_inputs = Vec::new();
    let mut record_connection = |src_name: &String, outputs| {
        for output in outputs {
            let other = modules.get(output).unwrap_or(&Module::OutputSink);
            if let Module::Conjunction {
                name,
                last_inputs: _,
                outputs: _,
            } = other
            {
                conjunction_inputs.push((src_name.clone(), name.clone()));
            }
        }
    };
    for m in modules.values() {
        match m {
            Module::FlipFlop {
                name,
                last_input: _,
                outputs,
            } => {
                record_connection(name, outputs);
            }
            Module::Conjunction {
                name,
                last_inputs: _,
                outputs,
            } => {
                record_connection(name, outputs);
            }
            Module::Broadcaster { outputs } => {
                record_connection(&"broadcaster".to_owned(), outputs);
            }
            Module::OutputSink => (),
        }
    }
    for (src, dest) in conjunction_inputs {
        let module = modules.get_mut(&dest).unwrap();
        if let Module::Conjunction {
            name: _,
            last_inputs,
            outputs: _,
        } = module
        {
            last_inputs.insert(src, false);
        } else {
            unreachable!()
        }
    }
}

fn part2(_contents: &str) -> i64 {
    0
}

#[derive(Debug, Eq, PartialEq)]
enum Module {
    FlipFlop {
        name: String,
        last_input: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        last_inputs: HashMap<String, bool>,
        outputs: Vec<String>,
    },
    Broadcaster {
        outputs: Vec<String>,
    },
    OutputSink,
}

#[derive(Debug, Eq, PartialEq)]
struct Pulse {
    high: bool,
    sender: String,
    destination: String,
}

impl Module {
    fn new(line: &str) -> Self {
        let parts = line.split(' ').collect::<Vec<_>>();
        let module = parts[0];
        let outputs = parts[2..]
            .iter()
            .map(|&s| s.to_owned().replace(',', ""))
            .collect::<Vec<_>>();
        match module {
            _ if module.starts_with('%') => {
                let name = module[1..].to_owned();
                Self::FlipFlop {
                    name,
                    last_input: false,
                    outputs,
                }
            }
            _ if module.starts_with('&') => {
                let name = module[1..].to_owned();
                Self::Conjunction {
                    name,
                    last_inputs: HashMap::new(),
                    outputs,
                }
            }
            "broadcaster" => Self::Broadcaster { outputs },
            _ => panic!("unknown module in {line}"),
        }
    }

    fn pulse(&mut self, pulse: Pulse, queue: &mut VecDeque<Pulse>) {
        match self {
            Module::FlipFlop {
                name,
                last_input,
                outputs,
            } => {
                if !pulse.high {
                    *last_input = !*last_input;
                    for out in outputs.iter() {
                        queue.push_back(Pulse {
                            sender: name.clone(),
                            high: *last_input,
                            destination: out.clone(),
                        });
                    }
                }
            }
            Module::Conjunction {
                name,
                last_inputs,
                outputs,
            } => {
                last_inputs.insert(pulse.sender, pulse.high);
                let all_high = last_inputs.values().all(|f| *f);
                for out in outputs.iter() {
                    queue.push_back(Pulse {
                        sender: name.clone(),
                        high: !all_high,
                        destination: out.clone(),
                    });
                }
            }
            Module::Broadcaster { outputs } => {
                for out in outputs.iter() {
                    queue.push_back(Pulse {
                        sender: "broadcaster".to_owned(),
                        high: pulse.high,
                        destination: out.clone(),
                    });
                }
            }
            Module::OutputSink => (),
        }
    }
}

fn process_queue(
    modules: &mut HashMap<String, Module>,
    mut queue: VecDeque<Pulse>,
) -> (usize, usize) {
    let mut highs = 0usize;
    let mut lows = 0usize;
    while !queue.is_empty() {
        let pulse = queue.pop_front().unwrap();
        if pulse.high {
            highs += 1
        } else {
            lows += 1
        }
        if let Some(module) = modules.get_mut(&pulse.destination) {
            module.pulse(pulse, &mut queue);
        }
    }
    (highs, lows)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test20 {
    use super::*;

    #[test]
    fn GIVEN_flipflop_module_WHEN_pulsing_THEN_correct_pulses_output() {
        let mut ff = Module::new("%f1 -> f2, f3");
        let mut queue = VecDeque::<Pulse>::new();
        match ff {
            Module::FlipFlop {
                name: _,
                last_input: _,
                outputs: _,
            } => {
                // send high pulse
                ff.pulse(
                    Pulse {
                        high: true,
                        sender: "foobar".to_owned(),
                        destination: "f1".to_owned(),
                    },
                    &mut queue,
                );
                assert_eq!(0, queue.len());
                // send low pulse
                ff.pulse(
                    Pulse {
                        high: false,
                        sender: "foobar".to_owned(),
                        destination: "f1".to_owned(),
                    },
                    &mut queue,
                );
                assert_eq!(2, queue.len());
                assert_eq!(
                    Pulse {
                        high: true,
                        sender: "f1".to_owned(),
                        destination: "f2".to_owned()
                    },
                    queue.pop_front().unwrap()
                );
                assert_eq!(
                    Pulse {
                        high: true,
                        sender: "f1".to_owned(),
                        destination: "f3".to_owned()
                    },
                    queue.pop_front().unwrap()
                );
            }
            _ => panic!("flipflow was not correctly parsed"),
        };
    }

    #[test]
    fn GIVEN_conjunction_module_WHEN_pulsing_THEN_correct_pulses_output() {
        let mut con = Module::new("&inv -> f2, f3");
        let mut queue = VecDeque::<Pulse>::new();
        match con {
            Module::Conjunction {
                name: _,
                last_inputs: _,
                outputs: _,
            } => {
                // send high pulse
                con.pulse(
                    Pulse {
                        high: true,
                        sender: "foobar".to_owned(),
                        destination: "inv".to_owned(),
                    },
                    &mut queue,
                );
                assert_eq!(2, queue.len());
                assert_eq!(
                    Pulse {
                        high: false,
                        sender: "inv".to_owned(),
                        destination: "f2".to_owned()
                    },
                    queue.pop_front().unwrap()
                );
                assert_eq!(
                    Pulse {
                        high: false,
                        sender: "inv".to_owned(),
                        destination: "f3".to_owned()
                    },
                    queue.pop_front().unwrap()
                );
            }
            _ => panic!("conjunction was not correctly parsed"),
        };
    }

    static EXAMPLE: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!([2750, 4250], part1(EXAMPLE));
    }
}
