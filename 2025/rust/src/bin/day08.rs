use std::collections::HashSet;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    let points = Point::parse_points(&contents);
    println!("part1: {}", part1(&points));
    println!("part2: {}", part2(&points));
}

fn part1(points: &[Point]) -> usize {
    let pairs = PointPair::get_pairs(points);
    let groups = process_groupings(&pairs[0..1000], |_, _| true);
    let mut sizes: Vec<usize> = groups.into_iter().map(|group| group.len()).collect();
    sizes.sort();
    // println!("{sizes:?}");
    sizes.into_iter().rev().take(3).product()
}

fn part2(points: &[Point]) -> i64 {
    let distances = PointPair::get_pairs(points);
    let mut answer = 0;
    let callback = |single_group_len: usize, last_pair: &(Point, Point)| {
        if single_group_len == points.len() {
            answer = last_pair.0.x * last_pair.1.x;
            return false;
        }
        true
    };
    process_groupings(&distances, callback);
    answer
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(line: &str) -> Self {
        let mut iter = line.split(',');
        let x = iter
            .next()
            .expect("x missing")
            .parse::<i64>()
            .expect("x non-int");
        let y = iter
            .next()
            .expect("y missing")
            .parse::<i64>()
            .expect("y non-int");
        let z = iter
            .next()
            .expect("z missing")
            .parse::<i64>()
            .expect("z non-int");
        Self { x, y, z }
    }
    fn parse_points(contents: &str) -> Vec<Self> {
        contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Point::new)
            .collect::<Vec<_>>()
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct PointPair {
    distance: f64,
    points: (Point, Point),
}

impl PointPair {
    fn new(p1: &Point, p2: &Point) -> Self {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        let dz = p1.z - p2.z;
        let distance = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
        Self {
            points: (*p1, *p2),
            distance,
        }
    }

    fn get_pairs(points: &[Point]) -> Vec<Self> {
        let n = points.len();
        let mut distances = Vec::with_capacity(n / 2 * (n + 1));
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                distances.push(PointPair::new(&points[i], &points[j]));
            }
        }
        distances.sort_by(|lhs, rhs| {
            lhs.distance
                .partial_cmp(&rhs.distance)
                .expect("unexpected NaN!")
        });
        distances
    }
}

fn process_groupings<F>(pairs: &[PointPair], mut callback: F) -> Vec<HashSet<Point>>
where
    F: FnMut(usize, &(Point, Point)) -> bool,
{
    let mut groups = Vec::<HashSet<Point>>::new();
    for PointPair {
        distance: _,
        points,
    } in pairs.iter()
    {
        let mut group_idxs = (None, None);
        for (i, group) in groups.iter().enumerate() {
            if group.contains(&points.0) {
                group_idxs.0 = Some(i);
            }
            if group.contains(&points.1) {
                group_idxs.1 = Some(i);
            }
        }
        if let Some(i) = group_idxs.0
            && let Some(j) = group_idxs.1
        {
            if i != j {
                let last = i.max(j);
                let first = i.min(j);
                let tmp = groups.remove(last);
                groups[first].extend(tmp);
            }
        } else if let Some(i) = group_idxs.0 {
            groups[i].insert(points.1);
        } else if let Some(i) = group_idxs.1 {
            groups[i].insert(points.0);
        } else {
            groups.push(HashSet::from([points.0, points.1]));
        }
        let single_group_size = match groups.len() {
            1 => groups[0].len(),
            _ => 0,
        };
        if !callback(single_group_size, points) {
            break;
        }
    }
    // println!("groups: {groups:?}");
    groups
}
