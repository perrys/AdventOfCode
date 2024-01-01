//!
//! Advent of code challenge 2023 day 25.
//!
//! See <https://adventofcode.com/2023/day/25>
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

    println!("part1 total is {}", part1(contents.as_str()));
}

fn part1(_contents: &str) -> usize {
    let (connections, component_map) = parse_file(_contents);
    // look for two components which are no longer connected after removing
    // three separate routes, as these must pass through the 3 connection
    // points:
    for i in 0..component_map.len() {
        for j in i + 1..component_map.len() {
            if let Some(trimmed_connections) =
                partition_in_three_passes(&connections, i as u32, j as u32)
            {
                let (_, first_size) = breadth_first_search(&trimmed_connections, i as u32, None);
                let (_, second_size) = breadth_first_search(&trimmed_connections, j as u32, None);
                return first_size * second_size;
            }
        }
    }
    panic!("couldn't find separate groups");
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Connection {
    lhs: u32,
    rhs: u32,
}

impl Connection {
    fn other(&self, comp_idx: u32) -> u32 {
        match comp_idx {
            _ if comp_idx == self.lhs => self.rhs,
            _ if comp_idx == self.rhs => self.lhs,
            _ => panic!("no match"),
        }
    }
}

type ConnectionMap = HashMap<[char; 3], u32>;

/**
 * Parse the input and return a set of connections. Each component is given an
 * index, and a connection is a pair of component indices. This pair fits into
 * a 64-bit machine word so is good for cache locality. A map is also returned
 * for later decoding of component indices back to their original names.
*/
fn parse_file(contents: &str) -> (Vec<Connection>, ConnectionMap) {
    let mut map = ConnectionMap::new();
    let mut counter: u32 = 0;
    let mut get_component_index = |key: &str| {
        let key = three_letter_key(key);
        match map.get(&key) {
            Some(i) => *i,
            None => {
                map.insert(key, counter);
                counter += 1;
                counter - 1
            }
        }
    };
    let mut connections = Vec::new();
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let mut tok_iter = line.split(&[':', ' ']).filter(|s| !s.trim().is_empty());
            let lhs = get_component_index(tok_iter.next().expect("empty line"));
            for tok in tok_iter {
                connections.push(Connection {
                    lhs,
                    rhs: get_component_index(tok),
                });
            }
        });
    (connections, map)
}

fn three_letter_key(key: &str) -> [char; 3] {
    key.chars()
        .collect::<Vec<_>>()
        .try_into()
        .expect("non-3-letter component")
}

#[derive(Copy, Clone)]
struct PathElement {
    comp_idx: u32,
    parent: Option<Connection>,
}

/**
 * Perform a BFS on the connection map, starting at the given component. If an
 * end is supplied and there is a route to the end, the set of connections for
 * this route are returned. Also returns the number of components explored.
 */
fn breadth_first_search(
    connections: &[Connection],
    start: u32,
    end: Option<u32>,
) -> (Option<Vec<Connection>>, usize) {
    let mark_next = |comp_idx,
                     parent,
                     queue: &mut VecDeque<PathElement>,
                     explored_list: &mut Vec<PathElement>| {
        let elt = PathElement { comp_idx, parent };
        queue.push_back(elt);
        explored_list.push(elt);
    };
    let get_explored = |comp_idx, explored_list: &Vec<PathElement>| {
        explored_list
            .iter()
            .rfind(|&elt| elt.comp_idx == comp_idx)
            .copied()
    };
    let get_connections = |comp_idx| {
        connections
            .iter()
            .filter(move |&con| con.lhs == comp_idx || con.rhs == comp_idx)
    };

    let mut queue = VecDeque::<PathElement>::new();
    let mut explored_list = Vec::<PathElement>::new();
    mark_next(start, None, &mut queue, &mut explored_list);

    let last_elt = 'l1: loop {
        if queue.is_empty() {
            break None;
        } else {
            let elt @ PathElement {
                comp_idx,
                parent: _,
            } = queue.pop_front().unwrap();
            if let Some(end) = end {
                if comp_idx == end {
                    break 'l1 Some(elt);
                }
            }
            for connection in get_connections(comp_idx) {
                let next_comp_idx = connection.other(comp_idx);
                if get_explored(next_comp_idx, &explored_list).is_none() {
                    mark_next(
                        next_comp_idx,
                        Some(*connection),
                        &mut queue,
                        &mut explored_list,
                    );
                }
            }
        }
    };

    let num_explored = explored_list.len();
    if let Some(last) = last_elt {
        let mut result = Vec::new();
        let mut elt = last;
        let mut comp_idx = elt.comp_idx;
        while let Some(conn) = elt.parent {
            result.push(conn);
            let next_comp_idx = conn.other(comp_idx);
            elt =
                get_explored(next_comp_idx, &explored_list).expect("ref to unexplored connection");
            comp_idx = next_comp_idx;
        }
        result.reverse();
        (Some(result), num_explored)
    } else {
        (None, num_explored)
    }
}

