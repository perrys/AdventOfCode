//!
//! Advent of code challenge 2023 day 22.
//!
//! See <https://adventofcode.com/2023/day/22>
//!
use std::{
    cell::Cell,
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::RangeInclusive,
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
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    let mut bricks = parse_file(contents);
    let zmap = drop_bricks_and_return_zmap(&mut bricks);
    bricks
        .iter()
        .filter(|&brick| {
            let mut okay = true;
            for idx in get_supported_bricks(brick, &zmap).iter() {
                if get_supporting_bricks(&bricks[*idx], &zmap).len() == 1 {
                    okay = false;
                    break;
                }
            }
            okay
        })
        .count()
}

fn part2(contents: &str) -> usize {
    let mut bricks = parse_file(contents);
    let zmap = drop_bricks_and_return_zmap(&mut bricks);
    let supports: HashMap<BrickIndex, Vec<BrickIndex>> =
        HashMap::from_iter(bricks.iter().enumerate().map(|(idx, b)| {
            let sup = get_supported_bricks(b, &zmap);
            (idx, sup)
        }));
    let supported_by: HashMap<BrickIndex, Vec<BrickIndex>> =
        HashMap::from_iter(bricks.iter().enumerate().map(|(idx, b)| {
            let sup = get_supporting_bricks(b, &zmap);
            (idx, sup)
        }));
    (0..bricks.len())
        .map(|idx| {
            //let uset = recursive_chain_reaction(vec![idx], &supports, &supported_by);
            let uset = breadth_first_search(idx, &supports, &supported_by);
            uset.len() - 1
        })
        .sum()
}

fn breadth_first_search(
    id: BrickIndex,
    supports: &HashMap<BrickIndex, Vec<BrickIndex>>,
    supported_by: &HashMap<BrickIndex, Vec<BrickIndex>>,
) -> HashSet<BrickIndex> {
    let mut queue = VecDeque::<BrickIndex>::new();
    queue.push_back(id);
    let mut removed = HashSet::<BrickIndex>::new();
    while !queue.is_empty() {
        let brick_id = queue.pop_front().unwrap();
        removed.insert(brick_id);
        let supported = &supports[&brick_id];
        for id in supported.iter() {
            if supported_by[id].iter().all(|b| removed.contains(b)) {
                queue.push_back(*id);
            }
        }
    }
    removed
}

fn parse_file(contents: &str) -> Vec<Brick> {
    let mut result = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Brick::new)
        .collect::<Vec<_>>();
    result.sort_by_key(|b| b.z_hi.get());
    result
}

type XYCoord = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z_len: usize,
    z_hi: Cell<usize>,
}

impl Brick {
    fn new(line: &str) -> Self {
        let mut parts = line.split('~');
        let first = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let second = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't parse {line}"));
        let parser = |part: &str| -> [usize; 3] {
            part.split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|_| panic!("parse error for {line}"))
                .try_into()
                .unwrap_or_else(|_| panic!("unexpected number of coords for {line}"))
        };
        let first = parser(first);
        let second = parser(second);
        if first[2] != second[2] && (first[1] != second[1] || first[0] != second[0]) {
            panic!("Brick is not perfectly horizontal or vertical for {line}");
        }
        let z = first[2].min(second[2])..=second[2].max(second[2]);
        Self {
            x: first[0].min(second[0])..=second[0].max(second[0]),
            y: first[1].min(second[1])..=second[1].max(second[1]),
            z_len: z.end() - z.start() + 1,
            z_hi: Cell::new(*z.end()),
        }
    }

    fn iter_xy(&self) -> impl Iterator<Item = XYCoord> {
        XYIter::new(self.x.clone(), self.y.clone())
    }
}

struct XYIter<T>
where
    T: IntoIterator<Item = usize>,
{
    x_iter: T::IntoIter,
    y_iter: T::IntoIter,
    last_x: Option<usize>,
    y_range: T,
}

impl<T> XYIter<T>
where
    T: IntoIterator<Item = usize> + Clone,
{
    fn new(xrange: T, y_range: T) -> Self {
        let mut x_iter = xrange.into_iter();
        let last_x = x_iter.next();
        Self {
            x_iter,
            last_x,
            y_iter: y_range.clone().into_iter(),
            y_range,
        }
    }
}

impl<T> Iterator for XYIter<T>
where
    T: IntoIterator<Item = usize> + Clone,
{
    type Item = XYCoord;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.last_x {
            if let Some(y) = self.y_iter.next() {
                return Some((x, y));
            } else {
                self.last_x = self.x_iter.next();
                self.y_iter = self.y_range.clone().into_iter();
            }
        }
        None
    }
}

type BrickIndex = usize;
type ZStack = Vec<(RangeInclusive<usize>, BrickIndex)>;

