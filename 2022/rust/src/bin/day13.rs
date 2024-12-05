use std::{cell::RefCell, cmp, fs, rc::Rc};

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
    0
}

fn part2(_contents: &str) -> usize {
    0
}

fn make_shared<T>(item: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(item))
}

type SharedItem = Rc<RefCell<Item>>;

enum Item {
    Branch(Vec<SharedItem>),
    Leaf(u8),
}

impl Item {
    fn new_branch() -> Self {
        Self::Branch(vec![])
    }
    fn new_leaf(num: u8) -> Self {
        Self::Leaf(num)
    }
}

impl Item {
    fn parse(line: &str) -> SharedItem {
        let add_child = |stack: &mut Vec<SharedItem>, item: SharedItem| {
            if !stack.is_empty() {
                let parent = stack.last().unwrap().clone();
                let mut kind = parent.borrow_mut();
                match *kind {
                    Item::Leaf(_) => panic!("logic error - non-branch parent"),
                    Item::Branch(ref mut children) => {
                        children.push(item);
                    }
                }
            }
        };

        let mut stack = Vec::<SharedItem>::new();
        let mut accumulator: Option<u8> = None;
        for c in line.chars() {
            let flush = |stack, accumulator: &mut Option<u8>| {
                if let Some(num) = accumulator {
                    add_child(stack, make_shared(Item::new_leaf(*num)));
                    *accumulator = None;
                };
            };
            match c {
                '[' => {
                    flush(&mut stack, &mut accumulator);
                    let new_branch = make_shared(Item::new_branch());
                    add_child(&mut stack, new_branch.clone());
                    stack.push(new_branch);
                }
                ']' => {
                    flush(&mut stack, &mut accumulator);
                    if stack.len() == 1 {
                        return stack.pop().unwrap();
                    }
                    stack.pop();
                }
                ',' => {
                    flush(&mut stack, &mut accumulator);
                }
                '0'..='9' => {
                    match accumulator {
                        None => accumulator = Some(c as u8 - b'0'),
                        Some(val) => {
                            let newval = 10 * val + (c as u8 - b'0');
                            accumulator = Some(newval);
                        }
                    };
                }
                _ => panic!("unrecognized syntax: {}", c),
            }
        }
        panic!("empty or unbalanced AST");
    }

    fn to_str(&self) -> String {
        let mut buffer = Vec::<char>::new();
        fn walk(item: &Item, buffer: &mut Vec<char>) {
            match item {
                Item::Branch(children) => {
                    buffer.push('[');
                    for (idx, child) in children.iter().enumerate() {
                        if idx > 0 {
                            buffer.push(',');
                        }
                        walk(&child.borrow(), buffer);
                    }
                    buffer.push(']');
                }
                Item::Leaf(val) => {
                    let outstr = format!("{val}");
                    buffer.extend(outstr.chars());
                }
            }
        }
        walk(self, &mut buffer);
        String::from_iter(buffer.into_iter())
    }
}

fn compare(lhs: &SharedItem, rhs: &SharedItem) -> cmp::Ordering {
    match *lhs.borrow() {
        Item::Branch(ref lhs_children) => match *rhs.borrow() {
            Item::Branch(ref rhs_children) => {
                for i in 0..lhs_children.len().min(rhs_children.len()) {
                    let cmp_result = compare(&lhs_children[i], &rhs_children[i]);
                    match cmp_result {
                        cmp::Ordering::Equal => (),
                        _ => return cmp_result,
                    };
                }
                return lhs_children.len().cmp(&rhs_children.len());
            }
            Item::Leaf(_) => todo!(),
        },
        Item::Leaf(_) => todo!(),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn GIVEN_simple_lines_WHEN_parsing_THEN_correct_AST_produced() {
        fn do_test(s: &str) {
            let ast = Item::parse(s);
            assert_eq!(s, ast.borrow().to_str());
        }
        do_test("[]");
        do_test("[27]");
        do_test("[27,17]");
        do_test("[[27],17]");
        do_test("[3,[27,31],17]");
    }
}