fn partition_in_three_passes(
    connections: &[Connection],
    start: u32,
    end: u32,
) -> Option<Vec<Connection>> {
    let mut working_set = connections.to_vec();
    for count in 1..=3 {
        match breadth_first_search(&working_set, start, Some(end)).0 {
            Some(route) => working_set.retain(|c| !route.contains(c)),
            None => panic!("Exahusted routes between {start} and {end} on try {count}"),
        }
    }
    // now try a fourth time. This will fail if start and end are on different sides
    match breadth_first_search(&working_set, start, Some(end)).0 {
        Some(_) => None,
        None => Some(working_set),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test25 {
    use super::*;

    #[test]
    fn GIVEN_small_connection_list_WHEN_running_bfs_THEN_shortest_route_found() {
        let connections = r#"
aaa: eee bbb
eee: fff
fff: zzz
bbb: zzz
"#;
        let (connections, map) = parse_file(connections);
        let a = three_letter_key("aaa");
        let b = three_letter_key("bbb");
        let z = three_letter_key("zzz");
        let result = breadth_first_search(&connections, map[&a], Some(map[&z]));
        match result.0 {
            Some(route) => {
                assert_eq!(2, route.len());
                assert_eq!(
                    Connection {
                        lhs: map[&a],
                        rhs: map[&b]
                    },
                    route[0]
                );
                assert_eq!(
                    Connection {
                        lhs: map[&b],
                        rhs: map[&z]
                    },
                    route[1]
                );
            }
            None => panic!("no route found"),
        }
    }

    #[test]
    fn GIVEN_sample_connections_WHEN_testing_four_passes_THEN_only_succeeds_for_same_side_components(
    ) {
        let (connections, map) = parse_file(EXAMPLE);
        // same side of partition should fail:
        assert!(partition_in_three_passes(
            &connections,
            map[&three_letter_key("cmg")],
            map[&three_letter_key("frs")]
        )
        .is_none());
        assert!(partition_in_three_passes(
            &connections,
            map[&three_letter_key("lsr")],
            map[&three_letter_key("qnr")]
        )
        .is_none());
        // different sides of partition should pass:
        assert!(partition_in_three_passes(
            &connections,
            map[&three_letter_key("cmg")],
            map[&three_letter_key("bvb")]
        )
        .is_some());
        assert!(partition_in_three_passes(
            &connections,
            map[&three_letter_key("nvd")],
            map[&three_letter_key("xhk")]
        )
        .is_some());
    }

    #[test]
    fn GIVEN_sample_connections_WHEN_counting_group_size_THEN_known_size_returned() {
        let (connections, map) = parse_file(EXAMPLE);
        let (_, group_size) =
            breadth_first_search(&connections, *map.values().next().expect("empty_map"), None);
        assert_eq!(map.len(), group_size);
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(54, part1(EXAMPLE));
    }

    static EXAMPLE: &str = r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
}