fn drop_bricks_and_return_zmap(bricks: &mut [Brick]) -> HashMap<XYCoord, ZStack> {
    let mut map: HashMap<XYCoord, ZStack> = HashMap::new();
    for (idx, brick) in bricks.iter().enumerate() {
        let mut max_z = 0usize;
        for (x, y) in brick.iter_xy() {
            if let Some(stack) = &map.get(&(x, y)) {
                let z = stack.last().unwrap().0.end();
                max_z = max_z.max(*z);
            }
        }
        brick.z_hi.set(max_z + brick.z_len);
        for (x, y) in brick.iter_xy() {
            let stack = map.entry((x, y)).or_default();
            let lo = 1 + brick.z_hi.get() - brick.z_len;
            stack.push((lo..=brick.z_hi.get(), idx));
        }
    }
    map
}

fn get_supported_bricks(brick: &Brick, zmap: &HashMap<XYCoord, ZStack>) -> Vec<BrickIndex> {
    let mut ids = Vec::new();
    for xy in brick.iter_xy() {
        if let Some(stack) = zmap.get(&xy) {
            if let Some((_z, idx)) = stack
                .iter()
                .find(|(z, _idx)| *z.start() == brick.z_hi.get() + 1)
            {
                ids.push(*idx);
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

fn get_supporting_bricks(brick: &Brick, zmap: &HashMap<XYCoord, ZStack>) -> Vec<BrickIndex> {
    let mut ids = Vec::new();
    for xy in brick.iter_xy() {
        if let Some(stack) = zmap.get(&xy) {
            if let Some((_z, idx)) = stack
                .iter()
                .find(|(z, _idx)| *z.end() == brick.z_hi.get() - brick.z_len)
            {
                ids.push(*idx);
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test22 {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn GIVEN_small_brick_set_WHEN_dropping_bricks_THEN_correct_zmap_produced() {
        let dotest = |lines, expected: Vec<(XYCoord, ZStack)>| {
            let mut bricks = parse_file(lines);
            let zmap = drop_bricks_and_return_zmap(&mut bricks);
            let exmap = HashMap::from_iter(expected);
            assert_eq!(exmap, zmap);
        };
        // single stack
        dotest("1,1,1~1,1,2", vec![((1, 1), vec![(1..=2, 0)])]);
        // stacked bricks, requires z-sorting
        dotest(
            "1,1,6~2,1,6\n1,1,1~1,1,2",
            vec![
                ((1, 1), vec![(1..=2, 0), (3..=3, 1)]),
                ((2, 1), vec![(3..=3, 1)]),
            ],
        );
    }

    #[test]
    fn GIVEN_XYIter_WHEN_iterating_THEN_x_and_y_iterated() {
        let xrange = 0..2;
        let yrange = 4..6;
        let mut xy = XYIter::new(xrange, yrange);
        assert_eq!(Some((0, 4)), xy.next());
        assert_eq!(Some((0, 5)), xy.next());
        assert_eq!(Some((1, 4)), xy.next());
        assert_eq!(Some((1, 5)), xy.next());
        assert_eq!(None, xy.next());
    }

    #[test]
    fn GIVEN_small_brick_set_WHEN_getting_supported_bricks_THEN_correct_bricks_returned() {
        let mut bricks = parse_file(EXAMPLE);
        let zmap = drop_bricks_and_return_zmap(&mut bricks);
        let dotest = |brick, expected: &[usize]| {
            let result: HashSet<usize> = HashSet::from_iter(get_supported_bricks(brick, &zmap));
            let expected = HashSet::from_iter(expected.iter().copied());
            assert_eq!(expected, result);
        };
        dotest(&bricks[0], &[1, 2]); // example A supports B and C
        dotest(&bricks[1], &[3, 4]); // example B supports D and E
        dotest(&bricks[6], &[]); // example G supports nothing
    }

    #[test]
    fn GIVEN_small_brick_set_WHEN_getting_supporting_bricks_THEN_correct_bricks_returned() {
        let mut bricks = parse_file(EXAMPLE);
        let zmap = drop_bricks_and_return_zmap(&mut bricks);
        let dotest = |brick, expected: &[usize]| {
            let result: HashSet<usize> = HashSet::from_iter(get_supporting_bricks(brick, &zmap));
            let expected = HashSet::from_iter(expected.iter().copied());
            assert_eq!(expected, result);
        };
        dotest(&bricks[1], &[0]); // example B supported by A
        dotest(&bricks[3], &[1, 2]); // example D supported by B and C
        dotest(&bricks[6], &[5]); // example G supported by F
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        assert_eq!(5, part1(EXAMPLE));
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        assert_eq!(7, part2(EXAMPLE));
    }

    static EXAMPLE: &str = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
}
